#![warn(clippy::all, clippy::pedantic)]

// use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let mut rock_grid = Vec::new();

    FileReader::process_lines("./input/problem14.txt", &mut |line| {
        rock_grid.push(line.chars().collect::<Vec<_>>());
    });

    let rock_grid_clone = clone_grid(&rock_grid);

    tilt_cycle_north(&mut rock_grid);

    println!("Total load: {}", calc_total_load(&rock_grid)); // 110274

    let mut rock_grid = clone_grid(&rock_grid_clone);

    let mut tilt_cycle_state = Vec::new();
    tilt_cycle_state.push(rock_grid_clone);

    loop {
        rock_grid = tilt_cycle(rock_grid);

        if let Some(cycle_start) = tilt_cycle_state
            .iter()
            .position(|state| *state == rock_grid)
        {
            let cycle_length = tilt_cycle_state.len() - cycle_start;
            let state_after_billion = (1_000_000_000 - cycle_start) % cycle_length + cycle_start;
            let total_load = calc_total_load(&tilt_cycle_state[state_after_billion]);

            println!("Total load after billion cycles: {}", total_load); //90982

            break;
        } else {
            tilt_cycle_state.push(clone_grid(&rock_grid));
        }
    }
}

fn calc_total_load(grid: &Vec<Vec<char>>) -> usize {
    let amount_of_rows = grid.len();

    grid.iter().enumerate().fold(0, |load, (row_idx, row)| {
        let amount_of_rounded_rocks = row.iter().filter(|el| **el == 'O').count();

        load + amount_of_rounded_rocks * (amount_of_rows - row_idx)
    })
}

fn tilt_cycle(mut rock_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let amount_of_rows = rock_grid.len();
    let amount_of_columns = rock_grid[0].len(); // we assume grid is rectangular

    tilt_cycle_north(&mut rock_grid);

    // to west
    for row_idx in 0..amount_of_rows {
        for column_idx in 1..amount_of_columns {
            if rock_grid[row_idx][column_idx] == 'O' {
                let mut new_column_idx = column_idx;

                while new_column_idx > 0 {
                    let next_column_idx = new_column_idx - 1;
                    if rock_grid[row_idx][next_column_idx] == '.' {
                        new_column_idx = next_column_idx;
                    } else {
                        break;
                    }
                }

                if new_column_idx != column_idx {
                    rock_grid[row_idx][new_column_idx] = 'O';
                    rock_grid[row_idx][column_idx] = '.';
                }
            }
        }
    }

    // to south
    for row_idx in (0..amount_of_rows - 1).rev() {
        for column_idx in 0..amount_of_columns {
            if rock_grid[row_idx][column_idx] == 'O' {
                let mut new_row_idx = row_idx;

                while new_row_idx < amount_of_rows - 1 {
                    let next_row_idx = new_row_idx + 1;
                    if rock_grid[next_row_idx][column_idx] == '.' {
                        new_row_idx = next_row_idx;
                    } else {
                        break;
                    }
                }

                if new_row_idx != row_idx {
                    rock_grid[new_row_idx][column_idx] = 'O';
                    rock_grid[row_idx][column_idx] = '.';
                }
            }
        }
    }

    // to east
    for row_idx in 0..amount_of_rows {
        for column_idx in (0..amount_of_columns - 1).rev() {
            if rock_grid[row_idx][column_idx] == 'O' {
                let mut new_column_idx = column_idx;

                while new_column_idx < amount_of_columns - 1 {
                    let next_column_idx = new_column_idx + 1;
                    if rock_grid[row_idx][next_column_idx] == '.' {
                        new_column_idx = next_column_idx;
                    } else {
                        break;
                    }
                }

                if new_column_idx != column_idx {
                    rock_grid[row_idx][new_column_idx] = 'O';
                    rock_grid[row_idx][column_idx] = '.';
                }
            }
        }
    }

    clone_grid(&rock_grid)
}

fn tilt_cycle_north(rock_grid: &mut Vec<Vec<char>>) {
    let amount_of_rows = rock_grid.len();
    let amount_of_columns = rock_grid[0].len(); // we assume grid is rectangular

    // to north
    for row_idx in 1..amount_of_rows {
        for column_idx in 0..amount_of_columns {
            if rock_grid[row_idx][column_idx] == 'O' {
                let mut new_row_idx = row_idx;

                while new_row_idx > 0 {
                    let next_row_idx = new_row_idx - 1;
                    if rock_grid[next_row_idx][column_idx] == '.' {
                        new_row_idx = next_row_idx;
                    } else {
                        break;
                    }
                }

                if new_row_idx != row_idx {
                    rock_grid[new_row_idx][column_idx] = 'O';
                    rock_grid[row_idx][column_idx] = '.';
                }
            }
        }
    }
}

fn clone_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().map(|row| row.clone()).collect::<Vec<_>>()
}
