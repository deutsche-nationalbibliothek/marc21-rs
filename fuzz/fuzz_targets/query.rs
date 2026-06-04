#![no_main]

use libfuzzer_sys::fuzz_target;
use marc21::Query;

fuzz_target!(|data: &[u8]| {
    let _ = Query::from_bytes(&data);
});
