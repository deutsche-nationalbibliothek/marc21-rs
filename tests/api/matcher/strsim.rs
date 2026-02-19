use crate::prelude::*;

#[test]
fn strsim_single_phrase() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("100/1#.a =* 'Lovelace, Ada'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("100/1#.a =* 'Hatelace, Ada'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("100/1#.a =* 'Lovelace, Bda'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("100/1#.a !* 'Lovelace, Ada'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("100/1#.a !* 'Hatelace, Ada'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("100/1#.a !* 'Lovelace, Bda'")?;
    assert!(!matcher.is_match(&record, &options));

    let options = MatchOptions::default().strsim_threshold(0.99);
    let matcher = RecordMatcher::new("100/1#.a =* 'Lovelace, Bda'")?;
    assert!(!matcher.is_match(&record, &options));

    let options = MatchOptions::default().strsim_threshold(0.99);
    let matcher = RecordMatcher::new("100/1#.a !* 'Lovelace, Bda'")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn strsim_multiple_phrases() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher =
        RecordMatcher::new("100/1#.a =* ['Foo', 'Lovelace, Ada']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("100/1#.a !* ['Foo', 'Lovelace, Ada']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("100/1#.a !* ['Foo', 'Bar']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("100/1#.a =* ['Foo', 'Bar']")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
