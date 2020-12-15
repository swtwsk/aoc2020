use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 15 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers = contents
        .split("\n")
        .nth(0)
        .unwrap()
        .split(",")
        .map(|el| el.parse().unwrap())
        .collect();

    let first_answer = get_first_answer(&numbers);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&numbers);
    println!("Second: {}", second_answer);
}

fn get_first_answer(numbers: &Vec<u64>) -> u64 {
    get_answer(numbers, 2020)
}

fn get_second_answer(numbers: &Vec<u64>) -> u64 {
    get_answer(numbers, 30000000)
}

fn get_answer(numbers: &Vec<u64>, nth: u64) -> u64 {
    let mut last_indexes = HashMap::new();

    for i in 0..numbers.len() - 1 {
        last_indexes.insert(numbers[i], i + 1);
    }

    (numbers.len()..(nth as usize)).fold(*numbers.last().unwrap(), |last, index| {
        let result = match last_indexes.get(&last) {
            Some(last_index) => index - last_index,
            None => 0,
        };

        last_indexes.insert(last, index);
        result as u64
    })
}
