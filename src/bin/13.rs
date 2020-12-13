use std::env;
use std::fs;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 13 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect();
    let (timestamp, bus_ids) = extract_data(lines);

    let non_empty_bus_ids = bus_ids
        .iter()
        .filter(|&id| id.is_some())
        .map(|id| id.unwrap())
        .collect();
    let first_answer = get_first_answer(timestamp, &non_empty_bus_ids);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&bus_ids);
    println!("Second: {}", second_answer);
}

fn extract_data(lines: Vec<&str>) -> (u64, Vec<Option<u64>>) {
    let timestamp = lines[0].parse().unwrap();
    let ids = lines[1].split(",").map(|id| id.parse().ok()).collect();
    (timestamp, ids)
}

fn get_first_answer(timestamp: u64, bus_ids: &Vec<u64>) -> u64 {
    bus_ids
        .iter()
        .map(|&id| {
            let multiplier = timestamp / id;
            let lower_bound = multiplier * id;
            let upper_bound = lower_bound + id;

            if lower_bound >= timestamp {
                (lower_bound - timestamp, id)
            } else {
                (upper_bound - timestamp, id)
            }
        })
        .min_by(|(wait_time_x, _), (wait_time_y, _)| wait_time_x.cmp(wait_time_y))
        .map(|(wait_time, id)| wait_time * id)
        .unwrap()
}

// The solution is based on the Chinese remainder theorem
fn get_second_answer(bus_ids: &Vec<Option<u64>>) -> i128 {
    let bus_pairs = bus_ids
        .iter()
        .enumerate()
        .filter(|&(_, option)| option.is_some())
        .map(|(i, option)| {
            let value = option.unwrap();
            if i == 0 {
                (0, value)
            } else {
                (value - (i as u64 % value), value)
            }
        })
        .map(|(a, b)| (a as i128, b as i128))
        .collect::<Vec<_>>();
    let big_m = bus_pairs.iter().fold(1 as i128, |acc, (_, m)| acc * m);
    let big_ms = bus_pairs.iter().map(|(_, m)| big_m / m).collect::<Vec<_>>();

    let result: i128 = bus_pairs
        .iter()
        .zip(big_ms)
        .map(|(&(i, mi), big_mi)| {
            let (_, g) = extended_euclidean(mi, big_mi);
            i * g * big_mi
        })
        .sum();

    // result is probably too big, so we need to reduce it by a M multiple
    let multiple = result / big_m;
    if result < 0 {
        result + (multiple.abs() + 1) * big_m
    } else {
        result - multiple * big_m
    }
}

fn extended_euclidean(a: i128, b: i128) -> (i128, i128) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;

        let tmp_r = r;
        r = old_r - quotient * r;
        old_r = tmp_r;

        let tmp_s = s;
        s = old_s - quotient * s;
        old_s = tmp_s;

        let tmp_t = t;
        t = old_t - quotient * t;
        old_t = tmp_t;
    }

    (old_s, old_t)
}
