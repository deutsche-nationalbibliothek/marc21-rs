use crate::prelude::*;

#[test]
fn contains_single_phrase() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =? 'Ada'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =? 'Hate'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#{ a =? 'Ada' || a =? 'Love' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#{ a =? 'Ada' && a =? 'Hate' }")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 400/1#.a =? 'Ada'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 400/1#{ ALL a =? 'Ada' }")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn contains_multiple_phrases() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =? ['Hate', 'Love']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =? ['Hate', 'Move']")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
