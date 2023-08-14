#![warn(clippy::all, clippy::pedantic)]

use utils::file_reader::FileReader;

const GRID_SIDE_SIZE: usize = 100;

type LightGrid = Box<[[bool; GRID_SIDE_SIZE]; GRID_SIDE_SIZE]>;

fn main() {
    let mut lights_grid_part_1 = Box::new([[false; GRID_SIDE_SIZE]; GRID_SIDE_SIZE]);
    let mut lights_grid_part_2 = Box::new([[false; GRID_SIDE_SIZE]; GRID_SIDE_SIZE]);
    let mut counter = 0;

    FileReader::process_lines("./input/problem_18.txt", &mut |line| {
        assert_eq!(line.len(), 100);

        for (index, char) in line.chars().enumerate() {
            assert!(char == '#' || char == '.');
            lights_grid_part_1[counter][index] = char == '#';
            lights_grid_part_2[counter][index] = char == '#';
        }

        counter += 1;
    });

    assert!(counter == 100);

    lights_grid_part_2[0][0] = true;
    lights_grid_part_2[0][GRID_SIDE_SIZE - 1] = true;
    lights_grid_part_2[GRID_SIDE_SIZE - 1][0] = true;
    lights_grid_part_2[GRID_SIDE_SIZE - 1][GRID_SIDE_SIZE - 1] = true;

    for _ in 0..100 {
        lights_grid_part_1 = next_light_grid(lights_grid_part_1);
        lights_grid_part_2 = next_light_grid(lights_grid_part_2);

        lights_grid_part_2[0][0] = true;
        lights_grid_part_2[0][GRID_SIDE_SIZE - 1] = true;
        lights_grid_part_2[GRID_SIDE_SIZE - 1][0] = true;
        lights_grid_part_2[GRID_SIDE_SIZE - 1][GRID_SIDE_SIZE - 1] = true;
    }

    let lights_on_part_1 = amount_of_lights_on(&lights_grid_part_1);
    let lights_on_part_2 = amount_of_lights_on(&lights_grid_part_2);

    println!(
        "Part 1, lights on after 100 iterations: {}",
        lights_on_part_1 // 814
    );

    println!(
        "Part 2, lights on after 100 iterations: {}",
        lights_on_part_2 // 924
    );
}

fn amount_of_lights_on(grid: &LightGrid) -> usize {
     grid.iter().flatten().filter(|x| **x).count()
}

fn next_light_grid(light_grid: LightGrid) -> LightGrid {
    let mut next_grid = Box::new([[false; GRID_SIDE_SIZE]; GRID_SIDE_SIZE]);

    for i in 0..GRID_SIDE_SIZE {
        for j in 0..GRID_SIDE_SIZE {
            let mut amount_of_lit_neightbourhs = 0;

            let prev_i = if i > 0 { i - 1 } else { 0 };
            let next_i = if i < GRID_SIDE_SIZE - 1 { i + 1 } else { i };

            for k in prev_i..=next_i {
                let prev_j = if j > 0 { j - 1 } else { 0 };
                let next_j = if j < GRID_SIDE_SIZE - 1 { j + 1 } else { j };

                for l in prev_j..=next_j {
                    if k == i && l == j {
                        continue;
                    }

                    if light_grid[k][l] {
                        amount_of_lit_neightbourhs += 1;
                    }
                }
            }

            if light_grid[i][j] {
                next_grid[i][j] = amount_of_lit_neightbourhs == 2 || amount_of_lit_neightbourhs == 3;
            } else {
                next_grid[i][j] = amount_of_lit_neightbourhs == 3;
            }
        }
    }

    next_grid
}
