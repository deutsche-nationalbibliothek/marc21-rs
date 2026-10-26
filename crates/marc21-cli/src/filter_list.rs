use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use flate2::read::GzDecoder;
use marc21::{ByteRecord, Path};
use unicode_normalization::UnicodeNormalization;

use crate::cli::FilterOpts;
use crate::error::CliError;
use crate::unicode::NormalizationForm;

#[derive(Debug, Default)]
pub(crate) struct FilterListBuilder {
    allow: Vec<PathBuf>,
    deny: Vec<PathBuf>,
    normalization_form: Option<NormalizationForm>,
    source: String,
    column: String,
}

impl FilterListBuilder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn with_normalization(
        mut self,
        form: &Option<NormalizationForm>,
    ) -> Self {
        self.normalization_form = form.clone();
        self
    }

    pub(crate) fn with_source(mut self, source: String) -> Self {
        self.source = source;
        self
    }

    pub(crate) fn with_column(
        mut self,
        column: Option<String>,
    ) -> Self {
        self.column = column.unwrap_or_default();
        self
    }

    pub(crate) fn with_allow(mut self, paths: Vec<PathBuf>) -> Self {
        self.allow = paths;
        self
    }

    pub(crate) fn with_deny(mut self, paths: Vec<PathBuf>) -> Self {
        self.deny = paths;
        self
    }

    pub(crate) fn build(self) -> Result<FilterList, CliError> {
        use NormalizationForm::*;

        let source = Path::from_bytes(match self.normalization_form {
            Some(Nfc) => self.source.nfc().collect(),
            Some(Nfkc) => self.source.nfkc().collect(),
            Some(Nfd) => self.source.nfd().collect(),
            Some(Nfkd) => self.source.nfkd().collect(),
            None => self.source,
        })?;

        Ok(FilterList {
            source,
            allow: read_filter_list(&self.allow, &self.column)?,
            deny: read_filter_list(&self.deny, &self.column)?,
        })
    }
}

impl TryFrom<&FilterOpts> for FilterList {
    type Error = CliError;

    fn try_from(opts: &FilterOpts) -> Result<Self, Self::Error> {
        FilterListBuilder::new()
            .with_normalization(&opts.query_normalization)
            .with_source(opts.filter_list_source.clone())
            .with_column(opts.filter_list_column.clone())
            .with_allow(opts.allow.clone())
            .with_deny(opts.deny.clone())
            .build()
    }
}

fn read_filter_list<P>(
    paths: &[P],
    column: &str,
) -> Result<Option<BTreeSet<Vec<u8>>>, CliError>
where
    P: AsRef<std::path::Path>,
{
    if paths.is_empty() {
        return Ok(None);
    }

    let mut list = BTreeSet::new();

    for path in paths {
        let path = path.as_ref();
        let path_str = path.to_str().unwrap_or_default();
        let mut has_headers = true;
        let mut index = 0;

        let inner: Box<dyn Read> = if path_str.ends_with(".gz") {
            Box::new(GzDecoder::new(File::open(path)?))
        } else {
            Box::new(BufReader::new(File::open(path)?))
        };

        if path_str.ends_with(".txt") || path_str.ends_with(".txt.gz") {
            has_headers = false;
        }

        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(inner);

        if has_headers {
            let mut found = false;

            for (i, name) in rdr.byte_headers()?.iter().enumerate() {
                if name == column.as_bytes() {
                    index = i;
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(CliError::AdHoc(format!(
                    "could not find column with name '{column}' in filter list {path:?}"
                )));
            }
        }

        for result in rdr.byte_records() {
            let record = result?;

            if let Some(value) = record.get(index) {
                let _ = list.insert(value.to_vec());
            } else {
                return Err(CliError::AdHoc(format!(
                    "expected column {index} in filter list"
                )));
            }
        }
    }

    Ok(Some(list))
}

#[derive(Debug)]
pub(crate) struct FilterList {
    source: Path,
    allow: Option<BTreeSet<Vec<u8>>>,
    deny: Option<BTreeSet<Vec<u8>>>,
}

impl FilterList {
    pub(crate) fn check(&self, record: &ByteRecord) -> bool {
        let values: Vec<_> =
            record.path(&self.source, &Default::default());

        if let Some(ref deny_list) = self.deny
            && (values.iter().any(|value| {
                deny_list.contains(AsRef::<[u8]>::as_ref(value))
            }))
        {
            return false;
        }

        if let Some(ref allow_list) = self.allow
            && (allow_list.is_empty()
                || !values.iter().any(|value| {
                    allow_list.contains(AsRef::<[u8]>::as_ref(value))
                }))
        {
            return false;
        }

        true
    }
}
