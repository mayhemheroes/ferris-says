#![no_main]
use libfuzzer_sys::fuzz_target;

use ferris_says::say;
use std::io::BufWriter;

fuzz_target!(|data: &[u8]| {
    let Ok(s) = std::str::from_utf8(data) else { return; };
    let width = 24;
    let mut buffer = Vec::new();
    let mut writer = BufWriter::new(&mut buffer);
    let _ = say(s, width, &mut writer);
});
