use std::collections::HashMap;
use std::{io,fs};
use std::io::{BufRead, Write};
use io::BufReader;
fn main() {
    let g = HashMap::from([
        ("I","1"), ("V","5"), ("X","10"), ("L","50"),
        ("C", "100"), ("D","500"), ("M", "1000")
    ]);
    let mut input = fs::File::open("cifre.in").unwrap();
    let mut output = fs::OpenOptions::new().write(true).open("cifre.out").unwrap();
    let mut out = String::new();
    for i in BufReader::new(input).lines().skip(1) {
        let i = match i {
            Err(_) => break,
            Ok(i) => i
        };
        out.push_str(&format!("{}\n",g.get(i.trim()).unwrap()));
    };
    output.write_all(out.as_bytes()).unwrap();
}
