use std::collections::BTreeMap;

use marc21::{ByteRecord, Query, QueryOptions};
use serde::Deserialize;

use crate::commands::check::validator::{CommonOpts, Outcome};

#[inline(always)]
const fn duplicates_threshold() -> usize {
    2
}

#[inline(always)]
fn duplicates_separator() -> String {
    "|".into()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Duplicates {
    query: Query,

    #[serde(default = "duplicates_threshold")]
    threshold: usize,

    #[serde(default = "duplicates_separator")]
    separator: String,

    #[serde(flatten)]
    common: CommonOpts,
}

impl Duplicates {
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
        let mut freqs = BTreeMap::new();

        for row in record.query(&self.query, &options) {
            let values = row
                .iter()
                .map(|value| value.to_str_lossy())
                .collect::<Vec<_>>();

            if values.iter().all(|value| value.is_empty()) {
                continue;
            }

            freqs
                .entry(values.join(&self.separator))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        freqs.retain(|_, count| *count >= self.threshold);
        freqs.keys().for_each(|value| outcome.add_value(value));
        outcome.is_error(!freqs.is_empty());

        if self.common.invert_match {
            outcome.invert();
        }

        Some(outcome)
    }
}
