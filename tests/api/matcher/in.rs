use crate::prelude::*;

#[test]
fn control_field_in() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher =
        RecordMatcher::new("001 in ['040992918', '119232022']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("001 in ['040992918', '119232023']")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("001 not in ['040992918', '119232022']")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("001 not in ['040992918', '119232023']")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn data_field_in() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("065.a in ['28p', '9.5p']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065.a not in ['28p', '9.5p']")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065.a not in ['28p', '29p']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065{ a in ['28p', '9.5p'] }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 065.a in ['28p', '9.5p']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 065.a in ['28p', '29p']")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("ALL 065{ a in ['28p', '9.5p'] }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 065{ a in ['28p', '29p'] }")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
