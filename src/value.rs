use std::borrow::Cow;

use bstr::ByteSlice;

#[derive(Debug, PartialEq, Default)]
pub struct Value<'a>(Cow<'a, [u8]>);

impl<'a> Value<'a> {
    pub fn to_str_lossy(&self) -> Cow<'_, str> {
        self.0.to_str_lossy()
    }
}

impl From<String> for Value<'_> {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value.into_bytes()))
    }
}

impl<'a> From<&'a [u8]> for Value<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl<T> PartialEq<T> for Value<'_>
where
    T: AsRef<[u8]>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == other.as_ref()
    }
}
