use winnow::ascii::multispace1;
use winnow::combinator::{alt, opt, terminated};
use winnow::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub(crate) enum Quantifier {
    All,
    #[default]
    Any,
}

pub(crate) fn parse_quantifier(
    i: &mut &[u8],
) -> ModalResult<Quantifier> {
    alt((b"ALL".value(Quantifier::All), b"ANY".value(Quantifier::Any)))
        .parse_next(i)
}

pub(crate) fn parse_quantifier_opt(
    i: &mut &[u8],
) -> ModalResult<Quantifier> {
    opt(terminated(parse_quantifier, multispace1))
        .map(Option::unwrap_or_default)
        .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_quantifier() {
        use Quantifier::*;

        assert_eq!(parse_quantifier.parse(b"ALL").unwrap(), All);
        assert_eq!(parse_quantifier.parse(b"ANY").unwrap(), Any);
    }
}
