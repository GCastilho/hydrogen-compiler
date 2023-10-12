mod grammar;
mod parser;
mod token;
mod tokenization;

use parser::*;
use std::fs;
use std::io::BufReader;
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
        .expect("parse failed");
    println!("root: {:?}", root);

    let output = fs::File::create(OUTPUT_FILE_PATH).unwrap();
    root.to_asm(output);

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
