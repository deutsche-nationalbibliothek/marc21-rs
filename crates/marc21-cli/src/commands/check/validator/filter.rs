use marc21::ByteRecord;
use marc21::matcher::RecordMatcher;
use serde::Deserialize;

use crate::commands::check::validator::{CommonOpts, Outcome};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Filter {
    matcher: RecordMatcher,

    #[serde(flatten)]
    common: CommonOpts,
}

impl Filter {
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

        if self.matcher.is_match(record, &Default::default()) {
            outcome.is_error(true);
        }

        if self.common.invert_match {
            outcome.invert();
        }

        Some(outcome)
    }
}
