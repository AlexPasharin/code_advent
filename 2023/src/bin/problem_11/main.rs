#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashSet;

// use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    println!(
        "Sum of shortest distances between galaxies, expansion factor 2: {}",
        calc_distance_sum(2) //9403026
    );

    println!(
        "Sum of shortest distances between galaxies, expansion factor 1 000 000: {}",
        calc_distance_sum(1_000_000) // 543018317006
    );
}

fn calc_distance_sum(expansion_factor: usize) -> usize {
    let mut rows = Vec::new();
    let mut max_column = 0;

    let factor = expansion_factor - 1;

    FileReader::process_lines("./input/problem11.txt", &mut |line| {
        let galaxy_indices: Vec<_> = line
            .char_indices()
            .filter_map(|(index, char)| if char == '#' { Some(index) } else { None })
            .collect();

        if let Some(max_column_index) = galaxy_indices.last() {
            max_column = max_column.max(*max_column_index);
        } else {
            for _ in 0..factor {
                rows.push(Vec::new());
            }
        }

        rows.push(galaxy_indices);
    });

    let used_column_indices: HashSet<_> = rows.iter().flat_map(|row_indices| row_indices).collect();

    let empty_column_indices: Vec<_> = (0..max_column)
        .filter(|idx| !used_column_indices.contains(idx))
        .collect();

    for row in rows.iter_mut() {
        let mut index = 0;

        for galaxy_id in row {
            while index < empty_column_indices.len() && *galaxy_id > empty_column_indices[index] {
                index += 1;
            }

            *galaxy_id += index * factor;
        }
    }

    let galaxies: Vec<_> = rows
        .into_iter()
        .enumerate()
        .flat_map(|(row_idx, row)| row.into_iter().map(move |column_idx| (row_idx, column_idx)))
        .collect();

    let mut shortest_path_sum = 0;

    for (id, galaxy_1) in galaxies.iter().enumerate() {
        for galaxy_2 in &galaxies[id + 1..] {
            let distance = galaxy_1.0.abs_diff(galaxy_2.0) + galaxy_1.1.abs_diff(galaxy_2.1);

            shortest_path_sum += distance;
        }
    }

    shortest_path_sum
}
