use crate::prelude::*;

#[test]
fn field_exists() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("001?")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("!001?")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#?")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/2#?")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("!400/2#?")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn subfield_exists() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // short form
    let matcher = RecordMatcher::new("065.a?")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065.x?")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("!065.a?")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("!065.x?")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065.[ax]?")?;
    assert!(matcher.is_match(&record, &options));

    // long form
    let matcher = RecordMatcher::new("065{ a? }")?;
    assert!(matcher.is_match(&record, &options));

    // composite
    let matcher = RecordMatcher::new("065{ a? && 2 == 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065{ 2 == 'xyz' || a? }")?;
    assert!(matcher.is_match(&record, &options));

    // negation
    let matcher = RecordMatcher::new("065{ !x? }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065{ x? }")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
