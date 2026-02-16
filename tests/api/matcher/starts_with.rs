use crate::prelude::*;

#[test]
fn starts_with_single_phrase() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =^ 'Love'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =^ 'Hate'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#{ [ac] =^ 'Count' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 400/1#.a !^ 'Love'")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn starts_with_multiple_phrase() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =^ ['Hate', 'Love']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =^ ['foo', 'bar']")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
