use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 9 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers: Vec<u64> = contents
        .split("\n")
        .filter(|&line| line.len() > 0)
        .map(|line| line.parse().unwrap())
        .collect();

    let first_answer = get_first_answer(&numbers);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&numbers, first_answer);
    println!("Second: {}", second_answer);
}

fn get_first_answer(numbers: &Vec<u64>) -> u64 {
    let mut set = vec![];
    let preambule_size = 25;

    for i in 0..preambule_size {
        let subsum = subsums(numbers, i, preambule_size - 1);
        set.push(subsum);
    }

    for i in preambule_size..numbers.len() {
        let contains = set.iter().find(|&vec| vec.contains(&numbers[i])).is_some();
        if !contains {
            return numbers[i];
        }

        set.remove(0);
        set.iter_mut().enumerate().for_each(|(index, vec)| {
            vec.insert(numbers[i - preambule_size + index + 1] + numbers[i]);
        });
        set.push(HashSet::new());
    }

    0
}

fn get_second_answer(numbers: &Vec<u64>, sum: u64) -> u64 {
    let mut curr_sum = numbers[0];
    let mut start = 0;

    let mut left_index = 0;
    let mut right_index = 0;

    for i in 1..(numbers.len() + 1) {
        while curr_sum > sum && start < i - 1 {
            curr_sum -= numbers[start];
            start += 1;
        }

        if curr_sum == sum {
            left_index = start;
            right_index = i - 1;
            break;
        }

        if i < numbers.len() {
            curr_sum = curr_sum + numbers[i];
        }
    }

    if right_index == 0 {
        panic!("Something went very wrong!");
    }

    let mut smallest = std::u64::MAX;
    let mut largest = std::u64::MIN;
    for i in left_index..(right_index + 1) {
        smallest = min(smallest, numbers[i]);
        largest = max(largest, numbers[i]);
    }

    smallest + largest
}

fn subsums(numbers: &Vec<u64>, left_index: usize, right_index: usize) -> HashSet<u64> {
    let mut res = HashSet::new();
    for i in (left_index + 1)..(right_index + 1) {
        res.insert(numbers[left_index] + numbers[i]);
    }
    res
}
