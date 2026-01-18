use winnow::ascii::multispace0;
use winnow::combinator::{alt, delimited};
use winnow::error::ParserError;
use winnow::prelude::*;
use winnow::stream::{AsChar, Stream, StreamIsPartial};
use winnow::token::{one_of, take_while};

/// Strip whitespaces from the beginning and end.
pub(crate) fn ws<I, O, E: ParserError<I>, F>(
    mut inner: F,
) -> impl Parser<I, O, E>
where
    I: Stream + StreamIsPartial,
    <I as Stream>::Token: AsChar + Clone,
    F: Parser<I, O, E>,
{
    move |i: &mut I| {
        let _ = multispace0.parse_next(i)?;
        let o = inner.parse_next(i);
        let _ = multispace0.parse_next(i)?;
        o
    }
}

pub(crate) fn parse_codes(i: &mut &[u8]) -> ModalResult<Vec<char>> {
    alt((
        one_of(AsChar::is_alphanum).map(|code| vec![code as char]),
        delimited(
            ws('['),
            take_while(1.., AsChar::is_alphanum),
            ws(']'),
        )
        .map(|codes: &[u8]| {
            let mut codes = codes
                .iter()
                .map(|code| *code as char)
                .collect::<Vec<char>>();
            codes.sort_unstable();
            codes.dedup();
            codes
        }),
    ))
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub enum ComparisonOp {
    Eq,
    Ne,
}

pub(crate) fn parse_comparison_op(
    i: &mut &[u8],
) -> ModalResult<ComparisonOp> {
    alt(("==".value(ComparisonOp::Eq), "!=".value(ComparisonOp::Ne)))
        .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comparison_op() {
        assert_eq!(
            parse_comparison_op.parse(b"==").unwrap(),
            ComparisonOp::Eq
        );

        assert_eq!(
            parse_comparison_op.parse(b"!=").unwrap(),
            ComparisonOp::Ne
        );
    }

    #[test]
    fn test_parse_codes() {
        assert_eq!(parse_codes.parse(b"0").unwrap(), vec!['0']);
        assert_eq!(parse_codes.parse(b"[01]").unwrap(), vec!['0', '1']);
        assert_eq!(
            parse_codes.parse(b"[010]").unwrap(),
            vec!['0', '1']
        );
        assert_eq!(
            parse_codes.parse(b"[012]").unwrap(),
            vec!['0', '1', '2']
        );
        assert_eq!(parse_codes.parse(b"a").unwrap(), vec!['a']);
        assert_eq!(parse_codes.parse(b"A").unwrap(), vec!['A']);
        assert!(parse_codes.parse(b".").is_err());
    }
}
