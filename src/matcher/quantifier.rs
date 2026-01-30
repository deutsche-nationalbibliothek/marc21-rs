use winnow::combinator::alt;
use winnow::prelude::*;

#[derive(Debug, PartialEq, Clone, Default)]
#[allow(dead_code)]
pub(crate) enum Quantifier {
    All,
    #[default]
    Any,
}

#[allow(dead_code)]
pub(crate) fn parse_quantifier(
    i: &mut &[u8],
) -> ModalResult<Quantifier> {
    alt((b"ALL".value(Quantifier::All), b"ANY".value(Quantifier::Any)))
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
