use std::env::current_dir;
use std::fs;

use marc21_record::ByteRecord;

#[test]
fn test_base_address_gt_length()
-> Result<(), Box<dyn std::error::Error>> {
    let path = current_dir()?.join("fuzz/regressions").join(
        "minimized-from-aca4bcd701d603885d569470c2c5767f0b54515e",
    );
    assert!(ByteRecord::from_bytes(&fs::read(path)?).is_err());

    Ok(())
}
