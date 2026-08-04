/// Options and flags which can be used to configure a matcher.
#[derive(Debug, PartialEq)]
pub struct MatchOptions {
    /// The threshold for string similarity comparisons.
    pub(crate) strsim_threshold: f64,
}

impl Default for MatchOptions {
    fn default() -> Self {
        Self {
            strsim_threshold: 0.8,
        }
    }
}

impl MatchOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strsim_threshold(mut self, threshold: f64) -> Self {
        self.strsim_threshold = threshold;
        self
    }
}
