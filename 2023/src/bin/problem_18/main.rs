#![warn(clippy::all, clippy::pedantic)]

// use itertools::{EitherOrBoth, Itertools};
use std::{collections::HashMap, iter::once};

use regex::Regex;
use utils::file_reader::FileReader;

#[derive(Debug, PartialEq, Eq)]
enum DigDirection {
    Right,
    Left,
    Up,
    Down,
}

use DigDirection::*;

// #[derive(Clone, Copy)]
// enum EdgeType {
//     UpRight,
//     DownRight,
//     LeftUp,
//     LeftDown,
// }
// use EdgeType::*;

struct Instruction {
    dig_direction: DigDirection,
    dig_amount: u32,
}

#[derive(Debug)]
struct Part {
    // part_increases_intersection_count: bool,
    range: (i64, i64),
}

fn main() {
    let regex = Regex::new(r"^(R|L|U|D) (\d+) (?:\(#([0-9a-f]{6})\))$").unwrap();

    let mut instructions = Vec::new();
    let mut instructions_2 = Vec::new();

    FileReader::process_lines("./input/problem18.txt", &mut |line| {
        let captures = regex.captures(line).expect(line);

        let dig_direction = match &captures[1] {
            "R" => Right,
            "L" => Left,
            "U" => Up,
            "D" => Down,
            _ => unreachable!(),
        };

        let dig_amount = captures[2].parse::<u32>().unwrap();
        assert_ne!(dig_amount, 0);
        let color = &captures[3];
        let dig_amount_2 = u32::from_str_radix(&color[0..color.len() - 1], 16)
            .expect(&format!("Unknown hex {color}",));

        assert_ne!(dig_amount_2, 0);

        let dig_direction_2 = match color.chars().last().unwrap() {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => unreachable!(),
        };

        instructions.push(Instruction {
            dig_direction,
            dig_amount,
        });

        instructions_2.push(Instruction {
            dig_direction: dig_direction_2,
            dig_amount: dig_amount_2,
        })
    });

    println!(
        "Digged out {} points, part 1, using shoelace formula",
        shoelace_and_pick(&instructions)
    ); // 52055

    // println!(
    //     "Digged out {} points, part 1, using ray casting",
    //     ray_casting(&instructions),
    // ); // 52055

    // shoelace and pick solution is fast even for part 2
    // this however does not check the path we get satisfies the assumptions of the shoelace and pick'e i.e. does not intersect itself!
    println!(
        "Digged out {} points, part 2, using shoelace formula",
        shoelace_and_pick(&instructions_2)
    ); // 67622758357096

    // solution using relatively "smart" implementation of ray casting algorithm still slow for part 2, does return in a minute or so in release mode, but probably there should be faster solution
    // println!(
    //     "Digged out {} points, part 2, using ray casting",
    //     ray_casting(&instructions_2)
    // ); // 67622758357096

    // check that polygon is non-intersecting, since Pick's theorem requires that - this is relatively slow for instructions_2 (takes about a minute)
    check(&instructions);
    check(&instructions_2);
}

/*
  Shoelace formula - suppose polygon's edge points are (x_i, y_i), i=1,...n, ordered (counter) clockwise
  Then the area of polygon is A = \sum_{i=1^n} x_i(y_{i+1} - y_{y-1}) where (x_0, y_0) =(x_n, y_n) (modulo sign)

  Pick's theorem - it is also true that if x_i, y_i are all integers then A = i + b / 2 -1, where i is the amount of "interior" integer-coordinate points and b is the amount of all boundary points

  Hence we can deduce that i + b = A + b / 2 + 1, where A can be computed by the shoelace formula and b can be easily computed as well
*/

fn shoelace_and_pick(instructions: &Vec<Instruction>) -> i64 {
    let starting_location = (0, 0);

    let (amount_of_boundary_points, vertices, last_location) = instructions.iter().fold(
        (0, Vec::with_capacity(instructions.len()), starting_location),
        |acc,
         Instruction {
             dig_direction,
             dig_amount,
         }| {
            let (amount_of_boundary_points, mut vertices, location) = acc;

            vertices.push(location);

            let (x, y) = location;
            let dig_amount = *dig_amount as i64;

            let next_location = match dig_direction {
                Right => (x + dig_amount, y),
                Left => (x - dig_amount, y),
                Down => (x, y + dig_amount),
                Up => (x, y - dig_amount),
            };

            (
                amount_of_boundary_points + dig_amount,
                vertices,
                next_location,
            )
        },
    );

    assert_eq!(last_location, starting_location); // check that we indeed get a loop

    let next_vertices_iter = vertices
        .iter()
        .skip(1)
        .chain(once(vertices.first().unwrap()));

    let prev_vertices_iter = once(vertices.last().unwrap()).chain(vertices.iter());

    let double_area = (vertices.iter().zip(next_vertices_iter))
        .zip(prev_vertices_iter)
        .map(|((cur_vertex, next_vertex), prev_vertex)| {
            cur_vertex.0 * (next_vertex.1 - prev_vertex.1)
        })
        .sum::<i64>();

    (double_area.abs() + amount_of_boundary_points as i64) / 2 + 1
}

fn check(instructions: &Vec<Instruction>) {
    let mut parts = HashMap::new();
    let starting_location = (0, 0);
    let mut current_location = starting_location;

    for instruction in instructions {
        let Instruction {
            dig_direction,
            dig_amount,
        } = instruction;

        let dig_amount = *dig_amount as i64;
        let (x, y) = current_location;
        let delta = match dig_direction {
            Right | Down => dig_amount,
            Left | Up => -dig_amount,
        };

        let next_parts = match dig_direction {
            Right | Left => {
                let next_x = x + delta;

                current_location.0 = next_x;

                vec![(
                    Part {
                        // part_increases_intersection_count: edges_let_inside(edge, next_edge),
                        range: if next_x < x { (next_x, x) } else { (x, next_x) },
                    },
                    y,
                )]
            }

            Down | Up => {
                let next_y = y + delta;
                current_location.1 = next_y;

                let range = if y < next_y {
                    (y + 1)..=(next_y - 1)
                } else {
                    (next_y + 1)..=(y - 1)
                };

                range
                    .map(|row_idx| {
                        (
                            Part {
                                // part_increases_intersection_count: true,
                                range: (x, x),
                            },
                            row_idx,
                        )
                    })
                    .collect()
            }
        };

        // this is the bottleneck, especially because of multiple insertions in "down/up" cases
        for (part, row_idx) in next_parts {
            parts.entry(row_idx).or_insert(Vec::new()).push(part);
        }
    }

    for row_parts in parts.values_mut() {
        row_parts.sort_by_cached_key(|part| part.range.0);

        let next_parts = row_parts.iter().skip(1);

        for (part, next_part) in row_parts.iter().zip(next_parts) {
            assert!(part.range.1 < next_part.range.0);
        }
    }
}

// fn ray_casting(instructions: &Vec<Instruction>) -> u64 {
//     let edges = get_edges(&instructions);
//     let enhanced_instructions = instructions.into_iter().zip(edges.iter());

//     let enhanced_instructions =
//         enhanced_instructions.zip(edges.iter().skip(1).chain(once(edges.first().unwrap())));

//     let mut parts = HashMap::new();
//     let starting_location = (0, 0);
//     let mut current_location = starting_location;

//     for ((instruction, edge), next_edge) in enhanced_instructions {
//         let Instruction {
//             dig_direction,
//             dig_amount,
//         } = instruction;

//         let dig_amount = *dig_amount as i64;
//         let (x, y) = current_location;
//         let delta = match dig_direction {
//             Right | Down => dig_amount,
//             Left | Up => -dig_amount,
//         };

//         let next_parts = match dig_direction {
//             Right | Left => {
//                 let next_x = x + delta;

//                 current_location.0 = next_x;

//                 vec![(
//                     Part {
//                         part_increases_intersection_count: edges_let_inside(edge, next_edge),
//                         range: if next_x < x { (next_x, x) } else { (x, next_x) },
//                     },
//                     y,
//                 )]
//             }

//             Down | Up => {
//                 let next_y = y + delta;
//                 current_location.1 = next_y;

//                 let range = if y < next_y {
//                     (y + 1)..=(next_y - 1)
//                 } else {
//                     (next_y + 1)..=(y - 1)
//                 };

//                 range
//                     .map(|row_idx| {
//                         (
//                             Part {
//                                 part_increases_intersection_count: true,
//                                 range: (x, x),
//                             },
//                             row_idx,
//                         )
//                     })
//                     .collect()
//             }
//         };

//         // this is the bottleneck, especially because of multiple insertions in "down/up" cases
//         for (part, row_idx) in next_parts {
//             parts.entry(row_idx).or_insert(Vec::new()).push(part);
//         }
//     }

//     assert_eq!(current_location, starting_location); // check that we indeed get loop i.e. come back to the starting point

//     for row_entries in parts.values_mut() {
//         row_entries.sort_by_cached_key(|part| part.range.0);
//     }

//     parts.values().fold(0, |inner_points_count, parts| {
//         let next_parts = parts.iter().skip(1);

//         let (row_inner_points_count, _) = parts.iter().zip_longest(next_parts).fold(
//             (0, 0),
//             |(cur_inner_points_count, cur_intersection_count), part_pair| match part_pair {
//                 EitherOrBoth::Both(part, next_part) => {
//                     let &Part {
//                         part_increases_intersection_count,
//                         range,
//                     } = part;

//                     assert!(range.1 < next_part.range.0); // this check assures path does not intersect itself

//                     let part_length = (range.1 - range.0 + 1) as u64;

//                     let next_intersection_count = cur_intersection_count
//                         + if part_increases_intersection_count {
//                             1
//                         } else {
//                             0
//                         };

//                     let next_inner_point_count = cur_inner_points_count
//                         + part_length
//                         + if next_intersection_count % 2 == 1 {
//                             (next_part.range.0 - range.1 - 1) as u64
//                         } else {
//                             0
//                         };

//                     (next_inner_point_count, next_intersection_count)
//                 }
//                 EitherOrBoth::Left(part) => {
//                     let range = part.range;

//                     (cur_inner_points_count + (range.1 - range.0 + 1) as u64, 0)
//                 }
//                 EitherOrBoth::Right(_) => unreachable!(),
//             },
//         );

//         inner_points_count + row_inner_points_count
//     })
// }

// fn get_edges(instructions: &[Instruction]) -> Vec<EdgeType> {
//     let last_direction = instructions.last().unwrap();

//     let instructions_iter = instructions.iter();
//     let shifted_instructions_iter = once(last_direction).chain(instructions.iter());

//     instructions_iter
//         .zip(shifted_instructions_iter)
//         .map(|(cur_instruction, prev_instruction)| {
//             edge_type(
//                 &cur_instruction.dig_direction,
//                 &prev_instruction.dig_direction,
//             )
//         })
//         .collect()
// }

// fn edge_type(next_direction: &DigDirection, prev_direction: &DigDirection) -> EdgeType {
//     match (next_direction, prev_direction) {
//         (Right, Up) | (Down, Left) => DownRight,
//         (Right, Down) | (Up, Left) => UpRight,
//         (Left, Up) | (Down, Right) => LeftDown,
//         (Left, Down) | (Up, Right) => LeftUp,
//         _ => unreachable!(),
//     }
// }

// fn edges_let_inside(edge1: &EdgeType, edge2: &EdgeType) -> bool {
//     (edge_is_up_edge(edge1) && !edge_is_up_edge(edge2))
//         || (!edge_is_up_edge(edge1) && edge_is_up_edge(edge2))
// }

// fn edge_is_up_edge(edge: &EdgeType) -> bool {
//     match edge {
//         UpRight | LeftUp => true,
//         DownRight | LeftDown => false,
//     }
// }
