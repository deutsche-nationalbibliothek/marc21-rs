use crate::prelude::*;

#[test]
fn subfield_matcher() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // EXISTS
    let matcher = RecordMatcher::new("079{ (a?) }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (b?) }")?;
    assert!(!matcher.is_match(&record, &options));

    // COMPARISON
    let matcher = RecordMatcher::new("079{ (a == 'g') }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (b == 'g') }")?;
    assert!(!matcher.is_match(&record, &options));

    // STARTS WITH
    let matcher = RecordMatcher::new("079{ (a =^ 'g') }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (a =^ 'x') }")?;
    assert!(!matcher.is_match(&record, &options));

    // ENDS WITH
    let matcher = RecordMatcher::new("079{ (a =$ 'g') }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (a =$ 'x') }")?;
    assert!(!matcher.is_match(&record, &options));

    // IN
    let matcher = RecordMatcher::new("079{ (a in ['g', 'k']) }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (a in ['x', 'y']) }")?;
    assert!(!matcher.is_match(&record, &options));

    // REGEX
    let matcher = RecordMatcher::new("079{ (a =~ '^g') }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (a =~ '(?i)^[xyz]') }")?;
    assert!(!matcher.is_match(&record, &options));

    // STRSIM
    let matcher = RecordMatcher::new("079{ (a =* 'g') }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (a =* 'x') }")?;
    assert!(!matcher.is_match(&record, &options));

    // COUNT
    let matcher = RecordMatcher::new("079{ (#q == 3) }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (#a > 1) }")?;
    assert!(!matcher.is_match(&record, &options));

    // GROUP
    let matcher = RecordMatcher::new("079{ ((a?)) }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ ((b?)) }")?;
    assert!(!matcher.is_match(&record, &options));

    // NOT
    let matcher = RecordMatcher::new("079{ (!(b?)) }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (!(a?)) }")?;
    assert!(!matcher.is_match(&record, &options));

    // AND
    let matcher = RecordMatcher::new("079{ (a? && q?) }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (q? && b?) }")?;
    assert!(!matcher.is_match(&record, &options));

    // OR
    let matcher = RecordMatcher::new("079{ (a? || q?) }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("079{ (b? || x?) }")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
