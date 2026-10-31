use std::str::FromStr;

use marc21::{ByteRecord, Path, QueryOptions};
use serde::Deserialize;

use crate::commands::check::validator::{CommonOpts, Outcome};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Format {
    Isbn10,
    Isbn13,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Isbn {
    path: Path,

    format: Option<Format>,

    #[serde(flatten)]
    common: CommonOpts,
}

impl Isbn {
    pub(crate) fn is_valid(
        &self,
        record: &ByteRecord,
    ) -> Option<Outcome> {
        use Format::*;

        let mut outcome = Outcome::default();
        let options = Default::default();

        if let Some(ref matcher) = self.common.scope
            && !matcher.is_match(record, &options)
        {
            return None;
        }

        let options = QueryOptions::from(options);

        for value in record.path(&self.path, &options) {
            let Ok(value) = String::from_utf8(value.to_vec()) else {
                outcome.add_value(value.to_str_lossy());
                outcome.is_error(true);
                continue;
            };

            let error = match self.format {
                Some(Isbn10) => isbn::Isbn10::from_str(&value).is_err(),
                Some(Isbn13) => isbn::Isbn13::from_str(&value).is_err(),
                None => isbn::Isbn::from_str(&value).is_err(),
            };

            if error {
                outcome.add_value(&value);
                outcome.is_error(true);
            }
        }

        if self.common.invert_match {
            outcome.invert();
        }

        Some(outcome)
    }
}
