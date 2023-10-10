mod grammar;
mod parser;
mod token;
mod tokenization;

use parser::*;
use std::fs;
use std::io::{BufReader, Write};
use std::process::Command;
use tokenization::Tokenizer;
use utf8_chars::BufReadCharsExt;

const INPUT_FILE_PATH: &str = "./main.hy";
const OUTPUT_FILE_PATH: &str = "./target/out.asm";

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

    let mut output = fs::File::create(OUTPUT_FILE_PATH).unwrap();
    output.write_all(root.to_asm().as_bytes()).unwrap();

    let nasm = Command::new("nasm")
        .args(["-felf64", OUTPUT_FILE_PATH])
        .output()
        .expect("nasm error");
    if !(nasm.status.success()) {
        panic!("nasm error: {nasm:?}");
    }

    let linker = Command::new("ld")
        .args(["-o", "out", "./target/out.o"])
        .output()
        .expect("linker error");
    if !(linker.status.success()) {
        panic!("linker error: {linker:?}");
    }
}
