use std::collections::{HashMap, VecDeque};
use std::env;
use std::fmt::Debug;
use std::fs;
use std::iter::FromIterator;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Intervals {
    field_name: String,
    left_first: u64,
    right_first: u64,
    left_second: u64,
    right_second: u64,
}

impl Intervals {
    fn is_inside(&self, x: u64) -> bool {
        (self.left_first <= x && x <= self.right_first)
            || (self.left_second <= x && x <= self.right_second)
    }
}

impl Debug for Intervals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let first = format!("{}-{}", self.left_first, self.right_first);
        let second = format!("{}-{}", self.left_second, self.right_second);

        f.debug_tuple("Intervals")
            .field(&self.field_name)
            .field(&first)
            .field(&second)
            .finish()
    }
}

#[derive(Debug)]
struct Data {
    intervals: Vec<Intervals>,
    ticket: Vec<u64>,
    nearby_tickets: Vec<Vec<u64>>,
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 16 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .map(|pack| pack.split("\n").collect())
        .collect();
    let mut data = extract_data(lines);

    let first_answer = get_first_answer(&mut data);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&data);
    println!("Second: {}", second_answer);
}

fn extract_data(lines: Vec<Vec<&str>>) -> Data {
    let interval_lines = &lines[0];
    let ticket = &lines[1];
    let nearby_tickets = &lines[2];

    let mut intervals = vec![];
    for &interval_line in interval_lines {
        let interval_line = interval_line.split(": ").collect::<Vec<_>>();
        let interval_pair = interval_line[1]
            .split(" or ")
            .map(|interval| {
                interval
                    .split("-")
                    .map(|number| number.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        intervals.push(Intervals {
            field_name: interval_line[0].to_string(),
            left_first: interval_pair[0][0],
            right_first: interval_pair[0][1],
            left_second: interval_pair[1][0],
            right_second: interval_pair[1][1],
        });
    }

    let parse_ticket_line = |line: &str| {
        line.split(",")
            .map(|number| number.parse::<u64>().unwrap())
            .collect()
    };

    let ticket = parse_ticket_line(ticket[1]);
    let nearby_tickets = nearby_tickets
        .iter()
        .skip(1)
        .map(|&line| parse_ticket_line(line))
        .collect();

    Data {
        intervals,
        ticket,
        nearby_tickets,
    }
}

fn get_first_answer(data: &mut Data) -> u64 {
    let filtered: Vec<(usize, u64)> = data
        .nearby_tickets
        .iter()
        .enumerate()
        .flat_map(|(index, nearby_ticket)| {
            let mut result = vec![];
            for num in nearby_ticket {
                if !data
                    .intervals
                    .iter()
                    .any(|interval| interval.is_inside(*num))
                {
                    result.push((index, *num));
                }
            }
            result
        })
        .collect();

    let mut result = 0;
    for (index, num) in filtered.iter().rev() {
        result += num;
        data.nearby_tickets.swap_remove(*index);
    }

    result
}

fn get_second_answer(data: &Data) -> u64 {
    let mut possibilities = HashMap::new();
    for index in 0..data.ticket.len() {
        possibilities.insert(index, data.intervals.clone());
    }

    for ticket in &data.nearby_tickets {
        for (i, num) in ticket.iter().enumerate() {
            let possible = possibilities.get(&i).unwrap();
            let new_possible = possible
                .iter()
                .filter(|&intervals| intervals.is_inside(*num))
                .cloned()
                .collect();
            possibilities.insert(i, new_possible);
        }
    }

    let mut result_map = HashMap::new();
    let mut next_indexes =
        VecDeque::from_iter(possibilities.iter().filter_map(|(index, intervals)| {
            if intervals.len() == 1 {
                Some(*index)
            } else {
                None
            }
        }));

    let mut indexes_to_remove = vec![];
    while let Some(next_index) = next_indexes.pop_front() {
        let interval = possibilities.remove(&next_index).unwrap();
        if interval.len() != 1 {
            panic!("SOMETHING WENT REALLY WRONG");
        }
        let field_name = interval[0].field_name.clone();
        result_map.insert(field_name, next_index);

        for (&index, intervals) in possibilities.iter_mut() {
            for (i, possible_interval) in intervals.iter().enumerate().rev() {
                if interval[0].eq(possible_interval) {
                    indexes_to_remove.push(i);
                }
            }

            for &i in &indexes_to_remove {
                intervals.swap_remove(i);
            }

            indexes_to_remove.clear();

            if intervals.len() == 1 {
                next_indexes.push_back(index);
            }
        }
    }

    result_map.iter().fold(1, |acc, (name, &index)| {
        if name.starts_with("departure") {
            acc * data.ticket[index]
        } else {
            acc
        }
    })
}
