#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use assert_fs::prelude::*;
use assert_fs::TempDir;
use libfuzzer_sys::fuzz_target;
use std::process::Command;

const MAX_WIDTH: usize = 100;

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    files: Vec<String>,
    /// Because at least one file is needed
    last_file: String,
    #[arbitrary(with = |u: &mut Unstructured| u.int_in_range(0..=MAX_WIDTH))]
    width: usize,
}

fuzz_target!(|data: FuzzInput| {
    let temp_dir = TempDir::new().unwrap();

    let mut paths = Vec::new();

    let mut data = data;
    data.files.push(data.last_file);

    for (i, contents) in data.files.iter().enumerate() {
        let file = temp_dir.child(format!("file_{i}"));
        // Write the contents of each file
        file.write_str(contents).unwrap();
        // Save the file path to pass to fsays later
        paths.push(file.to_str().unwrap().to_owned());
    }

    let output = Command::new("fsays")
        .arg("-w")
        .arg(data.width.to_string())
        .arg("-f")
        .args(&paths)
        .output()
        .expect("fsays failed");
    if !output.status.success() {
        println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
        println!("stderr: {}", String::from_utf8(output.stderr).unwrap());
        panic!("fsays exit status was {}", output.status);
    }
});
