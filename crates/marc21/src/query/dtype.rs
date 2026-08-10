use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub enum DataType {
    UInt32,
    String,
    Char,
}

impl Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UInt32 => write!(f, "uint32"),
            Self::String => write!(f, "string"),
            Self::Char => write!(f, "char"),
        }
    }
}
