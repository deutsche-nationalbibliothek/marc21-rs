#![no_main]

use libfuzzer_sys::fuzz_target;
use marc21::matcher::TagMatcher;

fuzz_target!(|data: &[u8]| {
    let _ = TagMatcher::new(&data);
});
