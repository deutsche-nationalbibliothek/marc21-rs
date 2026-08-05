use std::borrow::Cow;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use unicode_normalization::UnicodeNormalization;

#[derive(
    Debug, PartialEq, Clone, ValueEnum, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum NormalizationForm {
    Nfd,
    Nfkd,
    Nfc,
    Nfkc,
}

pub(crate) fn translit<'a>(
    s: &'a str,
    nf: &Option<NormalizationForm>,
) -> Cow<'a, str> {
    match nf {
        Some(NormalizationForm::Nfc) => Cow::Owned(s.nfc().collect()),
        Some(NormalizationForm::Nfd) => Cow::Owned(s.nfd().collect()),
        Some(NormalizationForm::Nfkc) => Cow::Owned(s.nfkc().collect()),
        Some(NormalizationForm::Nfkd) => Cow::Owned(s.nfkd().collect()),
        None => Cow::Borrowed(s),
    }
}
