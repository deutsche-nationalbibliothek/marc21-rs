mod error;
mod parse;

use std::borrow::Cow;
use std::fmt::{self, Display};
use std::str::FromStr;

use bstr::ByteSlice;
pub use error::ParsePathError;
use parse::parse_path;
use winnow::Parser;

use crate::matcher::leader::LeaderField;
use crate::matcher::{
    IndicatorMatcher, MatchOptions, SubfieldMatcher, TagMatcher,
};
use crate::{ByteRecord, ControlField, Field};

#[derive(Debug, PartialEq)]
pub struct Path {
    kind: PathKind,
    input: Vec<u8>,
}

impl Path {
    /// Creates a new path from a byte slice.
    ///
    /// # Errors
    ///
    /// This function returns an error if the given byte slice is not a
    /// valid path expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    ///
    /// let _path = Path::from_bytes("ldr.length")?;
    /// let _path = Path::from_bytes("001")?;
    /// let _path = Path::from_bytes("005[0:4]")?;
    /// let _path = Path::from_bytes("075{ _ | 2 == 'gndspec' }")?;
    /// let _path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes<B>(bytes: B) -> Result<Self, ParsePathError>
    where
        B: AsRef<[u8]>,
    {
        parse_path
            .parse(bytes.as_ref())
            .map_err(ParsePathError::from_parse)
    }

    /// Returns the arity of the path expression.
    ///
    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    ///
    /// let path = Path::from_bytes("ldr.length")?;
    /// assert_eq!(path.arity(), 1);
    ///
    /// let path = Path::from_bytes("005[0:8]")?;
    /// assert_eq!(path.arity(), 1);
    ///
    /// let path = Path::from_bytes("075{ _ | 2 == 'gndspec' }")?;
    /// assert_eq!(path.arity(), 0);
    ///
    /// let path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    /// assert_eq!(path.arity(), 1);
    ///
    /// let path = Path::from_bytes("400/1#{ a,c }")?;
    /// assert_eq!(path.arity(), 2);
    ///
    /// let path = Path::from_bytes("400/1#{ [ab],c }")?;
    /// assert_eq!(path.arity(), 2);
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn arity(&self) -> usize {
        match self.kind {
            PathKind::Leader(_) => 1,
            PathKind::ControlField(_) => 1,
            PathKind::DataField(ref df) => df.codes.len(),
            PathKind::Empty(_) => 0,
        }
    }

    /// # Example
    ///
    /// ```rust
    /// use marc21::Path;
    /// use marc21::prelude::*;
    ///
    /// let data = include_bytes!("../../tests/data/ada.mrc");
    /// let record = ByteRecord::from_bytes(data)?;
    ///
    /// let path = Path::from_bytes("ldr.length")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["3612".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("075.b")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["p".as_bytes(), "piz".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("075{ b }")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["p".as_bytes(), "piz".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("075{ b | 2 == 'gndspec' }")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["piz".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("001")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["119232022".as_bytes()]
    /// );
    ///
    /// let path = Path::from_bytes("005[:4]")?;
    /// assert_eq!(
    ///     path.project(&record, &Default::default()),
    ///     vec!["2025".as_bytes()]
    /// );
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Cow<'a, [u8]>> {
        use PathKind::*;

        match self.kind {
            Leader(ref path) => path.project(record, options),
            ControlField(ref path) => path.project(record, options),
            DataField(ref path) => path.project(record, options),
            Empty(_) => vec![],
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.input.to_str_lossy())
    }
}

impl FromStr for Path {
    type Err = ParsePathError;

    /// # Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use marc21::Path;
    ///
    /// let _path = Path::from_str("ldr.length")?;
    ///
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}

#[derive(Debug, PartialEq)]
enum PathKind {
    Leader(LeaderPath),
    ControlField(ControlFieldPath),
    DataField(DataFieldPath),
    Empty(EmptyPath),
}

#[derive(Debug, PartialEq)]
struct LeaderPath {
    field: LeaderField,
}

impl LeaderPath {
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        _options: &MatchOptions,
    ) -> Vec<Cow<'a, [u8]>> {
        let ldr = record.leader();
        let value = match self.field {
            LeaderField::BaseAddr => ldr.base_addr().to_string(),
            LeaderField::Encoding => ldr.encoding().to_string(),
            LeaderField::Length => ldr.length().to_string(),
            LeaderField::Status => ldr.status().to_string(),
            LeaderField::Type => ldr.r#type().to_string(),
        };

        vec![Cow::Owned(value.into_bytes())]
    }
}

#[derive(Debug, PartialEq)]
struct ControlFieldPath {
    tag_matcher: TagMatcher,
    range: Option<(Option<usize>, Option<usize>)>,
}

impl ControlFieldPath {
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        _options: &MatchOptions,
    ) -> Vec<Cow<'a, [u8]>> {
        let mut iter = record.fields();
        let mut values = vec![];

        while let Some(Field::Control(ControlField { tag, value })) =
            iter.next()
        {
            if !self.tag_matcher.is_match(tag) {
                continue;
            }

            let value: &[u8] = if let Some(range) = self.range {
                match range {
                    (Some(start), Some(end)) => {
                        value.get(start..end).unwrap_or_default()
                    }
                    (Some(start), None) => value
                        .get(start..value.len())
                        .unwrap_or_default(),
                    (None, Some(end)) => {
                        value.get(0..end).unwrap_or_default()
                    }
                    _ => unreachable!(),
                }
            } else {
                value
            };

            values.push(Cow::Borrowed(value));
        }

        values
    }
}

#[derive(Debug, PartialEq)]
struct DataFieldPath {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    codes: Vec<Vec<u8>>,
    subfield_matcher: Option<SubfieldMatcher>,
}

impl DataFieldPath {
    pub fn project<'a>(
        &self,
        record: &ByteRecord<'a>,
        options: &MatchOptions,
    ) -> Vec<Cow<'a, [u8]>> {
        let mut values = vec![];

        let fields = record
            .fields()
            .filter(|field| self.tag_matcher.is_match(field.tag()))
            .filter(|field| self.indicator_matcher.is_match(field))
            .filter_map(|field| match field {
                Field::Data(df) => Some(df),
                _ => None,
            })
            .filter(|field| {
                if let Some(ref matcher) = self.subfield_matcher {
                    matcher.is_match(field.subfields(), options)
                } else {
                    true
                }
            });

        for field in fields {
            for subfield in field.subfields() {
                if self
                    .codes
                    .iter()
                    .any(|codes| codes.contains(subfield.code()))
                {
                    values.push(Cow::Borrowed(subfield.value()));
                }
            }
        }

        values
    }
}

#[derive(Debug, PartialEq)]
struct EmptyPath {
    tag_matcher: TagMatcher,
    indicator_matcher: IndicatorMatcher,
    subfield_matcher: SubfieldMatcher,
}
