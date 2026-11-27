use marc21::{ByteRecord, Path, QueryOptions};
use serde::Deserialize;

use crate::commands::check::validator::{CommonOpts, Outcome};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Issn {
    path: Path,

    #[serde(flatten)]
    common: CommonOpts,
}

impl Issn {
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

        let options = QueryOptions::from(options);

        for value in record.path(&self.path, &options) {
            if issn::Issn::new(&value).is_err() {
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
