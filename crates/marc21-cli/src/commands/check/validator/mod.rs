use marc21::matcher::RecordMatcher;
use serde::Deserialize;

mod datetime;
mod duplicates;
mod filter;
mod isbn;
mod issn;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "validator")]
pub(crate) enum Validator {
    Datetime(Box<datetime::Datetime>),
    Duplicates(Box<duplicates::Duplicates>),
    Filter(Box<filter::Filter>),
    Isbn(Box<isbn::Isbn>),
    Issn(Box<issn::Issn>),
}

#[derive(Debug, Default)]
pub(crate) struct Outcome {
    pub(crate) error: bool,
    pub(crate) values: Vec<String>,
}

impl Outcome {
    pub(crate) fn is_error(&mut self, yes: bool) {
        self.error = yes;
    }

    pub(crate) fn invert(&mut self) {
        self.error = !self.error;
    }

    pub(crate) fn add_value<S: ToString>(&mut self, value: S) {
        self.values.push(value.to_string());
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct CommonOpts {
    scope: Option<RecordMatcher>,
    #[serde(default)]
    invert_match: bool,
}
