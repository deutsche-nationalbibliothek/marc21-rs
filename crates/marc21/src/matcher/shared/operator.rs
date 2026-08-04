use winnow::combinator::alt;
use winnow::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum BooleanOp {
    And,
    Or,
}

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
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_comparison_operator
                        .parse($i.as_bytes())
                        .unwrap(),
                    $o
                );
            };
        }

        parse_success!("==", ComparisonOperator::Eq);
        parse_success!("!=", ComparisonOperator::Ne);
        parse_success!(">=", ComparisonOperator::Ge);
        parse_success!(">", ComparisonOperator::Gt);
        parse_success!("<=", ComparisonOperator::Le);
        parse_success!("<", ComparisonOperator::Lt);
    }
}
