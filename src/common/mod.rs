use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_name: &str) -> String {
    let mut f = File::open(file_name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}
