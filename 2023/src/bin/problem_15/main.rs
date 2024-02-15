#![warn(clippy::all, clippy::pedantic)]

use std::collections::{HashMap, VecDeque};

// use regex::Regex;
use utils::file_reader::FileReader;

struct Lens {
    label: String,
    focal_length: u32,
}

fn main() {
    let mut current_hash_value = 0;
    let mut current_sum = 0;
    let mut setting_focal_length = false;
    let mut focal_length = None;

    let mut boxes = HashMap::new();

    let mut current_label = String::new();
    let mut current_short_hash_value = 0;

    FileReader::new("./input/problem15.txt").process_until(&mut |ch| {
        let new_line_ch = ch == '\n';

        if ch == ',' || new_line_ch {
            current_sum += current_hash_value;
            current_hash_value = 0;

            let box_number = current_short_hash_value;

            let suitable_box: &mut VecDeque<Lens> =
                boxes.entry(box_number).or_insert(VecDeque::new());

            let existing_index = suitable_box
                .iter()
                .position(|lens| lens.label == current_label);

            if let Some(length) = focal_length {
                if let Some(index) = existing_index {
                    suitable_box.get_mut(index).unwrap().focal_length = length;
                } else {
                    suitable_box.push_back(Lens {
                        label: current_label.clone(),
                        focal_length: length,
                    })
                }

                focal_length = None;
            } else if let Some(index) = existing_index {
                suitable_box.remove(index);
            }

            current_label.clear();
        } else {
            let ascii_value = ch as u32;
            let is_minus = ch == '-';
            let is_equal = ch == '=';

            if !(is_minus || is_equal || setting_focal_length) {
                current_label.push(ch);
            }

            if is_minus || is_equal {
                current_short_hash_value = current_hash_value;
            }

            current_hash_value = (17 * (current_hash_value + ascii_value)) % 256;

            if is_equal {
                setting_focal_length = true;
            } else if setting_focal_length {
                focal_length = Some(ch.to_digit(10).unwrap());
                setting_focal_length = false;
            }
        }

        new_line_ch
    });

    println!("Hash sum: {}", current_sum); // 512950

    let focusing_power = boxes.iter().fold(0, |cur_power, cur_box| {
        let (box_number, box_lenses) = cur_box;

        let lenses_power =
            box_lenses
                .iter()
                .enumerate()
                .fold(0, |cur_lenses_power, (index, lens)| {
                    cur_lenses_power + (index as u32 + 1) * lens.focal_length
                });

        cur_power + (box_number + 1) * lenses_power
    });

    println!("Focusing power: {}", focusing_power); // 247153
}
