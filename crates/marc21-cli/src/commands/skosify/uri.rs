use marc21::{Path, QueryOptions, StringRecord};
use serde::Deserialize;
use sophia::iri::{InvalidIri, IriRef};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub(crate) enum Uri {
    Path { path: Path },
}

impl Uri {
    pub(crate) fn get(
        &self,
        record: &StringRecord,
        options: &QueryOptions,
    ) -> Result<IriRef<String>, InvalidIri> {
        match self {
            Self::Path { path } => {
                let iri = record
                    .first(path, options)
                    .map(|value| {
                        value.to_str_unchecked().to_string()
                        // .replace(' ', "%20")
                    })
                    .unwrap_or_default();

                IriRef::new(iri)
            }
        }
    }

    pub(crate) fn all(
        &self,
        record: &StringRecord,
        options: &QueryOptions,
    ) -> Vec<Result<IriRef<String>, InvalidIri>> {
        let mut result = vec![];

        match self {
            Self::Path { path } => {
                for value in record.path(path, options) {
                    let iri = value.to_str_unchecked().to_string();
                    result.push(IriRef::new(iri));
                }
            }
        }

        result
    }
}
