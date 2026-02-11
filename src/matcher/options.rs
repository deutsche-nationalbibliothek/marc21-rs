/// Options and flags which can be used to configure a matcher.
#[derive(Debug, PartialEq, Default)]
pub struct MatchOptions {
    pub(crate) case_ignore: bool,
}

impl MatchOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn case_ignore(mut self, yes: bool) -> Self {
        self.case_ignore = yes;
        self
    }
}
