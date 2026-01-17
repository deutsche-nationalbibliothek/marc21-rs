use std::fmt::{self, Display};

use winnow::combinator::alt;

use crate::parse::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Quantifier {
    All,
    #[default]
    Any,
}

impl Display for Quantifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "ALL"),
            Self::Any => write!(f, "ANY"),
        }
    }
}

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
