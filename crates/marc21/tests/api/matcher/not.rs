use crate::prelude::*;

#[test]
fn not_expression() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher =
        RecordMatcher::new("!075{ b == 'p' && 2 == 'gndgen' }")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("!(075{ b == 'p' && 2 == 'gndgen' })")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("!075.x?")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}
