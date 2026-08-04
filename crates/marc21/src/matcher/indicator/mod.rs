use winnow::Parser;

use crate::Field;
use crate::matcher::ParseMatcherError;
use crate::matcher::indicator::parse::parse_indicator_matcher;

pub(crate) mod parse;

/// A matcher that can be applied on indicators.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum IndicatorMatcher {
    Values(u8, u8),
    Pattern(Constituent, Constituent),
    Wildcard,
    #[default]
    None,
}

impl IndicatorMatcher {
    /// Parse a indicator matcher from a byte slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::matcher::IndicatorMatcher;
    ///
    /// let _matcher = IndicatorMatcher::new("/#1")?;
    /// let _matcher = IndicatorMatcher::new("/12")?;
    /// let _matcher = IndicatorMatcher::new("/1[23]")?;
    /// let _matcher = IndicatorMatcher::new("/1[2-5]")?;
    /// let _matcher = IndicatorMatcher::new("/2.")?;
    /// let _matcher = IndicatorMatcher::new("/*")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<B: AsRef<[u8]>>(
        matcher: B,
    ) -> Result<Self, ParseMatcherError> {
        parse_indicator_matcher
            .parse(matcher.as_ref())
            .map_err(ParseMatcherError::from_parse)
    }

    /// Returns true if the indicator matcher matches against the given
    /// field.
    pub fn is_match(&self, field: &Field) -> bool {
        match field {
            Field::Control(_) => {
                matches!(self, Self::None | Self::Wildcard)
            }
            Field::Data(df) => match self {
                Self::Values(ind1, ind2) => {
                    ind1 == df.indicator1() && ind2 == df.indicator2()
                }
                Self::Pattern(c1, c2) => {
                    *c1 == *df.indicator1() && *c2 == *df.indicator2()
                }
                Self::None => {
                    b' ' == *df.indicator1() && b' ' == *df.indicator2()
                }
                Self::Wildcard => true,
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constituent {
    Value(u8),
    Class(Vec<u8>),
    Any,
}

impl PartialEq<u8> for Constituent {
    fn eq(&self, other: &u8) -> bool {
        match self {
            Self::Value(value) => value == other,
            Self::Class(class) => class.contains(other),
            Self::Any => true,
        }
    }
}
