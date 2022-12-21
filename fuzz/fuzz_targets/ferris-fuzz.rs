#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use ferris_says::say;
use std::io::BufWriter;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    input: String,
    width: usize,
}

fuzz_target!(|data: FuzzInput| {
    let mut writer = BufWriter::new(vec![0; 8196]);
    say(&data.input, data.width, &mut writer).unwrap();
});
