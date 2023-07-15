use std::cmp::max;

use regex::Regex;
use utils::file_reader::FileReader;

struct Ingridient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

const SUM: i32 = 100;
const IDEAL_CALORIES: i32 = 500;

fn main() {
    let re = Regex::new(
        r"^([^:]+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
    )
    .unwrap();

    let mut ingridients = Vec::new();

    FileReader::process_lines("./input/problem_15.txt", &mut |line| {
        let capture_match = re
            .captures(line.trim())
            .expect("Could not match the input line");

        let parse_int = |idx: usize| capture_match[idx].parse::<i32>().unwrap();

        // let ingridient_name = capture_match[1].to_string();

        let capacity = parse_int(2);
        let durability = parse_int(3);
        let flavor = parse_int(4);
        let texture = parse_int(5);
        let calories = parse_int(6);

        let ingridient = Ingridient {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        };

        ingridients.push(ingridient);
    });

    let amount_of_ingridients = ingridients.len();
    let all_combinations = vectors_with_fixed_sum(amount_of_ingridients, SUM);

    let mut max_total_score = 0;
    let mut max_total_score_with_calories = 0;

    let ingridients = &ingridients;

    for combination in all_combinations {
        let mut total_capacity = 0;
        let mut total_durability = 0;
        let mut total_flavor = 0;
        let mut total_texture = 0;
        let mut total_calories = 0;

        let enhanced_ingridients = combination.iter().zip(ingridients);

        for (amount, ingridient) in enhanced_ingridients {
            let Ingridient {
                capacity,
                durability,
                flavor,
                texture,
                calories,
            } = ingridient;

            total_capacity += capacity * amount;
            total_durability += durability * amount;
            total_flavor += flavor * amount;
            total_texture += texture * amount;
            total_calories += calories * amount;
        }

        total_capacity = max(total_capacity, 0);
        total_durability = max(total_durability, 0);
        total_flavor = max(total_flavor, 0);
        total_texture = max(total_texture, 0);

        let total_score = total_capacity * total_durability * total_flavor * total_texture;

        max_total_score = max(max_total_score, total_score);

        if total_calories == IDEAL_CALORIES {
            max_total_score_with_calories = max(max_total_score_with_calories, total_score);
        }
    }

    println!("Max total score: {}", max_total_score); // 222870
    println!(
        "Max total score with {} calories: {}",
        IDEAL_CALORIES, max_total_score_with_calories
    ); // 117936
}

fn vectors_with_fixed_sum(length: usize, sum: i32) -> Vec<Vec<i32>> {
    if length == 1 {
        return vec![vec![sum]];
    }

    let mut result = Vec::new();

    for i in 0..=sum {
        let remaining_sum = sum - i;

        let mut partial_result = vectors_with_fixed_sum(length - 1, remaining_sum);

        for v in &mut partial_result {
            v.push(i);
        }

        result.extend_from_slice(&partial_result);
    }

    return result;
}
