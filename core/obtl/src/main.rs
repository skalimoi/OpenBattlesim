use std::{fs, io};
use std::io::Read;
use crate::datatype::{parse_obtl_all};

mod datatype;
mod loader;
mod parser;
mod token;
mod syntax;
fn main() {
    let mut s: String = String::new();
    fs::File::open("test.act").unwrap().read_to_string(&mut s).expect("Cannot open file.");
    let o = parse_obtl_all(s.as_str());
    println!("{:?}", o.groups)
}