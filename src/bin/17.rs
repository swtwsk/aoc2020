use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Write};
use std::fs;

#[derive(Clone, Copy)]
enum FieldState {
    Active,
    Inactive,
}

impl FieldState {
    fn to_int(&self) -> u64 {
        match self {
            FieldState::Active => 1,
            FieldState::Inactive => 0,
        }
    }
}

impl Debug for FieldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            FieldState::Active => '#',
            FieldState::Inactive => '.',
        })
    }
}

type CoordTuple = (i64, i64, i64, i64);
type Grid = HashMap<CoordTuple, FieldState>;

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 17 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.strip_suffix("\n").unwrap().split("\n").collect();
    let grid = extract_grid(lines);

    let first_answer = get_first_answer(&grid);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&grid);
    println!("Second: {}", second_answer);
}

fn extract_grid(lines: Vec<&str>) -> Grid {
    lines
        .iter()
        .enumerate()
        .flat_map(|(i, &line)| {
            line.char_indices().map(move |(j, c)| {
                let coordinates: CoordTuple = (j as i64, i as i64, 0, 0);
                if c == '#' {
                    (coordinates, FieldState::Active)
                } else {
                    (coordinates, FieldState::Inactive)
                }
            })
        })
        .collect()
}

fn get_first_answer(grid: &Grid) -> u64 {
    // Monadic list-comprehension at its best
    // The following chain of maps and flat_maps is equivalent
    // to [(x, y, z, 0) | x <- [-1..1], y <- [-1..1], z <- [-1..1]]
    let coords_modifiers = (-1..2)
        .flat_map(|x| (-1..2).flat_map(move |y| (-1..2).map(move |z| (x, y, z, 0))))
        .filter(|quadruple| !is_zero(quadruple))
        .collect::<Vec<_>>();
    get_answer(grid, coords_modifiers)
}

fn get_second_answer(grid: &Grid) -> u64 {
    let coords_modifiers = (-1..2)
        .flat_map(|x| {
            (-1..2).flat_map(move |y| (-1..2).flat_map(move |z| (-1..2).map(move |w| (x, y, z, w))))
        })
        .filter(|quadruple| !is_zero(quadruple))
        .collect::<Vec<_>>();
    get_answer(grid, coords_modifiers)
}

fn get_answer(grid: &Grid, coords_modifiers: Vec<CoordTuple>) -> u64 {
    let mut current_grid = grid.clone();

    for _ in 1..7 {
        let next_grid = current_grid
            .keys()
            .flat_map(|coords| {
                coords_modifiers
                    .iter()
                    .map(move |modifier| add_tuples(coords, modifier))
            })
            .map(|new_coords| {
                let neighbours = coords_modifiers
                    .iter()
                    .map(|modifier| add_tuples(&new_coords, modifier));
                let active_neighbours_counter =
                    neighbours.fold(0, |acc, coords| match current_grid.get(&coords) {
                        Some(state) => acc + state.to_int(),
                        None => acc,
                    });
                let new_state = match current_grid
                    .get(&new_coords)
                    .unwrap_or(&FieldState::Inactive)
                {
                    FieldState::Active => {
                        if active_neighbours_counter == 2 || active_neighbours_counter == 3 {
                            FieldState::Active
                        } else {
                            FieldState::Inactive
                        }
                    }
                    FieldState::Inactive => {
                        if active_neighbours_counter == 3 {
                            FieldState::Active
                        } else {
                            FieldState::Inactive
                        }
                    }
                };
                (new_coords, new_state)
            })
            .collect::<HashMap<_, _>>();

        current_grid = next_grid;
    }

    current_grid.values().map(|field| field.to_int()).sum()
}

fn add_tuples((x1, y1, z1, w1): &CoordTuple, (x2, y2, z2, w2): &CoordTuple) -> CoordTuple {
    (x1 + x2, y1 + y2, z1 + z2, w1 + w2)
}

fn is_zero(&(x, y, z, w): &CoordTuple) -> bool {
    x == 0 && y == 0 && z == 0 && w == 0
}
