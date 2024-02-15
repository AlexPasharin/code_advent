#![warn(clippy::all, clippy::pedantic)]

use std::collections::{BinaryHeap, HashMap};

use utils::file_reader::FileReader;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

use Direction::*;

#[derive(PartialEq, Eq)]
struct Edge {
    position: (usize, usize),
    direction: Direction,
    distance: u32,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut grid = Vec::new();

    FileReader::process_lines("./input/problem17.txt", &mut |line| {
        grid.push(
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
    });

    let amount_of_rows = grid.len();
    let amount_of_columns = grid[0].len();

    let starting_position = (0 as usize, 0 as usize);
    let destination_position = (amount_of_columns - 1, amount_of_rows - 1);

    let next_directions = |position: (usize, usize), direction: Direction| {
        let mut directions = Vec::new();

        let (x, y) = position;

        match direction {
            Left | Right => {
                if y > 0 {
                    directions.push(Up);
                }

                if y < amount_of_rows - 1 {
                    directions.push(Down)
                }
            }
            Up | Down => {
                if x > 0 {
                    directions.push(Left);
                }

                if x < amount_of_columns - 1 {
                    directions.push(Right)
                }
            }
        }

        directions
    };

    let solve = |min_amount_of_steps, max_amount_of_steps| {
        let mut queue = BinaryHeap::new();

        queue.push(Edge {
            position: starting_position,
            direction: Right,
            distance: 0,
        });

        queue.push(Edge {
            position: starting_position,
            direction: Down,
            distance: 0,
        });

        let mut visited = HashMap::new();

        visited.insert((starting_position, Right), 0);
        visited.insert((starting_position, Down), 0);

        while let Some(Edge {
            position,
            direction,
            distance,
        }) = queue.pop()
        {
            if position == destination_position {
                return Some(distance);
            }

            if visited
                .get(&(position, direction))
                .is_some_and(|&best_distance| best_distance < distance)
            {
                continue;
            }

            let (x, y) = position;
            let directions = next_directions(position, direction);

            let mut cost = 0;

            for step in 1..=max_amount_of_steps {
                let next_position = match direction {
                    Left => {
                        if x < step {
                            break;
                        }

                        (x - step, y)
                    }
                    Right => {
                        let next_x = x + step;

                        if next_x >= amount_of_columns {
                            break;
                        }

                        (next_x, y)
                    }
                    Up => {
                        if y < step {
                            break;
                        }

                        (x, y - step)
                    }
                    Down => {
                        let next_y = y + step;

                        if next_y >= amount_of_rows {
                            break;
                        }

                        (x, next_y)
                    }
                };

                let (next_x, next_y) = next_position;

                cost += grid[next_x][next_y];

                if step >= min_amount_of_steps {
                    let next_distance = distance + cost;

                    for dir in &directions {
                        let next_direction = *dir;
                        let should_update = match visited.get(&(next_position, next_direction)) {
                            None => true,
                            Some(&distance) => next_distance < distance,
                        };

                        if should_update {
                            visited.insert((next_position, next_direction), next_distance);
                            queue.push(Edge {
                                position: next_position,
                                direction: next_direction,
                                distance: next_distance,
                            });
                        }
                    }
                }
            }
        }

        None
    };

    let standard_crucible_min_heatloss = solve(1, 3).unwrap();
    let ultra_crucible_min_heatloss = solve(4, 10).unwrap();

    println!(
        "Minimal heat loss, part 1: {}",
        standard_crucible_min_heatloss // 1076
    );

    println!("Minimal heat loss, part 2: {}", ultra_crucible_min_heatloss); // 1219
}
