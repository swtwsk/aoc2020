use std::collections::{HashMap, LinkedList};
use std::env;
use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Grammar {
    terminal_productions: HashMap<usize, char>,
    nonterminal_productions: HashMap<usize, Vec<Vec<usize>>>,
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 16 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents = contents
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .map(|pack| pack.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let grammar_lines = &contents[0];
    let words = &contents[1];

    let mut grammar = extract_grammar(grammar_lines);

    let first_answer = get_first_answer(&grammar, words);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&mut grammar, words);
    println!("Second: {}", second_answer);
}

fn extract_grammar(grammar_lines: &Vec<&str>) -> Grammar {
    let mut nonterminal_productions: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    let mut terminal_productions = HashMap::new();
    let mut highest_index = 0 as usize;

    for &line in grammar_lines {
        let line = line.split(": ").collect::<Vec<_>>();
        let nonterminal_index = line[0].parse::<usize>().unwrap();
        highest_index = std::cmp::max(highest_index, nonterminal_index);

        let productions = line[1].split(" | ");
        for production in productions {
            let bytes = production.as_bytes();
            if bytes[0] as char == '\"' {
                let terminal = bytes[1] as char;
                terminal_productions.insert(nonterminal_index, terminal);
            } else {
                let nonterminals = production
                    .split(" ")
                    .map(|index| index.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                if let Some(all_nonterminals) = nonterminal_productions.get_mut(&nonterminal_index)
                {
                    all_nonterminals.push(nonterminals);
                } else {
                    nonterminal_productions.insert(nonterminal_index, vec![nonterminals]);
                }
            }
        }
    }

    Grammar {
        terminal_productions,
        nonterminal_productions,
    }
}

fn get_first_answer(grammar: &Grammar, words: &Vec<&str>) -> u64 {
    get_answer(grammar, words)
}

fn get_second_answer(grammar: &mut Grammar, words: &Vec<&str>) -> u64 {
    let eight_rules = grammar.nonterminal_productions.get_mut(&8).unwrap();
    eight_rules.push(vec![42, 8]);
    let eleven_rules = grammar.nonterminal_productions.get_mut(&11).unwrap();
    eleven_rules.push(vec![42, 11, 31]);

    get_answer(grammar, words)
}

fn get_answer(grammar: &Grammar, words: &Vec<&str>) -> u64 {
    let mut next_nonterminals = LinkedList::new();
    next_nonterminals.push_front(0);
    words
        .iter()
        .map(|&word| get_answer_recursively(grammar, word, &mut next_nonterminals) as u64)
        .sum()
}

fn get_answer_recursively(
    grammar: &Grammar,
    word: &str,
    next_nonterminals: &mut LinkedList<usize>,
) -> bool {
    if next_nonterminals.is_empty() {
        return word.is_empty();
    }
    if word.is_empty() {
        return false;
    }

    let next = next_nonterminals.pop_front().unwrap();
    if let Some(&c) = grammar.terminal_productions.get(&next) {
        let answer = if word.as_bytes()[0] as char == c {
            get_answer_recursively(grammar, &word[1..], next_nonterminals)
        } else {
            false
        };
        next_nonterminals.push_front(next);
        return answer;
    } else if let Some(nonterminal_productions) = grammar.nonterminal_productions.get(&next) {
        let answer = nonterminal_productions.iter().any(|nonterminals| {
            nonterminals
                .iter()
                .rev()
                .for_each(|&index| next_nonterminals.push_front(index));
            let answer = get_answer_recursively(grammar, word, next_nonterminals);
            for _ in 0..nonterminals.len() {
                next_nonterminals.pop_front();
            }
            answer
        });
        next_nonterminals.push_front(next);
        return answer;
    }

    next_nonterminals.push_front(next);
    return false;
}
