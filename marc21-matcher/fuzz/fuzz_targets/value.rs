#![no_main]

use libfuzzer_sys::fuzz_target;
use marc21_matcher::Value;

fuzz_target!(|data: &[u8]| {
    let _ = Value::from_bytes(&data);
});
