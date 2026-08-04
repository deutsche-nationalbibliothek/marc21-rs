use marc21::ByteRecord;
use marc21::matcher::RecordMatcher;
use serde::Deserialize;

use crate::commands::check::validator::CommonOpts;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Filter {
    matcher: RecordMatcher,

    #[serde(flatten)]
    common: CommonOpts,
}

impl Filter {
    pub(crate) fn is_valid(&self, record: &ByteRecord) -> bool {
        let options = Default::default();

        if let Some(ref matcher) = self.common.scope
            && !matcher.is_match(record, &options)
        {
            return false;
        }

        let mut result =
            !self.matcher.is_match(record, &Default::default());

        if self.common.invert_match {
            result = !result
        }

        result
    }
}
