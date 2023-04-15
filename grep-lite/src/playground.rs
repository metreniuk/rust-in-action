use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn playground() {
    let f = File::open("main.rs").unwrap();
    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
