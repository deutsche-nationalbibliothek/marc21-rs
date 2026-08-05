use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::read_to_string;

use marc21::matcher::RecordMatcher;
use serde::Deserialize;
use sophia::api::graph::MutableGraph;
use sophia::api::ns::Namespace;
use sophia::api::serializer::TripleSerializer;
use sophia::inmem::graph::LightGraph;
use sophia::turtle::serializer::turtle::{
    TurtleConfig, TurtleSerializer,
};

use crate::commands::skosify::concept::Concept;
use crate::commands::skosify::uri::Uri;
use crate::commands::skosify::utils::{
    default_rdf_ns, default_skos_ns,
};
use crate::prelude::*;
use crate::unicode::NormalizationForm;
use crate::utils::Writer;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct SkosGraph {
    scope: Option<RecordMatcher>,
    uri: Uri,

    /// Whether to make extra effort to produce pretty output or not.
    #[serde(default)]
    pretty: bool,

    translit: Option<NormalizationForm>,

    #[serde(default, rename = "concept")]
    groups: BTreeMap<String, Concept>,

    #[serde(skip, default = "default_rdf_ns")]
    rdf_ns: Namespace<&'static str>,

    #[serde(skip, default = "default_skos_ns")]
    skos_ns: Namespace<&'static str>,

    #[serde(skip)]
    graph: LightGraph,
}

impl SkosGraph {
    /// Creates a new SkosGraph from a config file.
    pub(crate) fn from_path<P>(path: P) -> Result<Self, CliError>
    where
        P: AsRef<std::path::Path>,
    {
        let content: String = read_to_string(path)?;
        let mut graph: Self = toml::de::from_str(&content)?;
        graph.graph = LightGraph::new();

        Ok(graph)
    }

    pub(crate) fn process_record(
        &mut self,
        record: ByteRecord,
    ) -> Result<(), CliError> {
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

        for concept in self.groups.values() {
            for (p, o) in concept.labels(&record, &self.translit)? {
                self.graph.insert(&s, p, o).unwrap();
            }
        }

        Ok(())
    }

    pub(crate) fn serialize_graph(
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
