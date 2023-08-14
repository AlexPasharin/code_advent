#![warn(clippy::all, clippy::pedantic)]

use std::collections::VecDeque;
use utils::file_reader::FileReader;

struct ContainersSet {
    indices_used: Vec<usize>,
    size: u32,
}

const EGGNOG_AMOUNT: u32 = 150;

fn main() {
    let mut containers = Vec::new();

    FileReader::process_lines("./input/problem_17.txt", &mut |line| {
        let container_size = line.trim().parse::<u32>().unwrap();

        containers.push(container_size);
    });

    assert!(containers.iter().all(|c| *c > 0));

    let mut queue = VecDeque::new();

    for (idx, container) in containers.iter().enumerate() {
        queue.push_back(ContainersSet {
            indices_used: vec![idx],
            size: *container,
        });
    }

    let mut ways_to_get_eggnot_amount = 0;

    let mut minimal_amount_of_containers_needed = usize::MAX;
    let mut ways_to_get_eggnot_amount_with_minimal_amount = 0;

    while let Some(ContainersSet { indices_used, size }) = queue.pop_front() {
        if size == EGGNOG_AMOUNT {
            ways_to_get_eggnot_amount += 1;

            let current_amount_of_containers_used = indices_used.len();

            if current_amount_of_containers_used == minimal_amount_of_containers_needed {
                ways_to_get_eggnot_amount_with_minimal_amount += 1;
            } else if current_amount_of_containers_used < minimal_amount_of_containers_needed {
                minimal_amount_of_containers_needed = current_amount_of_containers_used;
                ways_to_get_eggnot_amount_with_minimal_amount = 1;
            }
        }

        if size >= EGGNOG_AMOUNT {
            continue;
        }

        let max_index_used = indices_used.last().unwrap();

        for (i, container) in containers.iter().enumerate().skip(max_index_used + 1) {
            let mut new_indiced_used = indices_used.clone();
            new_indiced_used.push(i);

            let new_size = size + container;

            queue.push_back(ContainersSet {
                indices_used: new_indiced_used,
                size: new_size,
            });
        }
    }

    println!(
        "Part 1, ways to get exact eggnog amount: {}",
        ways_to_get_eggnot_amount
    ); // 1304

    println!(
        "Part 2, ways to get exact eggnog amount with minimal amount of containers: {}",
        ways_to_get_eggnot_amount_with_minimal_amount
    ); // 18
}
