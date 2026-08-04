use crate::prelude::*;

#[test]
fn fixed_length_fields() -> TestResult {
    let record = ByteRecord::from_bytes(&ADA_LOVELACE)?;
    let options = MatchOptions::default();

    let matcher = RecordMatcher::new("005[:4] == '2025'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("005[4:] == '0720173911.0'")?;
    assert!(matcher.is_match(&record, &options));

    // 00-05 - Date entered on file
    let matcher = RecordMatcher::new("008[0:6] == '950316'")?;
    assert!(matcher.is_match(&record, &options));

    let matcher = RecordMatcher::new("008[:6] == '950316'")?;
    assert!(matcher.is_match(&record, &options));

    // 06 - Direct or indirect geographic subdivision
    let matcher = RecordMatcher::new("008[6] == 'n'")?;
    assert!(matcher.is_match(&record, &options));

    // 07 - Romanization scheme
    let matcher = RecordMatcher::new("008[7] == '|'")?;
    assert!(matcher.is_match(&record, &options));

    // 08 - Language of catalog
    let matcher = RecordMatcher::new("008[8] == '|'")?;
    assert!(matcher.is_match(&record, &options));

    // 09 - Kind of record
    let matcher = RecordMatcher::new("008[9] == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    // 10 - Descriptive cataloging rules
    let matcher = RecordMatcher::new("008[10] == 'z'")?;
    assert!(matcher.is_match(&record, &options));

    // 11 - Subject heading system/thesaurus
    let matcher = RecordMatcher::new("008[11] == 'z'")?;
    assert!(matcher.is_match(&record, &options));

    // 12 - Type of series
    let matcher = RecordMatcher::new("008[12] == 'n'")?;
    assert!(matcher.is_match(&record, &options));

    // 13 - Numbered or unnumbered series
    let matcher = RecordMatcher::new("008[13] == 'n'")?;
    assert!(matcher.is_match(&record, &options));

    // 14 - Heading use-main or added entry
    let matcher = RecordMatcher::new("008[14] == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    // 15 - Heading use-subject added entry
    let matcher = RecordMatcher::new("008[15] == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    // 16 - Heading use-series added entry
    let matcher = RecordMatcher::new("008[16] == 'b'")?;
    assert!(matcher.is_match(&record, &options));

    // 17 - Type of subject subdivision
    let matcher = RecordMatcher::new("008[17] == 'n'")?;
    assert!(matcher.is_match(&record, &options));

    // 28 - Type of government agency
    let matcher = RecordMatcher::new("008[28] == ' '")?;
    assert!(matcher.is_match(&record, &options));

    // 29 - Reference evaluation
    let matcher = RecordMatcher::new("008[29] == '|'")?;
    assert!(matcher.is_match(&record, &options));

    // 31 - Record update in process
    let matcher = RecordMatcher::new("008[31] == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    // 32 - Undifferentiated personal name
    let matcher = RecordMatcher::new("008[32] == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    // 33 - Level of establishment
    let matcher = RecordMatcher::new("008[33] == 'a'")?;
    assert!(matcher.is_match(&record, &options));

    // 38 - Modified record
    let matcher = RecordMatcher::new("008[38] == '|'")?;
    assert!(matcher.is_match(&record, &options));

    // 39 - Cataloging source
    let matcher = RecordMatcher::new("008[39] == 'c'")?;
    assert!(matcher.is_match(&record, &options));

    // invalid index
    let matcher = RecordMatcher::new("008[40] == 'X'")?;
    assert!(!matcher.is_match(&record, &options));

    // invalid range
    let matcher = RecordMatcher::new("008[0:1024] == 'X'")?;
    assert!(!matcher.is_match(&record, &options));

    Ok(())
}
