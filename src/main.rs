mod token;

use std::fs;
use std::io::{BufRead, BufReader};

const INPUT_FILE_PATH: &str = "./main.hy";

fn main() {
    let file = fs::File::open(INPUT_FILE_PATH).unwrap();
    let reader = BufReader::new(file);

    let tokens = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .flat_map(|l| {
            l.split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .into_iter()
        })
        .map(|word| word.parse::<token::Token>().unwrap())
        .collect::<Vec<_>>();

    println!("tokens: {:?}", tokens);
}
