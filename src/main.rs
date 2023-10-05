mod parser;
mod token;

use parser::Tokenizer;
use std::fs;
use std::io::BufReader;
use utf8_chars::BufReadCharsExt;

const INPUT_FILE_PATH: &str = "./main.hy";

fn main() {
    let file = fs::File::open(INPUT_FILE_PATH).unwrap();
    let mut reader = BufReader::new(file);

    let tokens = reader
        .chars()
        .tokens()
        .map(|c| c.expect("tokens failed"))
        .collect::<Vec<_>>();
    println!("tokens: {:?}", tokens);
}
