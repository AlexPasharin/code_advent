#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let re = Regex::new(
      r"^Sue (\d+): ((?:(?:children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): \d+, )+(?:(?:children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes): \d+))$",
  )
  .unwrap();

    let suitable_aunt_properties = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let mut part_1_suitable_aunt_numbers = Vec::new();
    let mut part_2_suitable_aunt_numbers = Vec::new();

    FileReader::process_lines("./input/problem_16.txt", &mut |line| {
        let capture_match = re
            .captures(line.trim())
            .expect(&(String::from("Could not match the input line ") + line));

        let aunt_number = capture_match[1].parse::<u32>().unwrap();
        let aunt_properties = capture_match[2]
            .split(", ")
            .map(|prop| prop.split(": ").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let part_1_aunt_is_suitable = aunt_properties.iter().all(|property_spec| {
            let property_name = property_spec[0];
            let amount = property_spec[1].parse::<u32>().unwrap();

            *suitable_aunt_properties.get(property_name).unwrap() == amount
        });

        let part_2_aunt_is_suitable = aunt_properties.iter().all(|property_spec| {
            let property_name = property_spec[0];
            let amount = property_spec[1].parse::<u32>().unwrap();

            if property_name == "cats" || property_name == "trees" {
                amount > *suitable_aunt_properties.get(property_name).unwrap()
            } else if property_name == "pomeranians" || property_name == "goldfish" {
                amount < *suitable_aunt_properties.get(property_name).unwrap()
            } else {
                *suitable_aunt_properties.get(property_name).unwrap() == amount
            }
        });

        if part_1_aunt_is_suitable {
            part_1_suitable_aunt_numbers.push(aunt_number);
        }

        if part_2_aunt_is_suitable {
            part_2_suitable_aunt_numbers.push(aunt_number);
        }
    });

    println!("Suitable aunts, part 1: {:?}", part_1_suitable_aunt_numbers); // [40]
    println!("Suitable aunts, part 2: {:?}", part_2_suitable_aunt_numbers); // [241]
}
