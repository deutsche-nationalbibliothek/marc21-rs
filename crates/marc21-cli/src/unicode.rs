use clap::ValueEnum;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub(crate) enum NormalizationForm {
    Nfd,
    Nfkd,
    Nfc,
    Nfkc,
}
