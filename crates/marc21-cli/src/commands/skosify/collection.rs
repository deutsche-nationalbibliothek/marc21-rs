use std::collections::BTreeMap;

use marc21::StringRecord;
use marc21::matcher::{MatchOptions, RecordMatcher};
use serde::Deserialize;
use sophia::api::graph::MutableGraph;
use sophia::api::ns::Namespace;
use sophia::iri::IriRef;

use crate::commands::skosify::uri::Uri;
use crate::commands::skosify::utils::*;
use crate::error::CliError;

#[derive(Debug, Deserialize)]
pub(crate) struct Collections {
    #[serde(skip, default = "default_rdf_ns")]
    rdf_ns: Namespace<&'static str>,

    #[serde(skip, default = "default_skos_ns")]
    skos_ns: Namespace<&'static str>,

    scope: Option<RecordMatcher>,

    uri: Uri,

    #[serde(default)]
    min: usize,

    #[serde(default)]
    max: usize,

    #[serde(skip, default)]
    map: BTreeMap<IriRef<String>, Vec<IriRef<String>>>,
}

impl Collections {
    pub(crate) fn process_record(
        &mut self,
        record: &StringRecord,
        options: &MatchOptions,
        subject: &IriRef<String>,
    ) -> Result<(), CliError> {
        if let Some(ref matcher) = self.scope
            && matcher.is_match(record, options)
        {
            return Ok(());
        }

        for result in self.uri.all(record, options) {
            self.map
                .entry(result?)
                .and_modify(|entry| entry.push(subject.clone()))
                .or_insert(vec![subject.clone()]);
        }

        Ok(())
    }

    pub(crate) fn finish<MG: MutableGraph>(
        &self,
        graph: &mut MG,
    ) -> Result<(), CliError> {
        for (subject, members) in self.map.iter() {
            if self.min > 0 && (members.len() < self.min) {
                continue;
            }

            if self.max > 0 && (members.len() > self.max) {
                continue;
            }

            let p = self.rdf_ns.get("type")?;
            let o = self.skos_ns.get("Collection")?;

            graph.insert(subject, p, o).unwrap();

            for member in members {
                let p = self.rdf_ns.get("member")?;
                graph.insert(subject, p, member).unwrap();
            }
        }

        Ok(())
    }
}
