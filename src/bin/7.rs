use std::fs;
use std::{
    collections::{HashMap, HashSet},
    env,
};

type NeighbourTuple = (u32, String);
type GraphMap = HashMap<String, Vec<NeighbourTuple>>;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 7 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n");

    let graph_map = create_graph_map(lines);
    let first_answer = get_first_answer(&graph_map);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&graph_map);
    println!("Second: {}", second_answer);
}

fn create_graph_map<'a, I>(lines: I) -> GraphMap
where
    I: Iterator<Item = &'a str>,
{
    let mut graph_map = HashMap::new();

    for line in lines {
        let line = line.split(" ").collect::<Vec<_>>();

        if line.len() == 1 {
            continue;
        }

        let key = format!("{} {}", line[0], line[1]);
        if line[4] == "no" {
            graph_map.insert(key, vec![]);
            continue;
        }

        let mut value = vec![];
        for index in (4..line.len()).step_by(4) {
            let value_str = format!("{} {}", line[index + 1], line[index + 2]);
            value.push((line[index].parse::<u32>().unwrap(), value_str));
        }
        graph_map.insert(key, value);
    }

    graph_map
}

fn get_first_answer(graph_map: &GraphMap) -> u32 {
    let mut relationship_map = HashMap::new();
    let mut visited_set = HashSet::new();

    for key in graph_map.keys() {
        dfs_first_answer(key, graph_map, &mut relationship_map, &mut visited_set);
    }

    relationship_map
        .iter()
        .map(|(_, &el)| el)
        .fold(0, |acc, el| acc + if el { 1 } else { 0 })
}

fn dfs_first_answer(
    current_node: &String,
    graph_map: &GraphMap,
    relationship_map: &mut HashMap<String, bool>,
    visited_set: &mut std::collections::HashSet<std::string::String>,
) {
    visited_set.insert(current_node.clone());

    let mut can_contains = match relationship_map.get(current_node) {
        Some(val) => val.clone(),
        None => false,
    };

    for (_, name) in graph_map.get(current_node).unwrap() {
        if !visited_set.contains(name) {
            dfs_first_answer(name, graph_map, relationship_map, visited_set);
        }

        match relationship_map.get(name) {
            Some(val) => {
                can_contains |= val;
            }
            None => {}
        }

        if name == "shiny gold" {
            can_contains = true;
        }
    }

    relationship_map.insert(current_node.clone(), can_contains);
}

fn get_second_answer(graph_map: &GraphMap) -> u32 {
    dfs_second_answer(&"shiny gold".to_string(), graph_map)
}

fn dfs_second_answer(current_node: &String, graph_map: &GraphMap) -> u32 {
    graph_map
        .get(current_node)
        .unwrap()
        .iter()
        .fold(0, |acc, (count, neighbour_name)| {
            acc + count * (1 + dfs_second_answer(neighbour_name, graph_map))
        })
}
