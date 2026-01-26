use winnow::combinator::alt;
use winnow::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum ComparisonOperator {
    Eq,
    Ne,
    Ge,
    Gt,
    Le,
    Lt,
}

pub(crate) fn parse_comparison_operator(
    i: &mut &[u8],
) -> ModalResult<ComparisonOperator> {
    alt((
        "==".value(ComparisonOperator::Eq),
        "!=".value(ComparisonOperator::Ne),
        ">=".value(ComparisonOperator::Ge),
        ">".value(ComparisonOperator::Gt),
        "<=".value(ComparisonOperator::Le),
        "<".value(ComparisonOperator::Lt),
    ))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comparison_operator() {
        assert_eq!(
            parse_comparison_operator.parse(b"==").unwrap(),
            ComparisonOperator::Eq
        );
        assert_eq!(
            parse_comparison_operator.parse(b"!=").unwrap(),
            ComparisonOperator::Ne
        );
        assert_eq!(
            parse_comparison_operator.parse(b">=").unwrap(),
            ComparisonOperator::Ge
        );
        assert_eq!(
            parse_comparison_operator.parse(b">").unwrap(),
            ComparisonOperator::Gt
        );
        assert_eq!(
            parse_comparison_operator.parse(b"<=").unwrap(),
            ComparisonOperator::Le
        );
        assert_eq!(
            parse_comparison_operator.parse(b"<").unwrap(),
            ComparisonOperator::Lt
        );
    }
}
