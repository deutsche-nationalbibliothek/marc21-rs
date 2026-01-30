use std::env::current_dir;
use std::fs;

use marc21::ByteRecord;
use marc21::matcher::RecordMatcher;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_base_address_gt_length() -> TestResult {
    let path = current_dir()?.join("fuzz/regressions").join(
        "minimized-from-aca4bcd701d603885d569470c2c5767f0b54515e",
    );
    assert!(ByteRecord::from_bytes(&fs::read(path)?).is_err());
    Ok(())
}

#[test]
fn test_entry_length_is_zero() -> TestResult {
    let path = current_dir()?.join("fuzz/regressions").join(
        "minimized-from-684599ec8142d3de6a48dac85feefe9aa52e7ef9",
    );
    assert!(ByteRecord::from_bytes(&fs::read(path)?).is_err());
    Ok(())
}

#[test]
fn test_field_count_matcher() -> TestResult {
    let path = current_dir()?.join("fuzz/regressions").join(
        "minimized-from-aca4bcd701d603885d569470c2c5767f0b54515e",
    );

    assert!(RecordMatcher::new(&fs::read(path)?).is_err());
    Ok(())
}
