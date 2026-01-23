use std::fmt::{self, Display};
use std::str::Utf8Error;

use bstr::ByteSlice;

use crate::{Subfield, Tag};

#[derive(Debug, PartialEq)]
pub enum Field<'a> {
    Control(ControlField<'a>),
    Data(DataField<'a>),
}

impl<'a> Field<'a> {
    /// Returns an [`std::str::Utf8Error`](Utf8Error) if the field
    /// contains invalid UTF-8 data, otherwise the unit.
    pub fn validate(&self) -> Result<(), Utf8Error> {
        match self {
            Self::Control(cf) => cf.validate(),
            Self::Data(df) => df.validate(),
        }
    }
}

impl Display for Field<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Control(cf) => write!(f, "{cf}"),
            Self::Data(df) => write!(f, "{df}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ControlField<'a> {
    pub(crate) tag: Tag<'a>,
    pub(crate) value: &'a [u8],
}

impl<'a> ControlField<'a> {
    pub fn tag(&self) -> &Tag<'a> {
        &self.tag
    }

    pub fn value(&self) -> &'a [u8] {
        self.value
    }

    /// Returns an [`std::str::Utf8Error`](Utf8Error) if the field
    /// contains invalid UTF-8 data, otherwise the unit.
    pub fn validate(&self) -> Result<(), Utf8Error> {
        if self.value.is_ascii() {
            return Ok(());
        };

        let _ = std::str::from_utf8(self.value)?;
        Ok(())
    }
}

impl Display for ControlField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.tag, self.value.as_bstr())
    }
}

#[derive(Debug, PartialEq)]
pub struct DataField<'a> {
    pub(crate) tag: Tag<'a>,
    pub(crate) indicator1: u8,
    pub(crate) indicator2: u8,
    pub(crate) subfields: Vec<Subfield<'a>>,
}

impl<'a> DataField<'a> {
    #[inline(always)]
    pub fn tag(&self) -> &Tag<'a> {
        &self.tag
    }

    #[inline(always)]
    pub fn subfields(&self) -> impl Iterator<Item = &Subfield<'a>> {
        self.subfields.iter()
    }

    #[inline(always)]
    pub fn indicator1(&self) -> &u8 {
        &self.indicator1
    }

    #[inline(always)]
    pub fn indicator2(&self) -> &u8 {
        &self.indicator2
    }

    /// Returns an [`std::str::Utf8Error`](Utf8Error) if the field
    /// contains invalid UTF-8 data, otherwise the unit.
    pub fn validate(&self) -> Result<(), Utf8Error> {
        for subfield in self.subfields() {
            subfield.validate()?;
        }

        Ok(())
    }
}

impl Display for DataField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.tag)?;

        if self.indicator1 != b' ' || self.indicator2 != b' ' {
            let ind1 = if self.indicator1 != b' ' {
                self.indicator1
            } else {
                b'#'
            };

            let ind2 = if self.indicator2 != b' ' {
                self.indicator2
            } else {
                b'#'
            };

            write!(f, "/{}{}", ind1 as char, ind2 as char)?;
        }

        for subfield in self.subfields.iter() {
            write!(f, " {subfield}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::TestResult;

    #[test]
    fn test_control_field_to_string() -> TestResult {
        let cf = ControlField {
            tag: Tag::from_bytes(b"001")?,
            value: b"abc",
        };

        assert_eq!(cf.to_string(), "001 abc");
        Ok(())
    }

    #[test]
    fn test_data_field_to_string() -> TestResult {
        let df = DataField {
            tag: Tag::from_bytes(b"024")?,
            indicator1: b'7',
            indicator2: b' ',
            subfields: vec![
                Subfield {
                    code: 'a',
                    value: b"119232022",
                },
                Subfield {
                    code: '2',
                    value: b"gnd",
                },
            ],
        };

        assert_eq!(df.to_string(), "024/7# $a 119232022 $2 gnd");
        Ok(())
    }
}
