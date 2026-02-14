use crate::prelude::*;

#[test]
fn compare_leader_fields() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("ldr.length == 3612")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.length != 3611")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.length >= 3612")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.length > 3611")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.length <= 3612")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.length < 3613")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.base_address == 589")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.status == 'n'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.status != 'd'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.type == 'z'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ldr.encoding == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn compare_control_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("001 == '119232022'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("001 != '119232021'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("005 >= '20250101'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("005 > '20250101'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("005 <= '20251231'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("005 < '20251231'")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn compare_data_field() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("065.a == '28p'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("065{ a == '28p' && 2 == 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065.a == '9.5p'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("065{ a == '9.5p' &&  2 == 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("ALL 065{ a =~ 'p$' &&  2 == 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065.a != '28p'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 065.a != '28p'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new(
        "100/1#{ a == 'Lovelace, Ada' && d >= '1800' }",
    )?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new(
        "ALL 075{ ALL b =~ '^p(iz)?$' && 2 =~ '^gnd' }",
    )?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}
