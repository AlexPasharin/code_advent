#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

// use regex::Regex;

use utils::file_reader::FileReader;

type Location = (i32, i32);

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

use Direction::*;
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct BeamState {
    location: Location,
    direction: Direction,
}

fn main() {
    let mut grid = Vec::new();

    FileReader::process_lines("./input/problem16.txt", &mut |line| {
        grid.push(line.chars().collect::<Vec<_>>());
    });

    let amount_of_rows = grid.len() as i32;
    let amount_of_columns = grid[0].len() as i32;

    let calc_energized_tile_amount = |starting_state: BeamState| {
        let mut visited_locations: HashSet<BeamState> = HashSet::new();
        let mut beams = Vec::new();

        beams.push(starting_state);

        while !beams.is_empty() {
            beams = beams
                .into_iter()
                .filter_map(|beam_state| {
                    if visited_locations.contains(&beam_state) {
                        return None;
                    }

                    let BeamState {
                        location,
                        direction,
                    } = beam_state;

                    let (row_idx, column_idx) = location;

                    if row_idx < 0
                        || column_idx < 0
                        || row_idx >= amount_of_rows
                        || column_idx >= amount_of_columns
                    {
                        return None;
                    }

                    visited_locations.insert(beam_state);

                    let move_in_direction = move |direction: Direction| {
                        let (row_idx, column_idx) = location;

                        let next_location = match direction {
                            Left => (row_idx, column_idx - 1),
                            Right => (row_idx, column_idx + 1),
                            Down => (row_idx + 1, column_idx),
                            Up => (row_idx - 1, column_idx),
                        };

                        BeamState {
                            location: next_location,
                            direction,
                        }
                    };

                    let tile = grid[row_idx as usize][column_idx as usize];

                    let next_directions = match tile {
                        '.' => vec![direction],
                        '/' => {
                            let next_direction = match direction {
                                Left => Down,
                                Right => Up,
                                Down => Left,
                                Up => Right,
                            };

                            vec![next_direction]
                        }
                        '\\' => {
                            let next_direction = match direction {
                                Left => Up,
                                Right => Down,
                                Down => Right,
                                Up => Left,
                            };

                            vec![next_direction]
                        }
                        '|' => match direction {
                            Up | Down => vec![direction],
                            Left | Right => vec![Up, Down],
                        },
                        '-' => match direction {
                            Left | Right => vec![direction],
                            Up | Down => vec![Left, Right],
                        },
                        _ => {
                            panic!("Unknown symbol: {}", tile)
                        }
                    };

                    Some(
                        next_directions
                            .into_iter()
                            .map(move |direction| move_in_direction(direction)),
                    )
                })
                .flatten()
                .collect();
        }

        let energized_tiles = visited_locations
            .into_iter()
            .map(|state| state.location)
            .collect::<HashSet<_>>();

        energized_tiles.len()
    };

    let starting_state = BeamState {
        location: (0, 0),
        direction: Right,
    };

    let energized_tile_amount = calc_energized_tile_amount(starting_state);

    println!(
        "Amount of energized tiles, starging from top-left: {}",
        energized_tile_amount
    ); // 6921

    // here we start from 1, because case 0 is already calculated for part 1
    let left_edge_starting_states = (1..amount_of_rows).map(|idx| BeamState {
        location: (idx, 0),
        direction: Right,
    });

    let upper_edge_starting_states = (0..amount_of_columns).map(|idx| BeamState {
        location: (0, idx),
        direction: Down,
    });

    let right_edge_starting_states = (0..amount_of_rows).map(|idx| BeamState {
        location: (idx, amount_of_columns - 1),
        direction: Left,
    });

    let lower_edge_starting_states = (0..amount_of_columns).map(|idx| BeamState {
        location: (amount_of_rows - 1, idx),
        direction: Up,
    });

    let starting_states = left_edge_starting_states
        .chain(upper_edge_starting_states)
        .chain(right_edge_starting_states)
        .chain(lower_edge_starting_states);

    let most_energized_tiles = starting_states.fold(
        energized_tile_amount,
        |current_max_amount, starting_state| {
            let energized_tile_amount = calc_energized_tile_amount(starting_state);

            current_max_amount.max(energized_tile_amount)
        },
    );

    println!("Most energized amount: {}", most_energized_tiles); // 7594
}
