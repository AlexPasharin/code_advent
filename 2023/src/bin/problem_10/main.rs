#![warn(clippy::all, clippy::pedantic)]

use core::panic;
use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use utils::file_reader::FileReader;

type Location = (usize, usize);

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

use Direction::*;

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            North => South,
            South => North,
            West => East,
            East => West,
        }
    }
}

fn main() {
    // let mut pipe_grid = Vec::new();
    let mut pipe_grid = HashMap::new();

    let mut starting_position = None;

    let mut row_index: usize = 0;

    FileReader::process_lines("./input/problem10.txt", &mut |line| {
        for (column_idx, ch) in line.chars().enumerate() {
            let coordinates = (column_idx, row_index);

            if ch == 'S' {
                if starting_position.is_some() {
                    panic!("Starting position must be unique");
                }

                starting_position = Some(coordinates);
            } else if ch != '.' {
                pipe_grid.insert(coordinates, ch);
            }
        }

        row_index += 1;
    });

    let starting_position = starting_position.unwrap();

    let neighbourhs = |coordinates: &(usize, usize),
                       directions: &[Direction],
                       pipe_grid: &HashMap<(usize, usize), char>| {
        directions
            .iter()
            .filter_map(|direction| {
                let ngh_point = location_ngh(coordinates, direction)?;
                let pipe_char = pipe_grid.get(&ngh_point)?;
                let pipe_part_directions = pipe_ch_to_directions(pipe_char);

                if pipe_part_directions.contains(&direction.opposite()) {
                    Some((*direction, ngh_point))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    let starting_position_nghs =
        neighbourhs(&starting_position, &[North, South, West, East], &pipe_grid);

    assert_eq!(starting_position_nghs.len(), 2);

    let mut starting_position_directions_iter = starting_position_nghs.into_iter();
    let starting_position_directions = [
        starting_position_directions_iter.next().unwrap(),
        starting_position_directions_iter.next().unwrap(),
    ];

    let mut previous_position = starting_position;
    let mut next_position = starting_position_directions[0].1;

    pipe_grid.insert(
        starting_position,
        directions_to_pipe_ch(&starting_position_directions.map(|(d, _)| d)),
    );

    let mut cycle_length = 0;
    let mut visited = HashMap::new();

    while next_position != starting_position {
        cycle_length += 1;

        let directions = pipe_ch_to_directions(pipe_grid.get(&next_position).unwrap());

        let next_position_nghs = neighbourhs(&next_position, &directions, &pipe_grid);

        assert_eq!(next_position_nghs.len(), 2);

        let previous_next_position = next_position;

        next_position = next_position_nghs
            .into_iter()
            .find(|(_, ngh)| *ngh != previous_position)
            .unwrap()
            .1;

        previous_position = previous_next_position;

        visited.insert(next_position, *pipe_grid.get(&next_position).unwrap());
    }

    // actually it does not anymore, since visited is now hashmap, not a vector
    //assert_eq!(cycle_length, visited.len()); // this implies that cycle does not intersect itself

    println!("Furthest cycle distance: {}", (cycle_length + 1) / 2); // 6690

    drop(pipe_grid); // we dont need the whole grid anymore, lets save some memory

    let (min_col_idx, max_col_idx) = get_min_max(visited.keys().map(|(column_idx, _)| *column_idx));
    let (min_row_idx, max_row_idx) = get_min_max(visited.keys().map(|(_, row_idx)| *row_idx));

    let mut amount_of_inner_points = 0;

    for row_idx in min_row_idx..=max_row_idx {
        let mut intersection_count = 0;
        let mut latest_direction = None;

        for column_idx in min_col_idx..=max_col_idx {
            let coordinates = (column_idx, row_idx);

            match visited.get(&coordinates) {
                Some(pipe_ch) => {
                    match pipe_ch {
                        '|' => intersection_count += 1,
                        '-' => {}
                        'L' => {
                            latest_direction = Some(North);
                        }
                        'J' => {
                            if latest_direction.unwrap() == South {
                                intersection_count += 1;
                            }
                        }
                        '7' => {
                            if latest_direction.unwrap() == North {
                                intersection_count += 1;
                            }
                        }
                        'F' => {
                            latest_direction = Some(South);
                        }
                        _ => panic!("Unknown pipe grid symbol"),
                    };
                }
                None => {
                    if intersection_count % 2 == 1 {
                        amount_of_inner_points += 1;
                    }
                }
            }
        }
    }

    println!("Amount of inner points: {}", amount_of_inner_points);
}

fn pipe_ch_to_directions(pipe_ch: &char) -> [Direction; 2] {
    match pipe_ch {
        '|' => [North, South],
        '-' => [West, East],
        'L' => [North, East],
        'J' => [North, West],
        '7' => [South, West],
        'F' => [South, East],
        _ => panic!("Unknown pipe grid symbol"),
    }
}

fn directions_to_pipe_ch(directions: &[Direction; 2]) -> char {
    match directions {
        [North, South] => '|',
        [West, East] => '-',
        [North, East] => 'L',
        [North, West] => 'J',
        [South, West] => '7',
        [South, East] => 'F',
        x => panic!("Unknown pipe grid symbol: {:?}", x),
    }
}

fn location_ngh(location: &Location, direction: &Direction) -> Option<Location> {
    let (column_idx, row_idx) = location;

    match direction {
        North => {
            if *row_idx > 0 {
                Some((*column_idx, row_idx - 1))
            } else {
                None
            }
        }
        South => Some((*column_idx, row_idx + 1)),
        West => {
            if *column_idx > 0 {
                Some((column_idx - 1, *row_idx))
            } else {
                None
            }
        }
        East => Some((column_idx + 1, *row_idx)),
    }
}

// "unsafe" min max to use when you know iterator is not empty
fn get_min_max(iter: impl Iterator<Item = usize>) -> (usize, usize) {
    match iter.minmax() {
        itertools::MinMaxResult::OneElement(x) => (x, x),
        itertools::MinMaxResult::MinMax(x, y) => (x, y),
        itertools::MinMaxResult::NoElements => unreachable!(),
    }
}
