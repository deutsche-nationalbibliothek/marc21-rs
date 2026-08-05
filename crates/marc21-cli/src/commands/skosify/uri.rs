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
