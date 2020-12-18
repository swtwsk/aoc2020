use std::env;
use std::fs;

use advent_of_code_2020::lib_18;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 18 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents = contents
        .strip_suffix("\n")
        .unwrap()
        .split("\n")
        .collect::<Vec<_>>();

    let first_answer: i64 = contents
        .iter()
        .map(|line| lib_18::first_parse(line).unwrap())
        .map(|expr| expr.compute_value())
        .sum();
    println!("First: {:?}", first_answer);

    let second_answer: i64 = contents
        .iter()
        .map(|line| lib_18::second_parse(line).unwrap())
        .map(|expr| expr.compute_value())
        .sum();
    println!("Second: {:?}", second_answer);
}
