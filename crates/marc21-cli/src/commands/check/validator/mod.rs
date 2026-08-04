use marc21::matcher::RecordMatcher;
use serde::Deserialize;

mod filter;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "validator")]
pub(crate) enum Validator {
    Filter(Box<filter::Filter>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CommonOpts {
    scope: Option<RecordMatcher>,
    #[serde(default)]
    invert_match: bool,
}
