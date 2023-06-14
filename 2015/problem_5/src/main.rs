use std::collections::{HashMap, HashSet};
use utils::file_reader::FileReader;

fn main() {
    let mut amount_of_nice_strings_part_one = 0;
    let mut amount_of_nice_strings_part_two = 0;

    FileReader::process_lines("./input.txt", &mut |line| {
        let chars: Vec<char> = line.chars().collect();

        let prev = chars.get(0);

        // empty line case
        if prev == None {
            return;
        }

        let mut prev = *prev.unwrap(); // first char in the string

        let mut amount_of_vowels = if is_vowel(prev) { 1 } else { 0 };
        let mut has_consecutive_dublicates = false;
        let mut contains_forbidden_pair = false;

        let mut contains_pair_twice = false;

        let mut different_letter_pairs: HashSet<String> = HashSet::new();
        let mut same_letter_twice: HashMap<char, usize> = HashMap::new();

        let mut contains_letter_between_same_letters = false;

        for current_index in 1..chars.len() {
            let curr = *chars.get(current_index).unwrap();

            if is_vowel(curr) {
                amount_of_vowels += 1;
            }

            if prev == curr {
                has_consecutive_dublicates = true;

                if let Some(prev_same_letter_twice_index) = same_letter_twice.get(&prev) {
                    contains_pair_twice =
                        contains_pair_twice || (*prev_same_letter_twice_index < current_index - 1);
                } else {
                    same_letter_twice.insert(curr, current_index);
                }
            } else {
                let consecutive_pair = format!("{prev}{curr}");

                if different_letter_pairs.contains(&consecutive_pair) {
                    contains_pair_twice = true;
                } else {
                    different_letter_pairs.insert(consecutive_pair);

                    contains_forbidden_pair =
                        contains_forbidden_pair || char_pair_is_forbidden(prev, curr)
                }
            }

            if current_index > 1 {
                let prev_prev = *chars.get(current_index - 2).unwrap();

                contains_letter_between_same_letters =
                    contains_letter_between_same_letters || prev_prev == curr
            }

            prev = curr;
        }

        if amount_of_vowels > 2 && has_consecutive_dublicates && !contains_forbidden_pair {
            amount_of_nice_strings_part_one += 1;
        }

        if contains_pair_twice && contains_letter_between_same_letters {
            amount_of_nice_strings_part_two += 1;
        }
    });

    // 258 and 53
    println!("Amount of nice strings in part 1: {amount_of_nice_strings_part_one}");
    println!("Amount of nice strings in part 2: {amount_of_nice_strings_part_two}");
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_vowel(ch: char) -> bool {
    VOWELS.contains(&ch)
}

fn char_pair_is_forbidden(ch1: char, ch2: char) -> bool {
    (ch1 == 'a' && ch2 == 'b')
        || (ch1 == 'c' && ch2 == 'd')
        || (ch1 == 'p' && ch2 == 'q')
        || (ch1 == 'x' && ch2 == 'y')
}
