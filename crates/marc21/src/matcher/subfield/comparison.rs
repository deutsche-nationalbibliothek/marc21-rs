use smallvec::SmallVec;
use winnow::combinator::{empty, seq};
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::{
    ComparisonOperator, Quantifier, Value, parse_codes,
    parse_comparison_operator, parse_quantifier_opt,
    parse_string_value, ws1,
};
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, PartialEq, Clone)]
pub struct ComparisonMatcher {
    pub(crate) quantifier: Quantifier,
    pub(crate) codes: SmallVec<[u8; 4]>,
    pub(crate) operator: ComparisonOperator,
    pub(crate) value: Value,
}

impl ComparisonMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let mut subfields = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()));

        let r#fn = |subfield: &Subfield| -> bool {
            let value = subfield.value();
            match self.operator {
                ComparisonOperator::Eq => value == self.value,
                ComparisonOperator::Ne => value != self.value,
                ComparisonOperator::Ge => value >= self.value,
                ComparisonOperator::Gt => value > self.value,
                ComparisonOperator::Le => value <= self.value,
                ComparisonOperator::Lt => value < self.value,
            }
        };

        match self.quantifier {
            Quantifier::Any => subfields.any(r#fn),
            Quantifier::All => subfields.all(r#fn),
        }
    }
}

pub(crate) fn parse_comparison_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { ComparisonMatcher {
        quantifier: empty.value(Quantifier::Any),
        codes: parse_codes.map(SmallVec::from),
        operator: ws1( parse_comparison_operator ),
        value: parse_string_value,
    }}
    .map(|m| SubfieldMatcher::Comparison(Box::new(m)))
    .parse_next(i)
}

pub(crate) fn parse_comparison_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    seq! { ComparisonMatcher {
        quantifier: parse_quantifier_opt,
        codes: parse_codes.map(SmallVec::from),
        operator: ws1( parse_comparison_operator ),
        value: parse_string_value,
    }}
    .map(|m| SubfieldMatcher::Comparison(Box::new(m)))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ByteRecord;
    use crate::common::TestResult;
    use crate::matcher::RecordMatcher;

    #[test]
    fn test_comparison_matcher() -> TestResult {
        let data = include_bytes!("../../../tests/data/ada.mrc");
        let record = ByteRecord::from_bytes(data)?;
        let options = MatchOptions::default();

        let matcher = RecordMatcher::new("042.a == 'gnd1'")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("042.a != 'gnd1'")?;
        assert!(!matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079.u > 'v'")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079.u >= 'w'")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079.u < 'w'")?;
        assert!(matcher.is_match(&record, &options));

        let matcher = RecordMatcher::new("079.u <= 'w'")?;
        assert!(matcher.is_match(&record, &options));

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_comparison_matcher_short() {
        use ComparisonOperator::*;
        use Quantifier::*;

        macro_rules! parse_success {
            ($i:expr, $inner:expr) => {
                assert_eq!(
                    parse_comparison_matcher_short
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Comparison(Box::new($inner))
                );
            };
        }

        parse_success!("a == 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Eq, value: "foo".into() });
        parse_success!("a != 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Ne, value: "foo".into() });
        parse_success!("a >= 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Ge, value: "foo".into() });
        parse_success!("a > 'foo'", ComparisonMatcher { quantifier: Any,  codes: SmallVec::from(vec![b'a']), operator: Gt, value: "foo".into() });
        parse_success!("a <= 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Le, value: "foo".into() });
        parse_success!("a < 'foo'", ComparisonMatcher { quantifier: Any,  codes: SmallVec::from(vec![b'a']), operator: Lt, value: "foo".into() });
    }

    #[test]
    #[rustfmt::skip]
    fn test_parse_comparison_matcher_long() {
        use ComparisonOperator::*;
        use Quantifier::*;

        macro_rules! parse_success {
            ($i:expr, $inner:expr) => {
                assert_eq!(
                    parse_comparison_matcher_long
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Comparison(Box::new($inner))
                );
            };
        }

        parse_success!("a == 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Eq, value: "foo".into() });
        parse_success!("a != 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Ne, value: "foo".into() });
        parse_success!("a >= 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Ge, value: "foo".into() });
        parse_success!("a > 'foo'", ComparisonMatcher { quantifier: Any,  codes: SmallVec::from(vec![b'a']), operator: Gt, value: "foo".into() });
        parse_success!("a <= 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Le, value: "foo".into() });
        parse_success!("a < 'foo'", ComparisonMatcher { quantifier: Any,  codes: SmallVec::from(vec![b'a']), operator: Lt, value: "foo".into() });

        parse_success!("ANY a == 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Eq, value: "foo".into() });
        parse_success!("ANY a != 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Ne, value: "foo".into() });
        parse_success!("ANY a >= 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Ge, value: "foo".into() });
        parse_success!("ANY a > 'foo'", ComparisonMatcher { quantifier: Any,  codes: SmallVec::from(vec![b'a']), operator: Gt, value: "foo".into() });
        parse_success!("ANY a <= 'foo'", ComparisonMatcher { quantifier: Any, codes: SmallVec::from(vec![b'a']), operator: Le, value: "foo".into() });
        parse_success!("ANY a < 'foo'", ComparisonMatcher { quantifier: Any,  codes: SmallVec::from(vec![b'a']), operator: Lt, value: "foo".into() });

        parse_success!("ALL a == 'foo'", ComparisonMatcher { quantifier: All, codes: SmallVec::from(vec![b'a']), operator: Eq, value: "foo".into() });
        parse_success!("ALL a != 'foo'", ComparisonMatcher { quantifier: All, codes: SmallVec::from(vec![b'a']), operator: Ne, value: "foo".into() });
        parse_success!("ALL a >= 'foo'", ComparisonMatcher { quantifier: All, codes: SmallVec::from(vec![b'a']), operator: Ge, value: "foo".into() });
        parse_success!("ALL a > 'foo'", ComparisonMatcher { quantifier: All,  codes: SmallVec::from(vec![b'a']), operator: Gt, value: "foo".into() });
        parse_success!("ALL a <= 'foo'", ComparisonMatcher { quantifier: All, codes: SmallVec::from(vec![b'a']), operator: Le, value: "foo".into() });
        parse_success!("ALL a < 'foo'", ComparisonMatcher { quantifier: All,  codes: SmallVec::from(vec![b'a']), operator: Lt, value: "foo".into() });
    }
}
