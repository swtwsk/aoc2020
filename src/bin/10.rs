use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 10 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading file");
    let mut contents = contents
        .split("\n")
        .filter(|&line| line.len() > 0)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    contents.sort();
    contents.insert(0, 0);
    contents.push(contents.last().unwrap() + 3);

    println!("First: {}", get_first_answer(&contents));
    println!("Second: {}", get_second_answer(&contents));
}

fn get_first_answer(contents: &Vec<u64>) -> u64 {
    let (ones, thirds) = contents
        .iter()
        .zip(contents.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .fold((0, 0), |(acc1, acc3), diff| {
            if diff == 1 {
                (acc1 + 1, acc3)
            } else if diff == 3 {
                (acc1, acc3 + 1)
            } else {
                (acc1, acc3)
            }
        });

    ones * thirds
}

fn get_second_answer(contents: &Vec<u64>) -> u128 {
    let mut graph_map = HashMap::new();
    let mut result_map = HashMap::new();
    let n = contents.len();

    for i in 0..n {
        let mut neighbours = vec![];
        if i < n - 1 && contents[i + 1] <= contents[i] + 3 {
            neighbours.push(i + 1);
        }
        if i < n - 2 && contents[i + 2] <= contents[i] + 3 {
            neighbours.push(i + 2);
        }
        if i < n - 3 && contents[i + 3] <= contents[i] + 3 {
            neighbours.push(i + 3);
        }
        graph_map.insert(i, neighbours);
    }

    result_map.insert(n - 1, 1 as u128);
    for i in (0..(n - 1)).rev() {
        let options = graph_map
            .get(&i)
            .unwrap()
            .iter()
            .map(|neighbour| result_map.get(neighbour).unwrap())
            .sum();
        result_map.insert(i, options);
    }

    let &result = result_map.get(&0).unwrap();
    result as u128
}
