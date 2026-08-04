use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use marc21::matcher::RecordMatcher;
use serde::Deserialize;
use unicode_normalization::UnicodeNormalization;

use crate::commands::check::level::Level;
use crate::commands::check::record::Record;
use crate::commands::check::validator::Validator;
use crate::commands::check::writer::Writer;
use crate::prelude::*;
use crate::unicode::NormalizationForm::{self, Nfc, Nfd, Nfkc, Nfkd};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct RuleSet {
    pub(crate) scope: Option<RecordMatcher>,

    #[serde(rename = "rule", default)]
    pub(crate) rules: HashMap<String, Rule>,

    #[serde(skip)]
    path: PathBuf,
}

impl RuleSet {
    pub(crate) fn from_path<P>(
        path: P,
        nf: Option<&NormalizationForm>,
    ) -> Result<Self, CliError>
    where
        P: AsRef<Path>,
    {
        let content = match nf {
            Some(Nfc) => read_to_string(&path)?.nfc().collect(),
            Some(Nfkc) => read_to_string(&path)?.nfkc().collect(),
            Some(Nfd) => read_to_string(&path)?.nfd().collect(),
            Some(Nfkd) => read_to_string(&path)?.nfkd().collect(),
            None => read_to_string(&path)?,
        };

        let mut rs: Self = toml::from_str(&content)?;
        rs.path = path.as_ref().to_path_buf();

        for (id, rule) in rs.rules.iter_mut() {
            rule.id = id.to_string();
        }

        Ok(rs)
    }

    /// Returns true if the rule set contains no rules.
    #[inline(always)]
    pub(crate) fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    pub(crate) fn validate(
        &self,
        record: &ByteRecord,
        writer: &mut Writer,
    ) -> Result<(), CliError> {
        if let Some(ref matcher) = self.scope
            && !matcher.is_match(record, &MatchOptions::default())
        {
            return Ok(());
        }

        for rule in self.rules.values() {
            rule.validate(record, writer)?;
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Rule {
    #[serde(skip)]
    pub(crate) id: String,

    #[serde(default)]
    pub(crate) level: Level,

    #[serde(default)]
    message: String,

    #[allow(dead_code)]
    description: Option<String>,

    #[allow(dead_code)]
    link: Option<String>,

    #[serde(flatten)]
    pub(crate) validator: Validator,
}

impl Rule {
    pub(crate) fn validate(
        &self,
        record: &ByteRecord,
        writer: &mut Writer,
    ) -> Result<(), CliError> {
        let result = match self.validator {
            Validator::Filter(ref v) => v.is_valid(record),
        };

        if !result {
            writer.write_record(Record {
                message: self.message.clone(),
                level: self.level.clone(),
                rule: self.id.clone(),
                cn: record
                    .control_number()
                    .expect("missing control number")
                    .to_str_unchecked()
                    .to_string(),
            })?
        }

        Ok(())
    }
}
