#![warn(clippy::all, clippy::pedantic)]

use std::ops::Range;

use regex::Regex;
// use regex::Regex;
use utils::file_reader::FileReader;

#[derive(Debug)]
struct TransitionRule {
    source_range: Range<u64>,
    destination_range: Range<u64>,
}

fn main() {
    let re = Regex::new(r"map").unwrap();

    let mut transition_maps = Vec::new();

    FileReader::process_lines("./input/problem5.txt", &mut |line| {
        if line.is_empty() {
            return;
        }

        let is_new_map_starting_line = re.is_match(line);

        if is_new_map_starting_line {
            transition_maps.push(Vec::new());
        } else {
            let transition_rule = line
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            assert_eq!(transition_rule.len(), 3);

            let destination_range_start = transition_rule[0];
            let source_range_start = transition_rule[1];
            let range_length = transition_rule[2];

            let new_rule = TransitionRule {
                source_range: (source_range_start..source_range_start + range_length),
                destination_range: (destination_range_start
                    ..destination_range_start + range_length),
            };

            transition_maps.last_mut().unwrap().push(new_rule);
        }
    });

    for map in transition_maps.iter_mut() {
        map.sort_unstable_by_key(|rule| rule.source_range.start);
    }

    let seeds =  "4043382508 113348245 3817519559 177922221 3613573568 7600537 773371046 400582097 2054637767 162982133 2246524522 153824596 1662955672 121419555 2473628355 846370595 1830497666 190544464 230006436 483872831"
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut values = seeds.clone(); // clone initial seeds, coz we will need them again in the second part

    for map in &transition_maps {
        values = values
            .into_iter()
            .map(
                |value| match map.iter().find(|rule| rule.source_range.contains(&value)) {
                    Some(rule) => {
                        let TransitionRule {
                            source_range,
                            destination_range,
                        } = rule;

                        let offset = value - source_range.start;

                        destination_range.start + offset
                    }
                    None => value,
                },
            )
            .collect();
    }

    let lowest_location = values.iter().min().unwrap();

    println!("Lowest location, part 1: {}", lowest_location); // 289863851

    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .collect::<Vec<_>>();

    let mut values = seed_ranges;

    for map in &transition_maps {
        let mut next_values = Vec::new();

        for range in values {
            let mut current_start = range.start;

            for transition_rule in map {
                let TransitionRule {
                    source_range,
                    destination_range,
                } = transition_rule;

                let intersection_start = source_range.start.max(current_start);
                let intersection_end = source_range.end.min(range.end);

                if intersection_start < intersection_end {
                    next_values.push(current_start..intersection_start);

                    current_start = intersection_end;

                    let start_offset = intersection_start - source_range.start;

                    let destination_start = destination_range.start + start_offset;
                    let destination_end =
                        destination_start + (intersection_end - intersection_start);

                    next_values.push(destination_start..destination_end);
                }
            }

            next_values.push(current_start..range.end);

            next_values = next_values
                .into_iter()
                .filter(|range| !range.is_empty())
                .collect();
        }

        values = next_values;
    }

    let lowest_location = values.iter().map(|r| r.start).min().unwrap();

    println!("Lowest location, part 2: {}", lowest_location); // 60568880
}
