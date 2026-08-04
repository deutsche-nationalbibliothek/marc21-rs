use crate::prelude::*;

#[test]
fn ends_with_single_phrase() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =$ 'Ada'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =$ 'Foo'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#{ [ac] =$ 'lace' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 400/1#.a !$ 'Ada'")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn ends_with_multiple_phrase() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =$ ['Foo', 'Ada']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =$ ['foo', 'bar']")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
