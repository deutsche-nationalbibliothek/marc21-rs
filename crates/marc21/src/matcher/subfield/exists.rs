use smallvec::SmallVec;
use winnow::combinator::{empty, opt, seq};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::*;
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct ExistsMatcher {
    pub(crate) codes: SmallVec<[u8; 4]>,
    pub(crate) negated: bool,
}

impl ExistsMatcher {
    /// Checks whether the list of subfields contains at least one code
    /// from the list of allowed codes. If the matcher is in negated
    /// form, the matcher checks whether the list of subfields contains
    /// no subfields with a code from the referencelist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::{FieldMatcher, MatchOptions};
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let options = MatchOptions::default();
    ///
    /// let matcher = FieldMatcher::new("079.a?")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("079.y?")?;
    /// assert!(!matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("079{ !a? }")?;
    /// assert!(!matcher.is_match(record.fields(), &options));
    ////
    /// let matcher = FieldMatcher::new("079{ !y? }")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let result = subfields
            .into_iter()
            .any(|subfield| self.codes.contains(subfield.code()));

        if self.negated { !result } else { result }
    }
}

pub(crate) fn parse_exists_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { ExistsMatcher {
        negated: empty.value(false),
        codes: parse_codes.map(SmallVec::from),
        _: b'?'

    }}
    .map(|m| SubfieldMatcher::Exists(Box::new(m)))
    .parse_next(i)
}

pub(crate) fn parse_exists_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { ExistsMatcher {
        negated: opt('!').map(|value| value.is_some()),
        codes: parse_codes.map(SmallVec::from),
        _: b'?'

    }}
    .map(|m| SubfieldMatcher::Exists(Box::new(m)))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ByteRecord;
    use crate::common::TestResult;
    use crate::matcher::RecordMatcher;

    #[test]
    fn test_exists_matcher_short() -> TestResult {
        let data = include_bytes!("../../../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(data)?;
        let options = MatchOptions::default();

        let matcher = RecordMatcher::new("079.a?")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079.b?")?;
        assert!(!matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079.[ab]?")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079.[a-c]?")?;
        assert!(matcher.is_match(&record, &options));

        Ok(())
    }

    #[test]
    fn test_exists_matcher_long() -> TestResult {
        let data = include_bytes!("../../../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(data)?;
        let options = MatchOptions::default();

        let matcher = RecordMatcher::new("079{ a? }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ b? }")?;
        assert!(!matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ [ab]? }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ [a-c]? }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ *? }")?;
        assert!(matcher.is_match(&record, &options));

        // negation
        let matcher = RecordMatcher::new("079{ !a? }")?;
        assert!(!matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ !b? }")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ ![ab]? }")?;
        assert!(!matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ ![a-c]? }")?;
        assert!(!matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079{ !*? }")?;
        assert!(!matcher.is_match(&record, &options));

        Ok(())
    }

    #[test]
    fn test_parse_exists_matcher_short() {
        macro_rules! parse_success {
            ($i:expr, $codes:expr) => {
                assert_eq!(
                    parse_exists_matcher_short
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                        codes: SmallVec::from($codes),
                        negated: false,
                    }))
                );
            };
        }

        parse_success!("a?", vec![b'a']);
        parse_success!("[ab]?", vec![b'a', b'b']);
        parse_success!("[a-c]?", vec![b'a', b'b', b'c']);
        parse_success!(
            "*?",
            (b'0'..=b'9')
                .chain(b'a'..=b'z')
                .chain(b'A'..=b'Z')
                .collect::<Vec<u8>>()
        );

        macro_rules! parse_failure {
            ($i:expr) => {
                assert!(
                    parse_exists_matcher_short
                        .parse($i.as_bytes())
                        .is_err()
                );
            };
        }

        parse_failure!(":?");
        parse_failure!("!a?");
    }

    #[test]
    fn test_parse_exists_matcher_long() {
        macro_rules! parse_success {
            ($i:expr, $negated:expr, $codes:expr) => {
                assert_eq!(
                    parse_exists_matcher_long
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                        codes: SmallVec::from($codes),
                        negated: $negated,
                    }))
                );
            };
        }

        parse_success!("a?", false, vec![b'a']);
        parse_success!("[ab]?", false, vec![b'a', b'b']);
        parse_success!("[a-c]?", false, vec![b'a', b'b', b'c']);
        parse_success!(
            "*?",
            false,
            (b'0'..=b'9')
                .chain(b'a'..=b'z')
                .chain(b'A'..=b'Z')
                .collect::<Vec<u8>>()
        );

        parse_success!("!a?", true, vec![b'a']);
        parse_success!("![ab]?", true, vec![b'a', b'b']);
        parse_success!("![a-c]?", true, vec![b'a', b'b', b'c']);

        parse_success!(
            "!*?",
            true,
            (b'0'..=b'9')
                .chain(b'a'..=b'z')
                .chain(b'A'..=b'Z')
                .collect::<Vec<u8>>()
        );

        macro_rules! parse_failure {
            ($i:expr) => {
                assert!(
                    parse_exists_matcher_short
                        .parse($i.as_bytes())
                        .is_err()
                );
            };
        }

        parse_failure!(":?");
    }
}
