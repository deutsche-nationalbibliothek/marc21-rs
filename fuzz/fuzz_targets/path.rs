#![no_main]

use libfuzzer_sys::fuzz_target;
use marc21::Path;

fuzz_target!(|data: &[u8]| {
    let _ = Path::from_bytes(&data);
});
