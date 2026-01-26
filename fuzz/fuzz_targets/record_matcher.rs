#![no_main]

use libfuzzer_sys::fuzz_target;
use marc21::matcher::RecordMatcher;

fuzz_target!(|data: &[u8]| {
    let _ = RecordMatcher::new(&data);
});
