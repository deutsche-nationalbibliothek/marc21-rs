use marc21::matcher::MatchOptions;
use marc21::{Path, StringRecord};
use serde::Deserialize;
use sophia::iri::{InvalidIri, IriRef};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum Uri {
    Path {
        path: Path,
    },
    #[serde(rename_all = "kebab-case")]
    Base {
        base_uri: String,
        path: Path,
    },
}

impl Uri {
    pub(crate) fn get(
        &self,
        record: &StringRecord,
        options: &MatchOptions,
    ) -> Result<IriRef<String>, InvalidIri> {
        match self {
            Self::Path { path } => {
                let iri = record
                    .first(path, options)
                    .map(|value| value.to_str_unchecked().to_string())
                    .unwrap_or_default();

                IriRef::new(iri)
            }
            Self::Base { base_uri, path } => {
                let suffix = record
                    .first(path, options)
                    .map(|value| value.to_str_unchecked().to_string())
                    .unwrap_or_default();

                IriRef::new(format!("{base_uri}{suffix}"))
            }
        }
    }

    pub(crate) fn all(
        &self,
        record: &StringRecord,
        options: &MatchOptions,
    ) -> Vec<Result<IriRef<String>, InvalidIri>> {
        let mut result = vec![];

        match self {
            Self::Path { path } => {
                for value in record.path(path, options) {
                    let iri = value.to_str_unchecked().to_string();
                    result.push(IriRef::new(iri));
                }
            }
            Self::Base { base_uri, path } => {
                for value in record.path(path, options) {
                    let suffix = value.to_str_unchecked();
                    result.push(IriRef::new(format!(
                        "{base_uri}{suffix}"
                    )));
                }
            }
        }

        result
    }
}
