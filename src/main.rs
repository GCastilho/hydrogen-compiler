mod parser;
mod token;
mod tokenization;

use parser::*;
use std::fs;
use std::io::BufReader;
use tokenization::Tokenizer;
use utf8_chars::BufReadCharsExt;

const INPUT_FILE_PATH: &str = "./main.hy";

fn main() {
    let file = fs::File::open(INPUT_FILE_PATH).unwrap();
    let mut reader = BufReader::new(file);

    let root = reader
        .chars()
        .tokens()
        .map(|c| c.expect("tokens failed"))
        .parse()
        .unwrap();
    println!("root: {:?}", root);
}
