#![no_main]
use libfuzzer_sys::fuzz_target;

use ferris_says::say;
use std::io::BufWriter;

fuzz_target!(|data: &[u8]| {
    let width = 24;
    let mut buffer = vec![0; 8196];

    let mut writer = BufWriter::new(buffer);
    say(data, width, &mut writer);
});
