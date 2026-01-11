#![no_main]

use libfuzzer_sys::fuzz_target;
use marc21_record::ByteRecord;

fuzz_target!(|data: &[u8]| {
    let _ = ByteRecord::from_bytes(&data);
});
