use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::fs;

type CommandPair<'a> = (&'a str, i64);

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 8 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect();
    let commands = convert_line_to_command(lines);

    let first_answer = get_first_answer(&commands);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&commands);
    println!("Second: {}", second_answer);
}

fn convert_line_to_command(lines: Vec<&str>) -> Vec<CommandPair> {
    lines
        .iter()
        .filter(|&&line| line.len() > 1)
        .map(|&line| {
            let cmds = line.split(" ").collect::<Vec<_>>();
            let (cmd_op, cmd_mod) = (cmds[0], cmds[1]);
            let parsed: i64 = cmd_mod.parse().unwrap();
            (cmd_op, parsed)
        })
        .collect()
}

fn get_first_answer(commands: &Vec<CommandPair>) -> i64 {
    let mut used = HashSet::new();

    let mut acc = 0;
    let mut index = 0;
    loop {
        if used.contains(&index) {
            return acc;
        } else {
            used.insert(index);
        }

        let (cmd_op, cmd_mod) = commands[index];

        match cmd_op {
            "nop" => {
                index += 1;
            }
            "acc" => {
                acc += cmd_mod;
                index += 1;
            }
            "jmp" => {
                index = max(0, index as i64 + cmd_mod) as usize;
            }
            _ => {}
        };
    }
}

fn get_second_answer(commands: &Vec<CommandPair>) -> i64 {
    backtrack(0, 0, false, commands, &mut HashSet::new()).unwrap()
}

fn backtrack(
    index: usize,
    acc: i64,
    changed: bool,
    commands: &Vec<CommandPair>,
    visited: &mut HashSet<usize>,
) -> Option<i64> {
    if visited.contains(&index) {
        return None;
    }
    if index == commands.len() - 1 {
        return Some(acc);
    }

    visited.insert(index);

    let (cmd_op, cmd_mod) = commands[index];

    let result = match cmd_op {
        "nop" => {
            let jmp_index = max(0, index as i64 + cmd_mod) as usize;

            if changed {
                backtrack(index + 1, acc, true, commands, visited)
            } else {
                match backtrack(index + 1, acc, false, commands, visited) {
                    Some(res) => Some(res),
                    None => backtrack(jmp_index, acc, true, commands, visited),
                }
            }
        }
        "acc" => backtrack(index + 1, acc + cmd_mod, changed, commands, visited),
        "jmp" => {
            let jmp_index = max(0, index as i64 + cmd_mod) as usize;

            if changed {
                backtrack(jmp_index, acc, true, commands, visited)
            } else {
                match backtrack(jmp_index, acc, false, commands, visited) {
                    Some(res) => Some(res),
                    None => backtrack(index + 1, acc, true, commands, visited),
                }
            }
        }
        _ => None,
    };

    visited.remove(&index);
    result
}
