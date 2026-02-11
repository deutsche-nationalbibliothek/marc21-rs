//! Various matchers that can be applied to a [`ByteRecord`] or its
//! element.
//!
//! # Overview
//!
//! The essential matcher types of this module are:
//!
//! * [`RecordMatcher`] --- check a record and its elements for specific
//!   properties
//! * [`LeaderMatcher`] --- check leader fields for specific properties
//!
//! * ...
//!
//! In addition to these high-level matchers, the API also provides
//! specialized matchers that can be applied to the elements of a
//! record:
//!
//! * [`TagMatcher`] --- check for a single tag or a set of tags
//! * [`IndicatorMatcher`] --- checks the indicator fields of a field
//! * [`FieldMatcher`] --- check a field and its subfields for specific
//!   properties
//! * [`SubfieldMatcher`]
//! * ...
//!
//! The behavior of some matchers can be influenced by [`MatchOptions`].
//!
//! # Errors
//!
//! Any parse error will return a [`ParseMatcherError`].
//!
//! [`ByteRecord`]: crate::ByteRecord

pub use error::ParseMatcherError;
pub use field::FieldMatcher;
pub use indicator::IndicatorMatcher;
pub use leader::LeaderMatcher;
pub use options::MatchOptions;
pub use record::RecordMatcher;
pub use subfield::SubfieldMatcher;
pub use tag::TagMatcher;

pub(crate) mod error;
pub(crate) mod field;
pub(crate) mod indicator;
pub(crate) mod leader;
pub(crate) mod options;
pub(crate) mod record;
pub(crate) mod shared;
pub(crate) mod subfield;
pub(crate) mod tag;
