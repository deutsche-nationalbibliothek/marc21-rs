#![no_main]

use libfuzzer_sys::fuzz_target;
use marc21_matcher::TagMatcher;

fuzz_target!(|data: &[u8]| {
    let _ = TagMatcher::from_bytes(&data);
});
