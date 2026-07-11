use winnow::combinator::preceded;
use winnow::prelude::*;

use crate::Subfield;
use crate::matcher::shared::*;
use crate::matcher::{MatchOptions, SubfieldMatcher};

#[derive(Debug, Clone, PartialEq)]
pub struct CountMatcher {
    pub(crate) codes: Vec<u8>,
    pub(crate) operator: ComparisonOperator,
    pub(crate) value: usize,
}

impl CountMatcher {
    pub fn is_match<'a, S: IntoIterator<Item = &'a Subfield<'a>>>(
        &self,
        subfields: S,
        _options: &MatchOptions,
    ) -> bool {
        let count = subfields
            .into_iter()
            .filter(|subfield| self.codes.contains(subfield.code()))
            .count();

        match self.operator {
            ComparisonOperator::Eq => count == self.value,
            ComparisonOperator::Ne => count != self.value,
            ComparisonOperator::Ge => count >= self.value,
            ComparisonOperator::Gt => count > self.value,
            ComparisonOperator::Le => count <= self.value,
            ComparisonOperator::Lt => count < self.value,
        }
    }
}

pub(crate) fn parse_count_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    preceded(
        '#',
        (parse_codes, ws1(parse_comparison_operator), parse_usize),
    )
    .map(|(codes, operator, value)| CountMatcher {
        codes,
        operator,
        value,
    })
    .map(|matcher| SubfieldMatcher::Count(Box::new(matcher)))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_count_matcher() {
        use ComparisonOperator::*;

        macro_rules! parse_success {
            ($i:expr, $codes:expr, $op:expr, $value:expr) => {
                assert_eq!(
                    parse_count_matcher.parse($i.as_bytes()).unwrap(),
                    SubfieldMatcher::Count(Box::new(CountMatcher {
                        codes: $codes,
                        operator: $op,
                        value: $value,
                    })),
                )
            };
        }

        parse_success!("#a == 0", vec![b'a'], Eq, 0);
        parse_success!("#a != 1", vec![b'a'], Ne, 1);
        parse_success!("#a >= 2", vec![b'a'], Ge, 2);
        parse_success!("#a > 3", vec![b'a'], Gt, 3);
        parse_success!("#a <= 4", vec![b'a'], Le, 4);
        parse_success!("#a < 5", vec![b'a'], Lt, 5);

        parse_success!("#[ab] == 0", vec![b'a', b'b'], Eq, 0);
    }
}
