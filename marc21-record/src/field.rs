use std::fmt::{self, Display};
use std::str::Utf8Error;

use bstr::BStr;

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
    pub(crate) value: &'a BStr,
}

impl<'a> ControlField<'a> {
    pub fn tag(&self) -> &Tag<'a> {
        &self.tag
    }

    pub fn value(&self) -> &'a BStr {
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
        write!(f, "{} {}", self.tag, self.value)
    }
}

#[derive(Debug, PartialEq)]
pub struct DataField<'a> {
    pub(crate) tag: Tag<'a>,
    pub(crate) indicator1: char,
    pub(crate) indicator2: char,
    pub(crate) subfields: Vec<Subfield<'a>>,
}

impl<'a> DataField<'a> {
    pub fn tag(&self) -> &Tag<'a> {
        &self.tag
    }

    pub fn subfields(&self) -> impl Iterator<Item = &Subfield<'a>> {
        self.subfields.iter()
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
        let ind1 = if self.indicator1 != ' ' {
            self.indicator1
        } else {
            '#'
        };

        let ind2 = if self.indicator2 != ' ' {
            self.indicator2
        } else {
            '#'
        };

        write!(f, "{} {ind1}{ind2}", self.tag)?;

        for subfield in self.subfields.iter() {
            write!(f, " {subfield}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;

    use super::*;
    use crate::parse::TestResult;

    #[test]
    fn test_control_field_to_string() -> TestResult {
        let cf = ControlField {
            tag: Tag::from_bytes(b"001")?,
            value: b"abc".as_bstr(),
        };

        assert_eq!(cf.to_string(), "001 abc");
        Ok(())
    }

    #[test]
    fn test_data_field_to_string() -> TestResult {
        let df = DataField {
            tag: Tag::from_bytes(b"024")?,
            indicator1: '7',
            indicator2: ' ',
            subfields: vec![
                Subfield {
                    code: 'a',
                    value: b"119232022".as_bstr(),
                },
                Subfield {
                    code: '2',
                    value: b"gnd".as_bstr(),
                },
            ],
        };

        assert_eq!(df.to_string(), "024 7# $a 119232022 $2 gnd");
        Ok(())
    }
}
