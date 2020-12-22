use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

type Deck = VecDeque<u64>;

enum RoundWinner {
    PlayerOne,
    PlayerTwo,
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 22 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let contents = contents
        .strip_suffix("\n")
        .unwrap()
        .split("\n\n")
        .map(|player_deck| {
            player_deck
                .split("\n")
                .skip(1)
                .map(|line| line.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>()
        })
        .collect::<Vec<_>>();

    let player_one_deck = &contents[0];
    let player_two_deck = &contents[1];

    let first_answer = get_first_answer(player_one_deck, player_two_deck);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(player_one_deck, player_two_deck);
    println!("Second: {}", second_answer);
}

fn get_first_answer(player_one_deck: &Deck, player_two_deck: &Deck) -> u64 {
    let mut player_one_deck = player_one_deck.clone();
    let mut player_two_deck = player_two_deck.clone();

    while !player_one_deck.is_empty() && !player_two_deck.is_empty() {
        let top_one = player_one_deck.pop_front().unwrap();
        let top_second = player_two_deck.pop_front().unwrap();

        if top_one > top_second {
            player_one_deck.push_back(top_one);
            player_one_deck.push_back(top_second);
        } else {
            player_two_deck.push_back(top_second);
            player_two_deck.push_back(top_one);
        }
    }

    (if player_one_deck.is_empty() {
        player_two_deck
    } else {
        player_one_deck
    })
    .iter()
    .rev()
    .fold((0, 1), |(acc, mult), card| (acc + card * mult, mult + 1))
    .0
}

fn get_second_answer(player_one_deck: &Deck, player_two_deck: &Deck) -> u64 {
    let player_one_deck = player_one_deck.clone();
    let player_two_deck = player_two_deck.clone();

    get_second_recursive(player_one_deck, player_two_deck, 1)
        .0
        .iter()
        .rev()
        .fold((0, 1), |(acc, mult), card| (acc + card * mult, mult + 1))
        .0
}

fn get_second_recursive(
    mut player_one_deck: Deck,
    mut player_two_deck: Deck,
    round: u64,
) -> (Deck, RoundWinner) {
    let mut previous_decks = HashSet::new();

    loop {
        if player_one_deck.is_empty() {
            return (player_two_deck, RoundWinner::PlayerTwo);
        }
        if player_two_deck.is_empty() {
            return (player_one_deck, RoundWinner::PlayerOne);
        }

        if previous_decks
            .get(&(player_one_deck.clone(), player_two_deck.clone()))
            .is_some()
        {
            return (player_one_deck, RoundWinner::PlayerOne);
        }

        previous_decks.insert((player_one_deck.clone(), player_two_deck.clone()));

        let top_one = player_one_deck.pop_front().unwrap();
        let top_second = player_two_deck.pop_front().unwrap();

        let winner = if player_one_deck.len() as u64 >= top_one
            && player_two_deck.len() as u64 >= top_second
        {
            let player_one_deck = player_one_deck
                .iter()
                .take(top_one as usize)
                .cloned()
                .collect();
            let player_two_deck = player_two_deck
                .iter()
                .take(top_second as usize)
                .cloned()
                .collect();

            get_second_recursive(player_one_deck, player_two_deck, round + 1).1
        } else if top_one > top_second {
            RoundWinner::PlayerOne
        } else {
            RoundWinner::PlayerTwo
        };

        match winner {
            RoundWinner::PlayerOne => {
                player_one_deck.push_back(top_one);
                player_one_deck.push_back(top_second);
            }
            RoundWinner::PlayerTwo => {
                player_two_deck.push_back(top_second);
                player_two_deck.push_back(top_one);
            }
        }
    }
}
