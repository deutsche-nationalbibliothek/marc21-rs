use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::path::PathBuf;

use marc21::matcher::RecordMatcher;
use serde::Deserialize;
use sophia::api::graph::MutableGraph;
use sophia::api::ns::{Namespace, xsd};
use sophia::api::serializer::TripleSerializer;
use sophia::api::term::{FromTerm, IriRef};
use sophia::inmem::graph::LightGraph;
use sophia::iri::InvalidIri;
use sophia::term::RcTerm;
use sophia::turtle::serializer::turtle::{
    TurtleConfig, TurtleSerializer,
};

use crate::prelude::*;
use crate::utils::Writer;

#[derive(Debug, clap::Parser)]
pub(crate) struct Skosify {
    #[arg(long, short, required = true)]
    config: PathBuf,

    /// MARC21 files to be processed as input. If no file is specified,
    /// or if the filename is `-`, the data is read from standard input
    /// (`stdin`) by default.
    #[arg(default_value = "-", hide_default_value = true)]
    input: Vec<PathBuf>,

    /// Write output to <filename> instead of stdout.
    #[arg(short, long, value_name = "filename")]
    output: Option<PathBuf>,

    #[command(flatten, next_help_heading = "Filter options")]
    pub(crate) filter_opts: FilterOpts,

    #[command(flatten, next_help_heading = "Common options")]
    pub(crate) common: CommonOpts,
}

impl Skosify {
    pub(crate) fn execute(self) -> CliResult {
        let mut progress = Progress::new(self.common.progress);
        let options = MatchOptions::from(&self.filter_opts);
        let filter = self.filter_opts.filter()?;
        let mut count = 0;
        let mut line = 0;

        let mut output = WriterBuilder::default()
            .with_compression(self.common.compression)
            .try_from_path_or_stdout(self.output)?;

        let mut graph = SkosGraph::from_path(&self.config)?;

        // let base_uri = config.concept_uri.base_uri;
        // let skos = Namespace::new_unchecked(SKOS_NS);
        // let rdf = Namespace::new_unchecked(RDF_NS);

        'outer: for path in self.input.iter() {
            let mut reader = MarcReadOptions::default()
                .try_into_reader_from_path(path)?;

            while let Some(result) = reader.next_byte_record() {
                line += 1;

                match result {
                    Err(ReadMarcError::Parse(_))
                        if self.filter_opts.skip_invalid =>
                    {
                        progress.update(true);
                        continue;
                    }
                    Err(e) => {
                        return Err(CliError::from_parse(e, line));
                    }
                    Ok(record) => {
                        progress.update(false);

                        if let Some(ref m) = filter
                            && !m.is_match(&record, &options)
                        {
                            continue;
                        }

                        graph.process_record(record)?;

                        count += 1;
                        if self.filter_opts.limit == count {
                            break 'outer;
                        }
                    }
                }
            }
        }

        graph.serialize_graph(&mut output)?;
        progress.finish();
        output.finish()?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SkosGraph {
    scope: Option<RecordMatcher>,
    uri: ConceptUri,

    /// Whether to make extra effort to produce pretty output or not.
    #[serde(default)]
    pretty: bool,

    #[serde(default, rename = "group")]
    groups: BTreeMap<String, Concept>,

    #[serde(skip, default = "default_rdf_ns")]
    rdf_ns: Namespace<&'static str>,

    #[serde(skip, default = "default_skos_ns")]
    skos_ns: Namespace<&'static str>,

    #[serde(skip)]
    graph: LightGraph,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct Concept {
    scope: Option<RecordMatcher>,
    labels: Vec<Label>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
struct Label {
    kind: LabelKind,
    path: Path,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum LabelKind {
    Preferred,
    Alternative,
    Hidden,
}

fn default_rdf_ns() -> Namespace<&'static str> {
    Namespace::new_unchecked_const(
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
    )
}

fn default_skos_ns() -> Namespace<&'static str> {
    Namespace::new_unchecked_const(
        "http://www.w3.org/2004/02/skos/core#",
    )
}

impl SkosGraph {
    /// Creates a new SkosGraph from a config file.
    fn from_path<P>(path: P) -> Result<Self, CliError>
    where
        P: AsRef<std::path::Path>,
    {
        let content: String = read_to_string(path)?;
        let mut graph: Self = toml::de::from_str(&content)?;
        graph.graph = LightGraph::new();

        Ok(graph)
    }

    fn process_record(
        &mut self,
        record: ByteRecord,
    ) -> Result<(), CliError> {
        use LabelKind::*;
        let record = StringRecord::try_from(record)?;
        let options = MatchOptions::default();

        if let Some(ref matcher) = self.scope
            && !matcher.is_match(&record, &options)
        {
            return Ok(());
        }

        let s = self
            .uri
            .get(&record)
            .map_err(|e| CliError::AdHoc(e.to_string()))?;
        let p = self.rdf_ns.get("type").unwrap();
        let o = self.skos_ns.get("Concept").unwrap();

        match self.graph.insert(&s, p, o) {
            Err(e) => return Err(CliError::AdHoc(e.to_string())),
            Ok(false) => {
                // We expect that each record is a new concept. A value
                // of `false` means that the graph insertion doesn't
                // changed the underlying graph. This happens when the
                // triple is already present.
                return Err(CliError::AdHoc(format!(
                    "the skos graph already contains a concept with iri {:?}.",
                    s.to_string()
                )));
            }
            _ => (),
        }

        let mut stop = false;

        for group in self.groups.values() {
            if let Some(ref matcher) = group.scope
                && !matcher.is_match(&record, &options)
            {
                continue;
            }

            for label in group.labels.iter() {
                let p = match label.kind {
                    Preferred => self.skos_ns.get("prefLabel"),
                    Alternative => self.skos_ns.get("altLabel"),
                    Hidden => self.skos_ns.get("hiddenLabel"),
                }
                .unwrap();

                for value in record.path(&label.path, &options) {
                    let literal =
                        value.to_str_unchecked() * xsd::string;
                    let o = RcTerm::from_term(literal);

                    self.graph.insert(&s, p, o).unwrap();
                    stop = true;
                }
            }

            if stop {
                break;
            }
        }

        Ok(())
    }

    fn serialize_graph(
        self,
        writer: &mut Writer,
    ) -> Result<(), CliError> {
        let config = TurtleConfig::default().with_pretty(self.pretty);
        let mut ser = TurtleSerializer::new_with_config(writer, config);
        ser.serialize_graph(&self.graph)
            .map_err(|err| CliError::AdHoc(err.to_string()))?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
enum ConceptUri {
    Path {
        path: Path,
    },
    #[serde(rename_all = "kebab-case")]
    Base {
        base_uri: String,
        path: Path,
    },
}

impl ConceptUri {
    fn get(
        &self,
        record: &StringRecord,
    ) -> Result<IriRef<String>, InvalidIri> {
        let options = MatchOptions::default();

        match self {
            Self::Path { path } => {
                let iri = record
                    .first(path, &options)
                    .map(|value| value.to_str_unchecked().to_string())
                    .unwrap_or_default();

                IriRef::new(iri)
            }
            Self::Base { base_uri, path } => {
                let suffix = record
                    .first(path, &options)
                    .map(|value| value.to_str_unchecked().to_string())
                    .unwrap_or_default();

                IriRef::new(format!("{base_uri}{suffix}"))
            }
        }
    }
}
