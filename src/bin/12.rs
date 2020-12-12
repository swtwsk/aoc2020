use std::env;
use std::fs;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_action(&self, value: i64) -> Action {
        match self {
            Direction::North => Action::North(value),
            Direction::South => Action::South(value),
            Direction::East => Action::East(value),
            Direction::West => Action::West(value),
        }
    }

    fn rotate(&mut self, steps: i64, is_left: bool) {
        let steps = steps % 4;
        let directions = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let index = directions
            .iter()
            .position(|direction| direction.eq(self))
            .unwrap();

        let new_index = if is_left {
            let mut new_index = (index as i64 - steps) % directions.len() as i64;
            if new_index < 0 {
                new_index = directions.len() as i64 + new_index
            }
            new_index as usize
        } else {
            (index + steps as usize) % directions.len()
        };

        let new_direction = directions[new_index];
        *self = new_direction;
    }
}

struct FerryState {
    north: i64,
    east: i64,
    current_face: Direction,
}

impl FerryState {
    fn new() -> FerryState {
        FerryState {
            north: 0,
            east: 0,
            current_face: Direction::East,
        }
    }

    fn take_action(&mut self, action: &Action) {
        let rotation_from_degree = |degree| (degree % 360) as i64 / 90;

        match action {
            Action::North(v) => self.north += v,
            Action::South(v) => self.north -= v,
            Action::East(v) => self.east += v,
            Action::West(v) => self.east -= v,
            Action::Left(degree) => {
                let steps = rotation_from_degree(degree);
                self.current_face.rotate(steps, true);
            }
            Action::Right(degree) => {
                let steps = rotation_from_degree(degree);
                self.current_face.rotate(steps, false);
            }
            Action::Forward(v) => self.take_action(&self.current_face.to_action(*v)),
        }
    }
}

enum Action {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl Action {
    fn new(direction: char, value: i64) -> Action {
        match direction {
            'N' => Action::North(value),
            'S' => Action::South(value),
            'E' => Action::East(value),
            'W' => Action::West(value),
            'L' => Action::Left(value),
            'R' => Action::Right(value),
            'F' => Action::Forward(value),
            _ => panic!("Couldn't parse action with given direction: {}", direction),
        }
    }
}

fn tuple_bimap<A, B, C, D, F, G>((a, b): (A, B), f: F, g: G) -> (C, D)
where
    F: Fn(A) -> C,
    G: Fn(B) -> D,
{
    (f(a), g(b))
}

struct FerryWaypointState {
    ferry_position: (i64, i64),
    waypoint_position: (i64, i64),
}

impl FerryWaypointState {
    fn new() -> FerryWaypointState {
        FerryWaypointState {
            ferry_position: (0, 0),
            waypoint_position: (1, 10),
        }
    }

    fn take_action(&mut self, action: &Action) {
        let rotate_waypoint = |degree, (x, y)| {
            let degree = (degree as f64).to_radians();
            let sin_value = degree.sin() as i64;
            let cos_value = degree.cos() as i64;

            (x * cos_value - y * sin_value, x * sin_value + y * cos_value)
        };

        let id = |x| x;

        match action {
            Action::North(v) => {
                self.waypoint_position = tuple_bimap(self.waypoint_position, |x| x + v, id)
            }
            Action::South(v) => {
                self.waypoint_position = tuple_bimap(self.waypoint_position, |x| x - v, id)
            }
            Action::East(v) => {
                self.waypoint_position = tuple_bimap(self.waypoint_position, id, |x| x + v)
            }
            Action::West(v) => {
                self.waypoint_position = tuple_bimap(self.waypoint_position, id, |x| x - v)
            }
            Action::Left(degree) => {
                self.waypoint_position = rotate_waypoint(-degree, self.waypoint_position)
            }
            Action::Right(degree) => {
                self.waypoint_position = rotate_waypoint(*degree, self.waypoint_position)
            }
            Action::Forward(v) => {
                let (north_waypoint, east_waypoint) = self.waypoint_position;
                self.ferry_position = tuple_bimap(
                    self.ferry_position,
                    |n| n + v * north_waypoint,
                    |e| e + v * east_waypoint,
                );
            }
        }
    }
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(filename) => filename,
        None => panic!("Usage: 12 <input_file>"),
    };

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines = contents.split("\n").collect();
    let actions = convert_line_to_actions(lines);

    let first_answer = get_first_answer(&actions);
    println!("First: {}", first_answer);

    let second_answer = get_second_answer(&actions);
    println!("Second: {}", second_answer);
}

fn convert_line_to_actions(lines: Vec<&str>) -> Vec<Action> {
    lines
        .iter()
        .filter(|&&line| line.len() > 1)
        .map(|&line| {
            let value = (&line[1..]).parse().unwrap();
            Action::new(line.chars().nth(0).unwrap(), value)
        })
        .collect()
}

fn get_first_answer(actions: &Vec<Action>) -> i64 {
    let mut ferry_state = FerryState::new();
    actions
        .iter()
        .for_each(|action| ferry_state.take_action(action));
    ferry_state.north.abs() + ferry_state.east.abs()
}

fn get_second_answer(actions: &Vec<Action>) -> i64 {
    let mut ferry_waypoint_state = FerryWaypointState::new();
    actions
        .iter()
        .for_each(|action| ferry_waypoint_state.take_action(action));
    let (position_north, position_east) = ferry_waypoint_state.ferry_position;
    position_north.abs() + position_east.abs()
}
