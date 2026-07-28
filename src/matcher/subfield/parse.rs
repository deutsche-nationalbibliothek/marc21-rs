use std::cell::RefCell;

use winnow::combinator::{
    alt, delimited, preceded, repeat, terminated,
};
use winnow::prelude::*;

use crate::matcher::shared::ws0;
use crate::matcher::subfield::SubfieldMatcher;
use crate::matcher::subfield::comparison::*;
use crate::matcher::subfield::count::*;
use crate::matcher::subfield::exists::*;
use crate::matcher::subfield::member::*;
use crate::matcher::subfield::prefix::*;
use crate::matcher::subfield::regex::*;
use crate::matcher::subfield::strsim::*;
use crate::matcher::subfield::substr::*;
use crate::matcher::subfield::suffix::*;

pub(crate) fn parse_subfield_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((
        parse_group_matcher,
        parse_boolean_connective,
        parse_not_matcher,
        alt((
            parse_comparison_matcher_long,
            parse_exists_matcher_long,
            parse_count_matcher_long,
            parse_prefix_matcher_long,
            parse_suffix_matcher_long,
            parse_member_matcher_long,
            parse_substr_matcher_long,
            parse_regex_matcher_long,
            parse_strsim_matcher_long,
        )),
    ))
    .map(|matcher| {
        group_level_reset();
        matcher
    })
    .parse_next(i)
}

pub(crate) fn parse_subfield_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((
        parse_comparison_matcher_short,
        parse_exists_matcher_short,
        parse_prefix_matcher_short,
        parse_suffix_matcher_short,
        parse_member_matcher_short,
        parse_substr_matcher_short,
        parse_regex_matcher_short,
        parse_strsim_matcher_short,
    ))
    .parse_next(i)
}

thread_local! {
    pub static GROUP_LEVEL: RefCell<u32> = const { RefCell::new(0) };
}

fn group_level_incr(i: &mut &[u8]) -> ModalResult<()> {
    GROUP_LEVEL.with(|level| {
        *level.borrow_mut() += 1;

        if *level.borrow() > 256 {
            Err(winnow::error::ParserError::from_input(i))
        } else {
            Ok(())
        }
    })
}

fn group_level_decr() {
    GROUP_LEVEL.with(|level| *level.borrow_mut() -= 1);
}

fn group_level_reset() {
    GROUP_LEVEL.with(|level| *level.borrow_mut() = 0);
}

fn parse_group_matcher(i: &mut &[u8]) -> ModalResult<SubfieldMatcher> {
    delimited(
        terminated(ws0('('), group_level_incr),
        alt((
            parse_boolean_connective,
            parse_comparison_matcher_long,
            parse_exists_matcher_long,
            parse_count_matcher_long,
            parse_prefix_matcher_long,
            parse_suffix_matcher_long,
            parse_member_matcher_long,
            alt((
                parse_substr_matcher_long,
                parse_regex_matcher_long,
                parse_strsim_matcher_long,
            )),
            alt((parse_group_matcher, parse_not_matcher)),
        )),
        ws0(')').map(|_| group_level_decr),
    )
    .map(|m| SubfieldMatcher::Group(Box::new(m)))
    .parse_next(i)
}

fn parse_not_matcher(i: &mut &[u8]) -> ModalResult<SubfieldMatcher> {
    preceded(ws0('!'), alt((parse_group_matcher,)))
        .map(|m| SubfieldMatcher::Not(Box::new(m)))
        .parse_next(i)
}

fn parse_boolean_connective(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((parse_boolean_connective_or, parse_boolean_connective_and))
        .parse_next(i)
}

fn parse_boolean_connective_or(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    let atom = |i: &mut &[u8]| -> ModalResult<SubfieldMatcher> {
        ws0(alt((
            parse_boolean_connective_and,
            parse_group_matcher,
            alt((
                parse_comparison_matcher_long,
                parse_exists_matcher_long,
                parse_count_matcher_long,
                parse_prefix_matcher_long,
                parse_suffix_matcher_long,
                parse_member_matcher_long,
                parse_substr_matcher_long,
                parse_regex_matcher_long,
                parse_strsim_matcher_long,
            )),
        )))
        .parse_next(i)
    };

    (atom, repeat(1.., preceded(ws0("||"), atom)))
        .map(|(head, tail): (_, Vec<_>)| {
            tail.into_iter().fold(head, |prev, next| prev | next)
        })
        .parse_next(i)
}

fn parse_boolean_connective_and(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    let atom = |i: &mut &[u8]| -> ModalResult<SubfieldMatcher> {
        ws0(alt((
            parse_group_matcher,
            parse_comparison_matcher_long,
            parse_exists_matcher_long,
            parse_count_matcher_long,
            parse_prefix_matcher_long,
            parse_suffix_matcher_long,
            parse_member_matcher_long,
            parse_substr_matcher_long,
            alt((parse_regex_matcher_long, parse_strsim_matcher_long)),
        )))
        .parse_next(i)
    };

    (atom, repeat(1.., preceded(ws0("&&"), atom)))
        .map(|(head, tail): (_, Vec<_>)| {
            tail.into_iter().fold(head, |prev, next| prev & next)
        })
        .parse_next(i)
}

#[cfg(test)]
mod test {
    use aho_corasick::AhoCorasick;
    use regex::bytes::RegexSet;
    use smallvec::SmallVec;

    use super::*;
    use crate::matcher::shared::{
        BooleanOp, ComparisonOperator, Quantifier,
    };
    use crate::matcher::subfield::comparison::ComparisonMatcher;
    use crate::matcher::subfield::count::CountMatcher;
    use crate::matcher::subfield::exists::ExistsMatcher;
    use crate::matcher::subfield::member::MemberMatcher;
    use crate::matcher::subfield::prefix::PrefixMatcher;
    use crate::matcher::subfield::substr::SubstrMatcher;
    use crate::matcher::subfield::suffix::SuffixMatcher;

    #[test]
    fn test_parse_subfield_matcher_short() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_subfield_matcher_short
                        .parse($i.as_bytes())
                        .unwrap(),
                    $o
                );
            };
        }

        // comparison
        parse_success!(
            "a == 'foo'",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Eq,
                value: "foo".into()
            }))
        );

        parse_success!(
            "[bc] != 'foo'",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'b', b'c']),
                operator: ComparisonOperator::Ne,
                value: "foo".into()
            }))
        );

        // exists
        parse_success!(
            "a?",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            }))
        );

        // prefix
        parse_success!(
            "a =^ 'foo'",
            SubfieldMatcher::Prefix(Box::new(PrefixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                prefixes: vec![b"foo".into()],
            }))
        );

        // suffix
        parse_success!(
            "a =$ 'foo'",
            SubfieldMatcher::Suffix(Box::new(SuffixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                suffixes: vec![b"foo".into()],
            }))
        );

        // member
        parse_success!(
            "a in ['foo', 'bar']",
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                values: vec![b"foo".into(), b"bar".into()],
            }))
        );

        parse_success!(
            "a not in ['foo', 'bar', ]",
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: true,
                values: vec![b"foo".into(), b"bar".into()],
            }))
        );

        // substr
        parse_success!(
            "a =? 'foo'",
            SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"foo".into()],
                ac: AhoCorasick::new([b"foo"]).unwrap(),
            }))
        );

        // regex
        parse_success!(
            "a =~ '^foo'",
            SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"^foo".into()],
                matcher: RegexSet::new(vec!["^foo"]).unwrap(),
            }))
        );

        // strsim
        parse_success!(
            "a =* 'foo'",
            SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec!["foo".into()],
            }))
        );
    }

    #[test]
    fn test_parse_subfield_matcher_long() {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_subfield_matcher_long
                        .parse($i.as_bytes())
                        .unwrap(),
                    $o
                );
            };
        }

        // exists matcher
        parse_success!(
            "!a?",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: true
            }))
        );

        // count matcher
        parse_success!(
            "#a == 1",
            SubfieldMatcher::Count(Box::new(CountMatcher {
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Eq,
                value: 1
            }))
        );

        // comparison
        parse_success!(
            "a == 'foo'",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Eq,
                value: "foo".into()
            }))
        );

        parse_success!(
            "[bc] != 'foo'",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'b', b'c']),
                operator: ComparisonOperator::Ne,
                value: "foo".into()
            }))
        );

        // prefix
        parse_success!(
            "a =^ 'foo'",
            SubfieldMatcher::Prefix(Box::new(PrefixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                prefixes: vec![b"foo".into()],
            }))
        );

        // suffix
        parse_success!(
            "a =$ 'foo'",
            SubfieldMatcher::Suffix(Box::new(SuffixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                suffixes: vec![b"foo".into()],
            }))
        );

        // member
        parse_success!(
            "a in ['foo', 'bar']",
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                values: vec![b"foo".into(), b"bar".into()],
            }))
        );

        parse_success!(
            "a not in ['foo', 'bar', ]",
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: true,
                values: vec![b"foo".into(), b"bar".into()],
            }))
        );

        // substr
        parse_success!(
            "a =? ['foo', 'bar']",
            SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"foo".into(), b"bar".into()],
                ac: AhoCorasick::new([b"foo", b"bar"]).unwrap(),
            }))
        );

        // regex
        parse_success!(
            "a =~ '^foo'",
            SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"^foo".into()],
                matcher: RegexSet::new(vec!["^foo"]).unwrap(),
            }))
        );

        // strsim
        parse_success!(
            "a =* 'foo'",
            SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec!["foo".into()],
            }))
        );

        // boolean connective or
        parse_success!(
            "a? || b?",
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                ))),
                op: BooleanOp::Or,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                )))
            }
        );

        // boolean connective and
        parse_success!(
            "a? && b?",
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                ))),
                op: BooleanOp::And,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                )))
            }
        );

        // group
        parse_success!(
            "(!a?)",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Exists(
                Box::new(ExistsMatcher {
                    codes: SmallVec::from(vec![b'a']),
                    negated: true
                })
            )))
        );

        // not
        parse_success!(
            "!(!a?)",
            SubfieldMatcher::Not(Box::new(SubfieldMatcher::Group(
                Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: true
                    }
                )))
            )))
        );

        // complex example
        parse_success!(
            "(((!a?) || b?) && c?)",
            SubfieldMatcher::Group(Box::new(
                SubfieldMatcher::Composite {
                    lhs: Box::new(SubfieldMatcher::Group(Box::new(
                        SubfieldMatcher::Composite {
                            lhs: Box::new(SubfieldMatcher::Group(
                                Box::new(SubfieldMatcher::Exists(
                                    Box::new(ExistsMatcher {
                                        codes: SmallVec::from(vec![
                                            b'a'
                                        ]),
                                        negated: true
                                    })
                                ))
                            )),
                            op: BooleanOp::Or,
                            rhs: Box::new(SubfieldMatcher::Exists(
                                Box::new(ExistsMatcher {
                                    codes: SmallVec::from(vec![b'b']),
                                    negated: false
                                })
                            ))
                        }
                    ))),
                    op: BooleanOp::And,
                    rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                        ExistsMatcher {
                            codes: SmallVec::from(vec![b'c']),
                            negated: false
                        }
                    )))
                }
            ))
        );
    }

    #[test]
    fn test_parse_group_matcher() {
        macro_rules! parse_success {
            ($i:expr, $inner:expr) => {
                assert_eq!(
                    parse_group_matcher.parse($i.as_bytes()).unwrap(),
                    SubfieldMatcher::Group(Box::new($inner)),
                );
            };
        }

        // comparison
        parse_success!(
            "(x != '')",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'x']),
                operator: ComparisonOperator::Ne,
                value: "".into(),
            }))
        );

        parse_success!(
            "(ALL a >= 'bar')",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::All,
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Ge,
                value: "bar".into(),
            }))
        );

        // exists
        parse_success!(
            "(a?)",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            }))
        );

        // count
        parse_success!(
            "(#a >= 1)",
            SubfieldMatcher::Count(Box::new(CountMatcher {
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Ge,
                value: 1
            }))
        );

        // prefix
        parse_success!(
            "(a =^ 'foo')",
            SubfieldMatcher::Prefix(Box::new(PrefixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                prefixes: vec![b"foo".into()],
            }))
        );

        // suffix
        parse_success!(
            "(a =$ ['foo', 'bar'])",
            SubfieldMatcher::Suffix(Box::new(SuffixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                suffixes: vec![b"foo".into(), b"bar".into()],
            }))
        );

        // member
        parse_success!(
            "(ALL a in ['foo', 'bar'])",
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::All,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                values: vec![b"foo".into(), b"bar".into()],
            }))
        );

        // substr
        parse_success!(
            "(ANY a =? 'foo')",
            SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"foo".into()],
                ac: AhoCorasick::new([b"foo"]).unwrap(),
            }))
        );

        // regex
        parse_success!(
            "(a =~ '^foo')",
            SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"^foo".into()],
                matcher: RegexSet::new(vec!["^foo"]).unwrap(),
            }))
        );

        // strsim
        parse_success!(
            "(a =* 'foo')",
            SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec!["foo".into()],
            }))
        );

        // group
        parse_success!(
            "((a?))",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Exists(
                Box::new(ExistsMatcher {
                    codes: SmallVec::from(vec![b'a']),
                    negated: false
                })
            )))
        );

        // not
        parse_success!(
            "(!(a?))",
            SubfieldMatcher::Not(Box::new(SubfieldMatcher::Group(
                Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                )))
            )))
        );

        parse_success!(
            "(a? || b?)",
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                ))),
                op: BooleanOp::Or,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                ))),
            }
        );

        parse_success!(
            "(a? && b?)",
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                ))),
                op: BooleanOp::And,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                ))),
            }
        );
    }

    #[test]
    fn test_parse_not_matcher() {
        macro_rules! parse_success {
            ($i:expr, $inner:expr) => {
                assert_eq!(
                    parse_not_matcher.parse($i.as_bytes()).unwrap(),
                    SubfieldMatcher::Not(Box::new($inner)),
                );
            };
        }

        // comparison
        parse_success!(
            "!(ANY [x-z] == 'baz')",
            SubfieldMatcher::Group(Box::new(
                SubfieldMatcher::Comparison(Box::new(
                    ComparisonMatcher {
                        quantifier: Quantifier::Any,
                        codes: SmallVec::from(vec![b'x', b'y', b'z']),
                        operator: ComparisonOperator::Eq,
                        value: "baz".into(),
                    }
                ))
            ))
        );

        // exists
        parse_success!(
            "!(a?)",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Exists(
                Box::new(ExistsMatcher {
                    codes: SmallVec::from(vec![b'a']),
                    negated: false
                })
            )))
        );

        // count
        parse_success!(
            "!(#[ab] > 3)",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Count(
                Box::new(CountMatcher {
                    codes: SmallVec::from(vec![b'a', b'b']),
                    operator: ComparisonOperator::Gt,
                    value: 3,
                })
            )))
        );

        // prefix
        parse_success!(
            "!(a =^ 'foo')",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Prefix(
                Box::new(PrefixMatcher {
                    quantifier: Quantifier::Any,
                    codes: SmallVec::from(vec![b'a']),
                    negated: false,
                    prefixes: vec![b"foo".into()],
                })
            )))
        );

        // suffix
        parse_success!(
            "!(a =$ 'foo')",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Suffix(
                Box::new(SuffixMatcher {
                    quantifier: Quantifier::Any,
                    codes: SmallVec::from(vec![b'a']),
                    negated: false,
                    suffixes: vec![b"foo".into()],
                })
            )))
        );

        // member
        parse_success!(
            "!(ANY a not in ['foo', 'bar'])",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Member(
                Box::new(MemberMatcher {
                    quantifier: Quantifier::Any,
                    codes: SmallVec::from(vec![b'a']),
                    negated: true,
                    values: vec![b"foo".into(), b"bar".into()],
                })
            )))
        );

        // substr
        parse_success!(
            "!(ANY a =? 'foo')",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Substr(
                Box::new(SubstrMatcher {
                    quantifier: Quantifier::Any,
                    codes: SmallVec::from(vec![b'a']),
                    negated: false,
                    patterns: vec![b"foo".into()],
                    ac: AhoCorasick::new([b"foo"]).unwrap(),
                })
            )))
        );

        // regex
        parse_success!(
            "!(a =~ '^foo')",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Regex(
                Box::new(RegexMatcher {
                    quantifier: Quantifier::Any,
                    codes: SmallVec::from(vec![b'a']),
                    negated: false,
                    patterns: vec![b"^foo".into()],
                    matcher: RegexSet::new(vec!["^foo"]).unwrap(),
                })
            )))
        );

        // strsim
        parse_success!(
            "!(a =* 'foo')",
            SubfieldMatcher::Group(Box::new(
                SubfieldMatcher::Similarity(Box::new(
                    SimilarityMatcher {
                        quantifier: Quantifier::Any,
                        codes: SmallVec::from(vec![b'a']),
                        negated: false,
                        patterns: vec!["foo".into()],
                    }
                ))
            ))
        );
    }

    #[test]
    fn test_parse_boolean_connective() {
        macro_rules! parse_success {
            ($i:expr, $lhs:expr, $op:expr, $rhs:expr) => {
                assert_eq!(
                    parse_boolean_connective
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Composite {
                        lhs: Box::new($lhs),
                        op: $op,
                        rhs: Box::new($rhs),
                    }
                );
            };
        }

        parse_success!(
            "a? || b?",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            BooleanOp::Or,
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'b']),
                negated: false
            }))
        );

        parse_success!(
            "a? && b?",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            BooleanOp::And,
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'b']),
                negated: false
            }))
        );

        parse_success!(
            "b == 'p' && 2 == 'gndgen'",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'b']),
                operator: ComparisonOperator::Eq,
                value: "p".into(),
            })),
            BooleanOp::And,
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'2']),
                operator: ComparisonOperator::Eq,
                value: "gndgen".into(),
            }))
        );
    }

    #[test]
    fn test_parse_boolean_connective_or() {
        macro_rules! parse_success {
            ($i:expr, $lhs:expr, $rhs:expr) => {
                assert_eq!(
                    parse_boolean_connective_or
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Composite {
                        lhs: Box::new($lhs),
                        op: BooleanOp::Or,
                        rhs: Box::new($rhs),
                    }
                );
            };
        }

        parse_success!(
            "a? || b?",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'b']),
                negated: false
            }))
        );

        parse_success!(
            "a? || b? || !c?",
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                ))),
                op: BooleanOp::Or,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                ))),
            },
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'c']),
                negated: true
            }))
        );

        parse_success!(
            "a? && b? || c?",
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                ))),
                op: BooleanOp::And,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                ))),
            },
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'c']),
                negated: false
            }))
        );

        parse_success!(
            "a? || b? && c?",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                ))),
                op: BooleanOp::And,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'c']),
                        negated: false
                    }
                ))),
            }
        );

        parse_success!(
            "(a?) || b?",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Exists(
                Box::new(ExistsMatcher {
                    codes: SmallVec::from(vec![b'a']),
                    negated: false
                })
            ))),
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'b']),
                negated: false
            }))
        );

        parse_success!(
            "a? || (b?)",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Exists(
                Box::new(ExistsMatcher {
                    codes: SmallVec::from(vec![b'b']),
                    negated: false
                })
            )))
        );

        parse_success!(
            "a? || #b < 3",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            SubfieldMatcher::Count(Box::new(CountMatcher {
                codes: SmallVec::from(vec![b'b']),
                operator: ComparisonOperator::Lt,
                value: 3
            }))
        );

        parse_success!(
            "a == 'foo' || a == 'bar'",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Eq,
                value: "foo".into(),
            })),
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Eq,
                value: "bar".into(),
            }))
        );

        parse_success!(
            "a =^ 'foo' || a =^ 'bar'",
            SubfieldMatcher::Prefix(Box::new(PrefixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                prefixes: vec!["foo".into()],
            })),
            SubfieldMatcher::Prefix(Box::new(PrefixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                prefixes: vec!["bar".into()],
            }))
        );

        // suffix
        parse_success!(
            "a =$ '.pdf' || a =$ '.PDF'",
            SubfieldMatcher::Suffix(Box::new(SuffixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                suffixes: vec![".pdf".into()],
            })),
            SubfieldMatcher::Suffix(Box::new(SuffixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                suffixes: vec![".PDF".into()],
            }))
        );

        // member
        parse_success!(
            "a in ['A', 'B'] || b not in ['C', 'D']",
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                values: vec!["A".into(), "B".into()],
            })),
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'b']),
                negated: true,
                values: vec!["C".into(), "D".into()],
            }))
        );

        // substr
        parse_success!(
            "a =? 'foo' || a =? 'bar'",
            SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"foo".into()],
                ac: AhoCorasick::new([b"foo"]).unwrap(),
            })),
            SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"bar".into()],
                ac: AhoCorasick::new([b"bar"]).unwrap(),
            }))
        );

        // regex
        parse_success!(
            "a =~ '^foo' || ALL b !~ 'bar$'",
            SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"^foo".into()],
                matcher: RegexSet::new(["^foo"]).unwrap(),
            })),
            SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier: Quantifier::All,
                codes: SmallVec::from(vec![b'b']),
                negated: true,
                patterns: vec![b"bar$".into()],
                matcher: RegexSet::new(["bar$"]).unwrap(),
            }))
        );

        // strsim
        parse_success!(
            "a =* 'foo' || ALL b !* ['bar', 'baz',]",
            SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec!["foo".into()],
            })),
            SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
                quantifier: Quantifier::All,
                codes: SmallVec::from(vec![b'b']),
                negated: true,
                patterns: vec!["bar".into(), "baz".into()],
            }))
        );
    }

    #[test]
    fn test_parse_boolean_connective_and() {
        macro_rules! parse_success {
            ($i:expr, $lhs:expr, $rhs:expr) => {
                assert_eq!(
                    parse_boolean_connective_and
                        .parse($i.as_bytes())
                        .unwrap(),
                    SubfieldMatcher::Composite {
                        lhs: Box::new($lhs),
                        op: BooleanOp::And,
                        rhs: Box::new($rhs),
                    }
                );
            };
        }

        parse_success!(
            "a? && b?",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'b']),
                negated: false
            }))
        );

        parse_success!(
            "a? && b? && !c?",
            SubfieldMatcher::Composite {
                lhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'a']),
                        negated: false
                    }
                ))),
                op: BooleanOp::And,
                rhs: Box::new(SubfieldMatcher::Exists(Box::new(
                    ExistsMatcher {
                        codes: SmallVec::from(vec![b'b']),
                        negated: false
                    }
                ))),
            },
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'c']),
                negated: true
            }))
        );

        parse_success!(
            "(a?) && b?",
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Exists(
                Box::new(ExistsMatcher {
                    codes: SmallVec::from(vec![b'a']),
                    negated: false
                })
            ))),
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'b']),
                negated: false
            }))
        );

        parse_success!(
            "a? && (b?)",
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'a']),
                negated: false
            })),
            SubfieldMatcher::Group(Box::new(SubfieldMatcher::Exists(
                Box::new(ExistsMatcher {
                    codes: SmallVec::from(vec![b'b']),
                    negated: false
                })
            )))
        );

        parse_success!(
            "#a != 12 && b?",
            SubfieldMatcher::Count(Box::new(CountMatcher {
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Ne,
                value: 12,
            })),
            SubfieldMatcher::Exists(Box::new(ExistsMatcher {
                codes: SmallVec::from(vec![b'b']),
                negated: false
            }))
        );

        parse_success!(
            "a == 'foo' && b == 'bar'",
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                operator: ComparisonOperator::Eq,
                value: "foo".into(),
            })),
            SubfieldMatcher::Comparison(Box::new(ComparisonMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'b']),
                operator: ComparisonOperator::Eq,
                value: "bar".into(),
            }))
        );

        parse_success!(
            "a =^ 'foo' && a =^ 'bar'",
            SubfieldMatcher::Prefix(Box::new(PrefixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                prefixes: vec!["foo".into()],
            })),
            SubfieldMatcher::Prefix(Box::new(PrefixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                prefixes: vec!["bar".into()],
            }))
        );

        // suffix
        parse_success!(
            "a =$ '.pdf' && a =$ '.PDF'",
            SubfieldMatcher::Suffix(Box::new(SuffixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                suffixes: vec![".pdf".into()],
            })),
            SubfieldMatcher::Suffix(Box::new(SuffixMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                suffixes: vec![".PDF".into()],
            }))
        );

        // member
        parse_success!(
            "a in ['A', 'B'] && b not in ['C', 'D']",
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                values: vec!["A".into(), "B".into()],
            })),
            SubfieldMatcher::Member(Box::new(MemberMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'b']),
                negated: true,
                values: vec!["C".into(), "D".into()],
            }))
        );

        // substr
        parse_success!(
            "a =? 'foo' && a =? 'bar'",
            SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"foo".into()],
                ac: AhoCorasick::new([b"foo"]).unwrap(),
            })),
            SubfieldMatcher::Substr(Box::new(SubstrMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"bar".into()],
                ac: AhoCorasick::new([b"bar"]).unwrap(),
            }))
        );

        // regex
        parse_success!(
            "a =~ '^foo' && ALL b !~ 'bar$'",
            SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec![b"^foo".into()],
                matcher: RegexSet::new(["^foo"]).unwrap(),
            })),
            SubfieldMatcher::Regex(Box::new(RegexMatcher {
                quantifier: Quantifier::All,
                codes: SmallVec::from(vec![b'b']),
                negated: true,
                patterns: vec![b"bar$".into()],
                matcher: RegexSet::new(["bar$"]).unwrap(),
            }))
        );

        // strsim
        parse_success!(
            "a =* 'foo' && ALL b !* ['bar', 'baz',]",
            SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
                quantifier: Quantifier::Any,
                codes: SmallVec::from(vec![b'a']),
                negated: false,
                patterns: vec!["foo".into()],
            })),
            SubfieldMatcher::Similarity(Box::new(SimilarityMatcher {
                quantifier: Quantifier::All,
                codes: SmallVec::from(vec![b'b']),
                negated: true,
                patterns: vec!["bar".into(), "baz".into()],
            }))
        );
    }
}
