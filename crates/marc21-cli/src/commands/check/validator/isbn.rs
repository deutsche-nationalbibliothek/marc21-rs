use std::str::FromStr;

use marc21::{ByteRecord, Path};
use serde::Deserialize;

use crate::commands::check::validator::{CommonOpts, Outcome};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Isbn {
    path: Path,

    #[serde(flatten)]
    common: CommonOpts,
}

impl Isbn {
    pub(crate) fn is_valid(
        &self,
        record: &ByteRecord,
    ) -> Option<Outcome> {
        let mut outcome = Outcome::default();
        let options = Default::default();

        if let Some(ref matcher) = self.common.scope
            && !matcher.is_match(record, &options)
        {
            return None;
        }

        let values = record.path(&self.path, &Default::default());

        for value in values {
            if let Ok(isbn_str) = String::from_utf8(value.to_vec()) {
                if isbn::Isbn::from_str(&isbn_str).is_err() {
                    outcome.add_value(&isbn_str);
                    outcome.is_error(true);
                }
            } else {
                outcome.add_value(value.to_str_lossy());
                outcome.is_error(true);
            }
        }

        if self.common.invert_match {
            outcome.invert();
        }

        Some(outcome)
    }
}
