use winnow::ascii::{multispace0, multispace1};
use winnow::combinator::{
    alt, delimited, opt, preceded, seq, terminated,
};
use winnow::prelude::*;

use crate::Field;
use crate::matcher::indicator_matcher::parse_indicator_matcher_opt;
use crate::matcher::operator::{
    ComparisonOperator, parse_comparison_operator,
};
use crate::matcher::quantifier::{Quantifier, parse_quantifier_opt};
use crate::matcher::subfield_matcher::parse_subfield_matcher_short_form;
use crate::matcher::tag_matcher::parse_tag_matcher;
use crate::matcher::utils::{parse_usize, ws};
use crate::matcher::{
    IndicatorMatcher, MatchOptions, ParseMatcherError, TagMatcher,
    control_field_matcher, subfield_matcher,
};

// /// A matcher that can be applied on a list of [Field]s.
#[derive(Debug, PartialEq, Clone)]
pub struct FieldMatcher {
    kind: MatcherKind,
}

impl FieldMatcher {
    /// Parse a field matcher from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::FieldMatcher;
    ///
    /// let matcher = FieldMatcher::new("001?")?;
    /// let matcher = FieldMatcher::new("001/12?")?;
    /// let matcher = FieldMatcher::new("5[01]0?")?;
    /// let matcher = FieldMatcher::new("5[01]0/*?")?;
    /// let matcher = FieldMatcher::new("450/*?")?;
    ///
    /// let matcher = FieldMatcher::new("#400/* == 13")?;
    /// let matcher = FieldMatcher::new("#035 < 6")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_field_matcher
            .parse(matcher.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if and only if the given field(s) match against the
    /// underlying matcher.
    ///
    /// # Example
    ///
    ///
    /// ```rust
    /// use marc21::matcher::FieldMatcher;
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    ///
    /// let matcher = FieldMatcher::new("100/1#?")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// let matcher = FieldMatcher::new("#400/* == 13")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// let matcher = FieldMatcher::new("#035 <= 6")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// let matcher = FieldMatcher::new("042.a == 'gnd1'")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        match self.kind {
            MatcherKind::Subfield(ref m) => m.is_match(fields, options),
            MatcherKind::Control(ref m) => m.is_match(fields, options),
            MatcherKind::Exists(ref m) => m.is_match(fields, options),
            MatcherKind::Count(ref m) => m.is_match(fields, options),
        }
    }
}

pub(crate) fn parse_field_matcher(
    i: &mut &[u8],
) -> ModalResult<FieldMatcher> {
    alt((
        parse_subfield_matcher.map(MatcherKind::Subfield),
        parse_control_field_matcher.map(MatcherKind::Control),
        parse_exists_matcher.map(MatcherKind::Exists),
        parse_count_matcher.map(MatcherKind::Count),
    ))
    .map(|kind| FieldMatcher { kind })
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
enum MatcherKind {
    Subfield(SubfieldMatcher),
    Control(ControlFieldMatcher),
    Exists(ExistsMatcher),
    Count(CountMatcher),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SubfieldMatcher {
    quantifier: Quantifier,
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    subfield_matcher: subfield_matcher::SubfieldMatcher,
}

impl SubfieldMatcher {
    /// Returns true if and only if the fields matches this given
    /// criteria.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::{FieldMatcher, MatchOptions};
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let options = MatchOptions::default();
    ///
    /// let matcher = FieldMatcher::new("042.a == 'gnd1'")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        let mut fields = fields.into_iter().filter(|field| {
            field.is_data_field()
                && self.tag_matcher.is_match(field.tag())
                && self.indicator_matcher.is_match(field)
        });

        let r#fn = |field: &Field| -> bool {
            match field {
                Field::Control(_) => unreachable!(),
                Field::Data(df) => self
                    .subfield_matcher
                    .is_match(df.subfields(), options),
            }
        };

        match self.quantifier {
            Quantifier::All => fields.all(r#fn),
            Quantifier::Any => fields.any(r#fn),
        }
    }
}

fn parse_subfield_matcher(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    alt((parse_subfield_matcher_short, parse_subfield_matcher_long))
        .parse_next(i)
}

fn parse_subfield_matcher_short(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    ws(seq! { SubfieldMatcher {
         quantifier: parse_quantifier_opt,
         tag_matcher: parse_tag_matcher,
         indicator_matcher: parse_indicator_matcher_opt,
         subfield_matcher: preceded('.', parse_subfield_matcher_short_form),
    }}).parse_next(i)
}

fn parse_subfield_matcher_long(
    i: &mut &[u8],
) -> ModalResult<SubfieldMatcher> {
    ws(seq! { SubfieldMatcher {
         quantifier: parse_quantifier_opt,
         tag_matcher: parse_tag_matcher,
         indicator_matcher: parse_indicator_matcher_opt,
         subfield_matcher: delimited(
            terminated('{', multispace0),
            subfield_matcher::parse_subfield_matcher,
            preceded(multispace0, '}')
        ),
    }})
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ControlFieldMatcher {
    quantifier: Quantifier,
    tag_matcher: TagMatcher,
    matcher: control_field_matcher::ControlFieldMatcher,
}

impl ControlFieldMatcher {
    /// Returns true if and only if the given control field(s) match
    /// against the underlying matcher.
    ///
    /// # Example
    ///
    ///
    /// ```rust
    /// use marc21::matcher::FieldMatcher;
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    ///
    /// let matcher = FieldMatcher::new("100/1#?")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// let matcher = FieldMatcher::new("#400/* == 13")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// let matcher = FieldMatcher::new("#035 <= 6")?;
    /// assert!(matcher.is_match(record.fields(), &Default::default()));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        options: &MatchOptions,
    ) -> bool {
        let mut fields = fields.into_iter().filter(|field| {
            field.is_control_field()
                && self.tag_matcher.is_match(field.tag())
        });

        match self.quantifier {
            Quantifier::All => fields
                .all(|field| self.matcher.is_match(field, options)),
            Quantifier::Any => fields
                .any(|field| self.matcher.is_match(field, options)),
        }
    }
}

fn parse_control_field_matcher(
    i: &mut &[u8],
) -> ModalResult<ControlFieldMatcher> {
    seq! { ControlFieldMatcher {
        quantifier: parse_quantifier_opt,
        tag_matcher: terminated(parse_tag_matcher, multispace1),
        matcher: control_field_matcher::parse_control_field_matcher,
    }}
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
struct ExistsMatcher {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    negated: bool,
}

impl ExistsMatcher {
    /// Returns true if and only if a field exists that matches the
    /// matcher criteria.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::{FieldMatcher, MatchOptions};
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let options = MatchOptions::default();
    ///
    /// let matcher = FieldMatcher::new("001?")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("100/1#?")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("!555/*?")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        _options: &MatchOptions,
    ) -> bool {
        fields.into_iter().any(|field| {
            let result = self.tag_matcher.is_match(field.tag())
                && self.indicator_matcher.is_match(field);

            if self.negated { !result } else { result }
        })
    }
}

fn parse_exists_matcher(i: &mut &[u8]) -> ModalResult<ExistsMatcher> {
    ws(terminated(
        seq! { ExistsMatcher {
            negated:  opt('!').map(|value| value.is_some()),
            tag_matcher: parse_tag_matcher,
            indicator_matcher:  parse_indicator_matcher_opt,
        }},
        '?',
    ))
    .parse_next(i)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CountMatcher {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    comparison_op: ComparisonOperator,
    count: usize,
}

impl CountMatcher {
    /// Returns true if and only if the number of fields that matches
    /// the matcher criteria is equal to the comparative value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::{FieldMatcher, MatchOptions};
    /// use marc21::prelude::*;
    ///
    /// # let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    /// let options = MatchOptions::default();
    ///
    /// let matcher = FieldMatcher::new("#400/* == 13")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// let matcher = FieldMatcher::new("#035 <= 6")?;
    /// assert!(matcher.is_match(record.fields(), &options));
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn is_match<'a, F: Iterator<Item = &'a Field<'a>>>(
        &self,
        fields: F,
        _options: &MatchOptions,
    ) -> bool {
        let count = fields
            .into_iter()
            .filter(|field| {
                self.tag_matcher.is_match(field.tag())
                    && self.indicator_matcher.is_match(field)
            })
            .count();

        match self.comparison_op {
            ComparisonOperator::Eq => count == self.count,
            ComparisonOperator::Ne => count != self.count,
            ComparisonOperator::Ge => count >= self.count,
            ComparisonOperator::Gt => count > self.count,
            ComparisonOperator::Le => count <= self.count,
            ComparisonOperator::Lt => count < self.count,
        }
    }
}

fn parse_count_matcher(i: &mut &[u8]) -> ModalResult<CountMatcher> {
    ws(preceded(
        '#',
        seq! { CountMatcher {
            tag_matcher: parse_tag_matcher,
            indicator_matcher: parse_indicator_matcher_opt,
            comparison_op: ws(parse_comparison_operator),
            count: parse_usize,
        }},
    ))
    .parse_next(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_parse_exists_matcher() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_exists_matcher.parse($i.as_bytes()).unwrap(),
                    $o
                )
            };
        }

        parse_success!(
            "001?",
            ExistsMatcher {
                tag_matcher: TagMatcher::new("001")?,
                indicator_matcher: IndicatorMatcher::None,
                negated: false
            }
        );

        parse_success!(
            "!001?",
            ExistsMatcher {
                tag_matcher: TagMatcher::new("001")?,
                indicator_matcher: IndicatorMatcher::None,
                negated: true
            }
        );

        parse_success!(
            "400/*?",
            ExistsMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::Wildcard,
                negated: false
            }
        );

        Ok(())
    }

    // #[test]
    // fn test_parse_field_matcher() -> TestResult {
    //     macro_rules! parse_success {
    //         ($i:expr, $o:expr) => {
    //             assert_eq!(
    //
    // parse_field_matcher.parse($i.as_bytes()).unwrap(),
    //                 $o
    //             )
    //         };
    //     }

    //     parse_success!(
    //         "400.[ab] == 'abc'",
    //         FieldMatcher::Subfield(SubfieldMatcher_ {
    //             quantifier: Quantifier::Any,
    //             tag_matcher: TagMatcher::new("400")?,
    //             indicator_matcher: IndicatorMatcher::None,
    //             subfield_matcher: SubfieldMatcher::new(
    //                 "[ab] == 'abc'"
    //             )?,
    //         })
    //     );

    //         parse_success!(
    //             "001?",
    //             FieldMatcher::Exists(ExistsMatcher {
    //                 tag_matcher: TagMatcher::new("001")?,
    //                 indicator_matcher: IndicatorMatcher::None,
    //                 negated: false,
    //             })
    //         );

    //         parse_success!(
    //             "#400 == 10",
    //             FieldMatcher::Count(CountMatcher {
    //                 tag_matcher: TagMatcher::new("400")?,
    //                 indicator_matcher: IndicatorMatcher::None,
    //                 comparison_op: ComparisonOperator::Eq,
    //                 value: 10usize
    //             })
    //         );

    //         Ok(())
    //     }

    //     #[test]
    //     fn test_parse_exists_matcher() -> TestResult {
    //         macro_rules! parse_success {
    //             ($i:expr, $o:expr) => {
    //                 assert_eq!(
    //
    // parse_exists_matcher.parse($i.as_bytes()).unwrap(),
    // $o                 )
    //             };
    //         }

    //         parse_success!(
    //             "001?",
    //             ExistsMatcher {
    //                 tag_matcher: TagMatcher::new("001")?,
    //                 indicator_matcher: IndicatorMatcher::None,
    //                 negated: false,
    //             }
    //         );

    //         parse_success!(
    //             "400/1#?",
    //             ExistsMatcher {
    //                 tag_matcher: TagMatcher::new("400")?,
    //                 indicator_matcher:
    // IndicatorMatcher::new("/1#")?,
    // negated: false,             }
    //         );

    //         parse_success!(
    //             "!400/1#?",
    //             ExistsMatcher {
    //                 tag_matcher: TagMatcher::new("400")?,
    //                 indicator_matcher:
    // IndicatorMatcher::new("/1#")?,
    // negated: true,             }
    //         );

    // Ok(())
    // }

    #[test]
    fn test_parse_count_matcher() -> TestResult {
        macro_rules! parse_success {
            ($i:expr, $o:expr) => {
                assert_eq!(
                    parse_count_matcher.parse($i.as_bytes()).unwrap(),
                    $o
                )
            };
        }

        parse_success!(
            "#400 == 10",
            CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::None,
                comparison_op: ComparisonOperator::Eq,
                count: 10usize
            }
        );

        parse_success!(
            "#400/* > 5",
            CountMatcher {
                tag_matcher: TagMatcher::new("400")?,
                indicator_matcher: IndicatorMatcher::Wildcard,
                comparison_op: ComparisonOperator::Gt,
                count: 5usize
            }
        );

        parse_success!(
            "#07[5-9]/* <= 4",
            CountMatcher {
                tag_matcher: TagMatcher::new("07[5-9]")?,
                indicator_matcher: IndicatorMatcher::Wildcard,
                comparison_op: ComparisonOperator::Le,
                count: 4usize
            }
        );

        parse_success!(
            "#100/[1-3]# < 2",
            CountMatcher {
                tag_matcher: TagMatcher::new("100")?,
                indicator_matcher: IndicatorMatcher::new("/[1-3]#")?,
                comparison_op: ComparisonOperator::Lt,
                count: 2usize
            }
        );

        Ok(())
    }
}
