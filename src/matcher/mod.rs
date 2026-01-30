//! Matchers that can be applied to a [ByteRecord](crate::ByteRecord) or
//! its element.

pub use error::ParseMatcherError;
pub use field_matcher::FieldMatcher;
pub use indicator_matcher::IndicatorMatcher;
pub use leader_matcher::LeaderMatcher;
pub use options::MatchOptions;
pub use record_matcher::RecordMatcher;
pub use subfield_matcher::SubfieldMatcher;
pub use tag_matcher::TagMatcher;

mod comparison_matcher;
mod error;
mod field_matcher;
mod indicator_matcher;
mod leader_matcher;
mod operator;
mod options;
mod quantifier;
mod record_matcher;
mod subfield_matcher;
mod tag_matcher;
mod utils;
mod value;
