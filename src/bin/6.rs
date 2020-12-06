use std::fs;
use std::{collections::HashSet, env, str::Split};

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 6 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading file");
    let contents = contents.split("\n\n");

    println!("First: {}", get_first_answer(contents.clone()));
    println!("Second: {}", get_second_answer(contents));
}

fn get_first_answer(contents: Split<&str>) -> usize {
    contents
        .map(|str| str.split_whitespace().collect::<String>())
        .map(|str| str.chars().collect::<HashSet<_>>())
        .fold(0, |acc, set| acc + set.len())
}

fn get_second_answer(contents: Split<&str>) -> usize {
    let split_by_whitespace = contents.map(|str| str.split_whitespace().collect::<Vec<_>>());
    let mut counter: usize = 0;

    for string_vec in split_by_whitespace {
        let char_value = |c: char| c as u32 - 97;

        let vec_size = string_vec.len();
        let mut array = ('a'..='z').map(|_| 0).collect::<Vec<_>>();

        for string in string_vec {
            for c in string.chars() {
                array[char_value(c) as usize] += 1;
            }
        }

        counter += array.iter().fold(0, |acc, &el| {
            if (el as usize) == vec_size {
                acc + 1
            } else {
                acc
            }
        });
    }

    counter
}
