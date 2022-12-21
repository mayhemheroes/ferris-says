#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use ferris_says::say;
use std::io::BufWriter;

#[derive(Debug)]
struct FuzzInput {
    input: String,
    width: usize,
}

const MAX_WIDTH: usize = 100;

impl<'a> Arbitrary<'a> for FuzzInput {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let width = u.int_in_range(0..=MAX_WIDTH)?;
        let input_size = u.arbitrary_len::<char>()?;
        let mut input = String::new();
        for _ in 0..=input_size {
            input.push(u.arbitrary()?);
        }
        Ok(FuzzInput { input, width })
    }
}

fuzz_target!(|data: FuzzInput| {
    let mut writer = BufWriter::new(vec![0; 8196]);
    say(&data.input, data.width, &mut writer).unwrap();
});
