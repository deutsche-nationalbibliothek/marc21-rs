use crate::prelude::*;

#[test]
fn count_subfield() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // equal
    let matcher = RecordMatcher::new("079{ #q == 3 }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ #a == 3 }")?;
    assert!(!matcher.is_match(&record, &options));

    // not equal
    let matcher = RecordMatcher::new("079{ #a != 3 }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ #q != 3 }")?;
    assert!(!matcher.is_match(&record, &options));

    // greater than or equal
    let matcher = RecordMatcher::new("079{ #q >= 3 }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ #a >= 3 }")?;
    assert!(!matcher.is_match(&record, &options));

    // greater than
    let matcher = RecordMatcher::new("079{ #q > 2 }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ #q > 3 }")?;
    assert!(!matcher.is_match(&record, &options));

    // less than or equal
    let matcher = RecordMatcher::new("079{ #q <= 3 }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ #q <= 2 }")?;
    assert!(!matcher.is_match(&record, &options));

    // less than
    let matcher = RecordMatcher::new("079{ #q < 4 }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ #q < 3 }")?;
    assert!(!matcher.is_match(&record, &options));

    // code groups
    let matcher = RecordMatcher::new("079{ #[qu] == 6 }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ #[aq] == 6 }")?;
    assert!(!matcher.is_match(&record, &options));

    // non-existing subfield code
    let matcher = RecordMatcher::new("079{ #x == 0 }")?;
    assert!(matcher.is_match(&record, &options));

    // group
    let matcher = RecordMatcher::new("079{ (q?) }")?;
    assert!(matcher.is_match(&record, &options));

    // let matcher = RecordMatcher::new("079{ (#a == 3) }")?;
    // assert!(!matcher.is_match(&record, &options));

    Ok(())
}
