#![warn(clippy::all, clippy::pedantic)]

use std::{
    collections::HashMap,
    iter::{once, repeat, repeat_with},
};

use itertools::Itertools;
// use regex::Regex;
use utils::file_reader::FileReader;

#[derive(PartialEq, Eq, Hash)]
struct State {
    groups: Vec<Vec<char>>,
    numbers: Vec<usize>,
}

fn main() {
    let mut sum_of_arragement_amounts = 0;
    let mut sum_of_folded_arragement_amounts = 0;

    let mut cache = HashMap::new();

    FileReader::process_lines("./input/problem12.txt", &mut |line| {
        let mut line_split = line.split_whitespace();

        let springs_row = line_split.next().unwrap();
        let springs_row_numbers = line_split.next().unwrap();

        let unfolded_springs_row = repeat(springs_row)
            .take(5)
            .interleave(repeat("?").take(4))
            .collect::<String>();

        assert!(line_split.next().is_none());

        let groups = groups_from_str(springs_row);

        let springs_row_numbers = springs_row_numbers
            .split(',')
            .map(|ch| ch.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let unfolded_groups = groups_from_str(&unfolded_springs_row);
        let unfolded_numbers = repeat_with(|| springs_row_numbers.iter())
            .take(5)
            .flatten()
            .copied()
            .collect::<Vec<_>>();

        let result = arragement_amounts_cached(
            State {
                groups,
                numbers: springs_row_numbers,
            },
            &mut cache,
        );

        sum_of_arragement_amounts += result;

        let result2 = arragement_amounts_cached(
            State {
                groups: unfolded_groups,
                numbers: unfolded_numbers,
            },
            &mut cache,
        );
        sum_of_folded_arragement_amounts += result2;
    });

    println!("Sum of arrangement_amounts: {}", sum_of_arragement_amounts); // 8419
    println!(
        "Sum of folded arrangement_amounts: {}",
        sum_of_folded_arragement_amounts
    );
    // 160500973317706
}

fn arragement_amounts_cached(state: State, cache: &mut HashMap<State, u64>) -> u64 {
    if let Some(&result) = cache.get(&state) {
        return result;
    }

    let State { groups, numbers } = &state;

    let result = match (groups.get(0), numbers.get(0)) {
        (_, None) => {
            if groups.iter().all(|group| group.iter().all(|ch| *ch == '?')) {
                1
            } else {
                0
            }
        }
        (None, _) => {
            if numbers.iter().all(|n| *n == 0) {
                1
            } else {
                0
            }
        }
        (Some(group), Some(&number)) => {
            match group.iter().find_position(|&&ch| ch == '?') {
                None => {
                    if group.is_empty() {
                        arragement_amounts_cached(
                            State {
                                groups: groups[1..].to_vec(),
                                numbers: numbers.clone(),
                            },
                            cache,
                        )
                    } else if number == group.len() {
                        arragement_amounts_cached(
                            State {
                                groups: groups[1..].to_vec(),
                                numbers: numbers[1..].to_vec(),
                            },
                            cache,
                        )
                    } else {
                        0
                    }
                }
                Some((idx, _)) => {
                    if idx == 0 {
                        // add_spring_case
                        let new_group = once('#')
                            .chain(group[1..].iter().copied())
                            .collect::<Vec<_>>();

                        let new_groups_with_added_spring = once(new_group)
                            .chain(groups[1..].iter().map(|v| v.clone()))
                            .collect::<Vec<_>>();

                        // dont_add_spring_case
                        let new_groups_without_added_spring = once(group[1..].to_vec())
                            .chain(groups[1..].iter().map(|v| v.clone()))
                            .collect::<Vec<_>>();

                        arragement_amounts_cached(
                            State {
                                groups: new_groups_with_added_spring,
                                numbers: numbers.clone(),
                            },
                            cache,
                        ) + arragement_amounts_cached(
                            State {
                                groups: new_groups_without_added_spring,
                                numbers: numbers.clone(),
                            },
                            cache,
                        )
                    } else if idx == number {
                        let new_group = group[(idx + 1)..].to_vec();

                        let new_groups = if new_group.is_empty() {
                            groups[1..].to_vec()
                        } else {
                            once(new_group)
                                .chain(groups[1..].iter().map(|v| v.clone()))
                                .collect::<Vec<_>>()
                        };

                        arragement_amounts_cached(
                            State {
                                groups: new_groups,
                                numbers: numbers[1..].to_vec(),
                            },
                            cache,
                        )
                    } else if idx < number {
                        let new_group = group[0..idx]
                            .iter()
                            .copied()
                            .chain(once('#'))
                            .chain(group[(idx + 1)..].iter().copied())
                            .collect::<Vec<_>>();

                        let new_groups = once(new_group)
                            .chain(groups[1..].iter().map(|v| v.clone()))
                            .collect::<Vec<_>>();

                        arragement_amounts_cached(
                            State {
                                groups: new_groups,
                                numbers: numbers.clone(),
                            },
                            cache,
                        )
                    } else {
                        0
                    }
                }
            }
        }
    };

    cache.insert(state, result);

    result
}

fn groups_from_str(str: &str) -> Vec<Vec<char>> {
    let mut chars_iter = str.chars().peekable();

    let mut groups = Vec::new();

    loop {
        while chars_iter.peek() == Some(&'.') {
            chars_iter.next();
        }

        let mut next_group = Vec::new();

        while let Some(next_el) = chars_iter.next().filter(|next_el| *next_el != '.') {
            next_group.push(next_el);
        }

        groups.push(next_group);

        if let None = chars_iter.peek() {
            break;
        }
    }

    groups
}
