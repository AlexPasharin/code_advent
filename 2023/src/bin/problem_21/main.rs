#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

use num::traits::Euclid;
// use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let mut grid = HashSet::new();

    let mut amount_of_rows = 0;
    let mut amount_of_columns = 0;
    let mut starting_point = (0, 0);

    FileReader::process_lines("./input/problem21.txt", &mut |line| {
        let chars = line.chars().collect::<Vec<_>>();
        amount_of_columns = chars.len() as i32;

        grid.extend(
            chars
                .into_iter()
                .enumerate()
                .filter_map(|(column_idx, ch)| match ch {
                    '.' => Some((amount_of_rows, column_idx as i32)),
                    'S' => {
                        starting_point = (amount_of_rows, column_idx as i32);
                        Some(starting_point)
                    }
                    _ => None,
                }),
        );

        amount_of_rows += 1;
    });

    // let mut visited_plots = HashSet::new();
    let points_queue = (0..64).fold(HashSet::from([starting_point]), |points_queue, _| {
        points_queue
            .iter()
            .flat_map(|(x, y)| {
                let x = *x;
                let y = *y;

                [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)]
                    .into_iter()
                    .filter(|(a, b)| grid.contains(&(*a, *b)))
            })
            .collect()
    });

    println!("Visited plots: {}", points_queue.len()); // 3649

    let mut points_queues = Vec::new();
    points_queues.push(HashSet::from([starting_point]));

    loop {
        let last_queue = points_queues.last().unwrap();

        let next_queue = last_queue
            .iter()
            .flat_map(|(x, y)| {
                let x = *x;
                let y = *y;

                [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)]
                    .into_iter()
                    .filter(|(a, b)| {
                        let a = (*a).rem_euclid(&amount_of_rows);
                        let b = (*b).rem_euclid(amount_of_columns);

                        grid.contains(&(a, b))
                    })
            })
            .collect();

        // let queue_already_encountered_idx =
        //     points_queues.iter().position(|queue| *queue == next_queue);

        if let Some(idx) = points_queues.iter().position(|queue| *queue == next_queue) {
            println!("Cycle start: {}", idx);
            println!("Cycle end: {}", points_queues.len());
            break;
        }

        points_queues.push(next_queue);
    }

    // // let mut visited_plots = HashSet::new();
    // let points_queue = (0..5000).fold(HashSet::from([starting_point]), |points_queue, _| {
    //     points_queue
    //         .iter()
    //         .flat_map(|(x, y)| {
    //             let x = *x;
    //             let y = *y;

    //             [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)]
    //                 .into_iter()
    //                 .filter(|(a, b)| {
    //                     let a = (*a).rem_euclid(&amount_of_rows);
    //                     let b = (*b).rem_euclid(amount_of_columns);

    //                     grid.contains(&(a, b))
    //                 })
    //         })
    //         .collect()
    // });

    // println!("Visited plots: {}", points_queue.len()); // 3649
}
