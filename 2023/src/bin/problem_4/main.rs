#![warn(clippy::all, clippy::pedantic)]

use std::{cmp::min, collections::HashSet, iter::repeat};

use pathfinding::num_traits::Pow;
use regex::Regex;
// use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let re = Regex::new(r"^Card(?:\s+)(\d+): ([^|]+) \| ([^|]+)$").unwrap();

    let mut sum_of_wins = 0;
    let mut matching_numbers = Vec::new();

    FileReader::process_lines("./input/problem4.txt", &mut |line| {
        let captures = re.captures(line).unwrap();

        let winning_numbers = &captures[2]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let scratched_numbers = &captures[3]
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        // println!("{:?}", winning_numbers);

        let matching_numbers_amount = winning_numbers.intersection(scratched_numbers).count();
        matching_numbers.push(matching_numbers_amount);

        if matching_numbers_amount > 0 {
            sum_of_wins += 2.pow(matching_numbers_amount - 1);
        }
    });

    println!("Sum of wins: {}", sum_of_wins);

    let mut card_copies = repeat(1).take(matching_numbers.len()).collect::<Vec<_>>();

    for (idx, card_matching_numbers) in matching_numbers.iter().enumerate() {
        (idx + 1..(min(idx + card_matching_numbers + 1, card_copies.len()))).for_each(|j| {
            card_copies[j] += card_copies[idx];
        });
    }

    let total_cards_amount = card_copies.iter().sum::<i32>();

    println!("Amount of cards: {}", total_cards_amount);
}
