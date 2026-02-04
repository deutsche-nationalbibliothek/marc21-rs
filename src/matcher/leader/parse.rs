use winnow::combinator::alt;
use winnow::prelude::*;

use crate::matcher::LeaderMatcher;
use crate::matcher::leader::LeaderField;
use crate::matcher::shared::{
    parse_char_value, parse_comparison_operator, parse_u32_value, ws1,
};

pub(crate) fn parse_leader_matcher(
    i: &mut &[u8],
) -> ModalResult<LeaderMatcher> {
    let _prefix = "ldr.".parse_next(i)?;
    let field = parse_leader_field.parse_next(i)?;
    let operator = ws1(parse_comparison_operator).parse_next(i)?;
    let value = match field {
        LeaderField::BaseAddr | LeaderField::Length => parse_u32_value,
        _ => parse_char_value,
    }
    .parse_next(i)?;

    Ok(LeaderMatcher {
        field,
        operator,
        value,
    })
}

pub(crate) fn parse_leader_field(
    i: &mut &[u8],
) -> ModalResult<LeaderField> {
    alt((
        "base_address".value(LeaderField::BaseAddr),
        "encoding".value(LeaderField::Encoding),
        "length".value(LeaderField::Length),
        "status".value(LeaderField::Status),
        "type".value(LeaderField::Type),
    ))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcher::shared::ComparisonOperator;

    #[test]
    fn test_parse_leader_field() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_leader_field.parse($i.as_bytes()).unwrap(),
                    $o
                );
            };
        }

        parse_success!("base_address", LeaderField::BaseAddr);
        parse_success!("encoding", LeaderField::Encoding);
        parse_success!("length", LeaderField::Length);
        parse_success!("status", LeaderField::Status);
        parse_success!("type", LeaderField::Type);
    }

    #[test]
    fn test_parse_leader_matcher() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_leader_matcher.parse($i.as_bytes()).unwrap(),
                    $o
                );
            };
        }

        parse_success!(
            "ldr.length >= 100",
            LeaderMatcher {
                field: LeaderField::Length,
                operator: ComparisonOperator::Ge,
                value: 100u32.into(),
            }
        );

        parse_success!(
            "ldr.base_address == 32",
            LeaderMatcher {
                field: LeaderField::BaseAddr,
                operator: ComparisonOperator::Eq,
                value: 32u32.into(),
            }
        );

        parse_success!(
            "ldr.status != 'a'",
            LeaderMatcher {
                field: LeaderField::Status,
                operator: ComparisonOperator::Ne,
                value: b'a'.into(),
            }
        );

        parse_success!(
            "ldr.encoding == 'a'",
            LeaderMatcher {
                field: LeaderField::Encoding,
                operator: ComparisonOperator::Eq,
                value: b'a'.into(),
            }
        );

        parse_success!(
            "ldr.type != 'z'",
            LeaderMatcher {
                field: LeaderField::Type,
                operator: ComparisonOperator::Ne,
                value: b'z'.into(),
            }
        );

        assert!(parse_leader_matcher.parse(b"ldr.type=='z'").is_err());
        assert!(parse_leader_matcher.parse(b"ldr.type== 'z'").is_err());
        assert!(parse_leader_matcher.parse(b"ldr.type =='z'").is_err());

        assert!(
            parse_leader_matcher.parse(b"LDR.type == 'z'").is_err()
        );
    }
}
