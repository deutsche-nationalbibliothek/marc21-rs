use winnow::combinator::{alt, separated, seq};
use winnow::prelude::*;

use crate::matcher::shared::ws0;
use crate::query::control_field::parse_control_field_expr;
use crate::query::data_field::parse_data_field_expr;
use crate::query::leader::parse_leader_expr;
use crate::query::{Constituent, Kind, Query};

pub(crate) fn parse_query(i: &mut &[u8]) -> ModalResult<Query> {
    ws0(parse_constituents)
        .with_taken()
        .map(|(constituents, input)| Query {
            constituents,
            input: input.to_vec(),
        })
        .parse_next(i)
}

#[inline(always)]
fn parse_constituents(i: &mut &[u8]) -> ModalResult<Vec<Constituent>> {
    separated(1.., parse_constituent, ws0(',')).parse_next(i)
}

fn parse_constituent(i: &mut &[u8]) -> ModalResult<Constituent> {
    seq! { Constituent {
        kind: parse_constituent_kind,
    }}
    .parse_next(i)
}

fn parse_constituent_kind(i: &mut &[u8]) -> ModalResult<Kind> {
    alt((
        parse_data_field_expr.map(Kind::DataField),
        parse_control_field_expr.map(Kind::ControlField),
        parse_leader_expr.map(Kind::Leader),
    ))
    .parse_next(i)
}
