use std::env;
use std::fs;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 11 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading file");
    let contents = contents
        .split("\n")
        .filter(|&line| line.len() > 0)
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("First: {}", get_first_answer(contents.clone()));
    println!("Second: {}", get_second_answer(contents));
}

fn get_first_answer(contents: Vec<Vec<char>>) -> u64 {
    get_answer(contents, 4, first_count_occupied)
}

fn get_second_answer(contents: Vec<Vec<char>>) -> u64 {
    get_answer(contents, 5, second_count_occupied)
}

fn get_answer<F>(
    mut contents: Vec<Vec<char>>,
    occupied_counter_threshold: u64,
    count_occupied: F,
) -> u64
where
    F: Fn(&Vec<Vec<char>>, usize, usize) -> u64,
{
    loop {
        let mut change = false;

        let mut new_contents = contents.clone();

        for i in 0..contents.len() {
            for j in 0..contents[i].len() {
                match contents[i][j] {
                    'L' => {
                        if count_occupied(&contents, i, j) == 0 {
                            new_contents[i][j] = '#';
                            change = true;
                        }
                    }
                    '#' => {
                        if count_occupied(&contents, i, j) >= occupied_counter_threshold {
                            new_contents[i][j] = 'L';
                            change = true;
                        }
                    }
                    _ => {}
                }
            }
        }

        if !change {
            break;
        }

        contents = new_contents;
    }
    contents.iter().fold(0, |acc, line| {
        acc + line
            .iter()
            .fold(0, |acc, &el| if el == '#' { acc + 1 } else { acc })
    })
}

fn one_if(fields: &Vec<Vec<char>>, i: usize, j: usize) -> u64 {
    if fields[i][j] == '#' {
        1
    } else {
        0
    }
}

fn first_count_occupied(fields: &Vec<Vec<char>>, i: usize, j: usize) -> u64 {
    let mut counter = 0;

    if i > 0 {
        if j > 0 {
            counter += one_if(fields, i - 1, j - 1);
        }
        if j < fields[i].len() - 1 {
            counter += one_if(fields, i - 1, j + 1);
        }
        counter += one_if(fields, i - 1, j);
    }

    if i < fields.len() - 1 {
        if j > 0 {
            counter += one_if(fields, i + 1, j - 1);
        }
        if j < fields[i].len() - 1 {
            counter += one_if(fields, i + 1, j + 1);
        }
        counter += one_if(fields, i + 1, j);
    }

    if j > 0 {
        counter += one_if(fields, i, j - 1);
    }
    if j < fields[i].len() - 1 {
        counter += one_if(fields, i, j + 1);
    }

    counter
}

fn one_if_condition<F>(
    fields: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    i_mod: i64,
    j_mod: i64,
    condition: F,
) -> u64
where
    F: Fn(i64, i64) -> bool,
{
    let mut answer = 0;

    let mut i_tmp = i as i64;
    let mut j_tmp = j as i64;

    while condition(i_tmp, j_tmp) {
        i_tmp += i_mod;
        j_tmp += j_mod;

        match fields[i_tmp as usize][j_tmp as usize] {
            'L' => {
                answer = 0;
                break;
            }
            '#' => {
                answer = 1;
                break;
            }
            _ => {}
        }
    }

    answer
}

fn second_count_occupied(fields: &Vec<Vec<char>>, i: usize, j: usize) -> u64 {
    let mut counter = 0;

    counter += one_if_condition(fields, i, j, -1, -1, |i, j| i > 0 && j > 0);
    counter += one_if_condition(fields, i, j, -1, 1, |i, j| {
        i > 0 && j < fields[0].len() as i64 - 1
    });
    counter += one_if_condition(fields, i, j, -1, 0, |i, _| i > 0);
    counter += one_if_condition(fields, i, j, 1, -1, |i, j| {
        i < fields.len() as i64 - 1 && j > 0
    });
    counter += one_if_condition(fields, i, j, 1, 1, |i, j| {
        i < fields.len() as i64 - 1 && j < fields[0].len() as i64 - 1
    });
    counter += one_if_condition(fields, i, j, 1, 0, |i, _| i < fields.len() as i64 - 1);
    counter += one_if_condition(fields, i, j, 0, -1, |_, j| j > 0);
    counter += one_if_condition(fields, i, j, 0, 1, |_, j| j < fields[0].len() as i64 - 1);

    counter
}
