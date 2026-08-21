use crate::matcher::MatchOptions;

/// Options and flags which can be used to configure a matcher.
#[derive(Debug, PartialEq)]
pub struct QueryOptions {
    match_options: MatchOptions,
    pub(crate) separator: String,
    pub(crate) squash: bool,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            match_options: MatchOptions::default(),
            separator: "|".into(),
            squash: false,
        }
    }
}

impl QueryOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn match_options(&self) -> &MatchOptions {
        &self.match_options
    }

    pub fn with_separator<S>(mut self, separator: S) -> Self
    where
        S: Into<String>,
    {
        self.separator = separator.into();
        self
    }

    pub fn with_squash(mut self, yes: bool) -> Self {
        self.squash = yes;
        self
    }
}

impl From<MatchOptions> for QueryOptions {
    fn from(options: MatchOptions) -> Self {
        Self {
            match_options: options,
            ..Default::default()
        }
    }
}
