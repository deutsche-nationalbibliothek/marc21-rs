use marc21::matcher::{MatchOptions, RecordMatcher};
use marc21::{Path, StringRecord};
use serde::Deserialize;
use sophia::api::ns::{Namespace, NsTerm, xsd};
use sophia::api::term::FromTerm;
use sophia::term::RcTerm;

use crate::commands::skosify::utils::default_skos_ns;
use crate::error::CliError;
use crate::unicode::{NormalizationForm, translit};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
pub(crate) struct Concept {
    #[serde(skip, default = "default_skos_ns")]
    skos_ns: Namespace<&'static str>,

    scope: Option<RecordMatcher>,

    labels: Vec<Label>,
}

impl Concept {
    pub(crate) fn labels(
        &self,
        record: &StringRecord,
        options: &MatchOptions,
        nf: &Option<NormalizationForm>,
    ) -> Result<Vec<(NsTerm<'_>, RcTerm)>, CliError> {
        use LabelKind::*;

        let mut result = vec![];

        if let Some(ref matcher) = self.scope
            && !matcher.is_match(record, options)
        {
            return Ok(vec![]);
        }

        for label in self.labels.iter() {
            let p = match label.kind {
                Preferred => self.skos_ns.get("prefLabel"),
                Alternative => self.skos_ns.get("altLabel"),
                Hidden => self.skos_ns.get("hiddenLabel"),
            }
            .unwrap();

            for value in record.path(&label.path, options) {
                let value = translit(value.as_ref(), nf);
                let literal = value.as_ref() * xsd::string;
                let o = RcTerm::from_term(literal);

                result.push((p, o));
            }
        }

        Ok(result)
    }
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
