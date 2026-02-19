use crate::prelude::*;

#[test]
fn record_conjunction() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher =
        RecordMatcher::new("001 == '119232022' && ldr.type == 'z'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("001 == '119232022' && ldr.status == 'a'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new(
        "001 == '119232022' && ldr.type == 'z' && 100/1#.a == 'Lovelace, Ada'",
    )?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new(
        "001 == '1192320XX' && ldr.type == 'z' && 100/1#.a == 'Lovelace, Ada'",
    )?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn record_disjunction() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher =
        RecordMatcher::new("001 == '119232022' || ldr.type == 'z'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("001 == '119232022' || ldr.type == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("001 == '119232023' || ldr.type == 'z'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("001 == '119232023' || ldr.type == 'a'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new(
        "001 == '119232023' || ldr.type == 'a' || 100/1#.a =^ 'Love'",
    )?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn record_connective() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // FALSE && FALSE || TRUE ≡ (FALSE && FALSE) || TRUE
    let matcher = RecordMatcher::new(
        "001 == '119232023' && ldr.type == 'a' || 100/1#.a =^ 'Love'",
    )?;
    assert!(matcher.is_match(&record, &options));

    // FALSE || FALSE && TRUE ≡ FALSE || (FALSE && TRUE)
    let matcher = RecordMatcher::new(
        "001 == '119232023' || ldr.type == 'a' && 100/1#.a =^ 'Love'",
    )?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn field_conjunction() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher =
        RecordMatcher::new("065{ a == '28p' && 2 == 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("065{ a == '28p' && 2 != 'sswd' }")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065{ a =^ '3' && 2 == 'sswd' }")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("065{ a =^ '3' && 2 != 'sswd' }")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn field_disjunction() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    // TRUE || TRUE = TRUE
    let matcher =
        RecordMatcher::new("065{ a == '28p' || 2 == 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    // TRUE || FALSE = TRUE
    let matcher =
        RecordMatcher::new("065{ a == '28p' || 2 != 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    // FALSE || TRUE == TRUE
    let matcher = RecordMatcher::new("065{ a =^ '3' || 2 == 'sswd' }")?;
    assert!(matcher.is_match(&record, &options));

    // FALSE || FALSE == TRUE
    let matcher = RecordMatcher::new("065{ a =^ '3' || 2 != 'sswd' }")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
