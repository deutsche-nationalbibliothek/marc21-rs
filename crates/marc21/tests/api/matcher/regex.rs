use crate::prelude::*;

#[test]
fn match_single_pattern() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =~ '^K[io]ng.*Ada$'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =~ '^K[Io]ng.*Ada$'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#{ a =~ '^Byr' && 4 == 'nafr' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("ANY 400/1#.a =~ '^K[io]ng.*Ada$'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("ALL 400/1#.a =~ 'Ada'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#{ ANY a =~ '^Byr' && 4 == 'nafr' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("040{ ALL [ac] =~ '^DE-' && b == 'ger' }")?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn match_negated_single_pattern() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a !~ '^(Love|Byron)'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#.a !~ '^(Love|Byron|King)'")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#{ a !~ '^King' && 4 == 'nafr' }")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#{ a !~ '^Byr' && 4 == 'nafr' }")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn match_multiple_patterns() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a =~ ['^Foo', '^Byron']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a =~ ['^Foo', '^Bar']")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new(
        "400/1#{ a =~ ['^Byr', 'sta$'] && 4 == 'nafr' }",
    )?;
    assert!(matcher.is_match(&record, &options));

    Ok(())
}

#[test]
fn match_negated_multiple_patterns() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("400/1#.a !~ ['^Foo', '^Byron']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("400/1#.a !~ ['^Foo', '^Bar']")?;
    assert!(matcher.is_match(&record, &options));

    let matcher =
        RecordMatcher::new("400/1#.a !~ ['^King', '^Byron', '^Love']")?;
    assert!(!matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new(
        "400/1#{ a !~ ['^King', '^Byron'] && 4 == 'nafr' }",
    )?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
