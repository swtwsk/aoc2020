use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
enum Command<'a> {
    SetMask(&'a str),
    Put(u64, u64),
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 14 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect();
    let commands = extract_data(lines);

    let first_answer = get_first_answer(&commands);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&commands);
    println!("Second: {}", second_answer);
}

fn extract_data<'a>(lines: Vec<&'a str>) -> Vec<Command<'a>> {
    let lines = lines
        .iter()
        .map(|&line| line.split(" = ").collect::<Vec<_>>())
        .filter(|line| line.len() == 2);

    let mut result = vec![];
    for line in lines {
        if line[0] == "mask" {
            result.push(Command::SetMask(line[1]));
            continue;
        }

        let memory_index = (&line[0][4..(line[0].len() - 1)]).parse().unwrap();
        let value = line[1].parse().unwrap();
        result.push(Command::Put(memory_index, value));
    }

    result
}

fn get_first_answer(commands: &Vec<Command<'_>>) -> u64 {
    let mut current_mask = vec![];
    let mut memory = HashMap::new();

    for command in commands {
        match command {
            Command::SetMask(mask) => {
                current_mask = extract_mask(mask);
            }
            Command::Put(mem, value) => {
                let value = current_mask.iter().fold(*value, |acc, fun| fun(acc));
                memory.insert(mem, value);
            }
        }
    }

    memory.values().sum()
}

fn get_second_answer(commands: &Vec<Command<'_>>) -> u64 {
    let mut current_mask = vec![];
    let mut memory = HashMap::new();

    for command in commands {
        match command {
            Command::SetMask(mask) => {
                current_mask = extract_mask_snd(mask);
            }
            Command::Put(mem, value) => {
                let decoded_addresses = current_mask.iter().fold(vec![*mem], |acc, funs| {
                    acc.iter()
                        .flat_map::<Vec<u64>, _>(|&el| funs.iter().map(|fun| fun(el)).collect())
                        .collect()
                });
                for mem in decoded_addresses {
                    memory.insert(mem, *value);
                }
            }
        }
    }

    memory.values().sum()
}

fn extract_mask(mask: &str) -> Vec<Box<dyn Fn(u64) -> u64>> {
    let closures = mask
        .chars()
        .enumerate()
        .map(|(i, c)| (mask.len() - 1 - i, c))
        .filter(|&(_, c)| c != 'X')
        .map(|(i, c)| {
            let x = if c == '1' { 1 } else { 0 };
            let lambda = move |number: u64| (number & !((1 as u64) << i)) | (x << i);
            Box::new(lambda) as Box<dyn Fn(u64) -> u64>
        })
        .collect::<Vec<_>>();

    closures
}

fn extract_mask_snd(mask: &str) -> Vec<Vec<Box<dyn Fn(u64) -> u64>>> {
    let closures = mask
        .chars()
        .enumerate()
        .map(|(i, c)| (mask.len() - 1 - i, c))
        .filter(|&(_, c)| c != '0')
        .map(|(i, c)| {
            let setting_lambda = move |number: u64| (number | ((1 as u64) << i));
            if c == '1' {
                vec![Box::new(setting_lambda) as Box<dyn Fn(u64) -> u64>]
            } else {
                let clearing_lambda = move |number: u64| (number & !((1 as u64) << i));
                vec![
                    Box::new(setting_lambda) as Box<dyn Fn(u64) -> u64>,
                    Box::new(clearing_lambda) as Box<dyn Fn(u64) -> u64>,
                ]
            }
        })
        .collect::<Vec<_>>();

    closures
}
