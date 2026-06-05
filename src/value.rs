use std::borrow::Cow;

use bstr::ByteSlice;

#[derive(Debug, Clone, Eq, PartialOrd, Ord, Default)]
pub struct Value<'a>(Cow<'a, [u8]>);

impl<'a> Value<'a> {
    pub fn to_str_lossy(&self) -> Cow<'_, str> {
        self.0.to_str_lossy()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline(always)]
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

impl AsRef<[u8]> for Value<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.0
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

impl<'a> From<&'a [u8; 0]> for Value<'a> {
    fn from(value: &'a [u8; 0]) -> Self {
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
