#![warn(clippy::all, clippy::pedantic)]

use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let re = Regex::new(r"^Game (\d+): (.+)").unwrap();
    let green_re = Regex::new(r"(\d+) green").unwrap();
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let blue_re = Regex::new(r"(\d+) blue").unwrap();

    let first_part_max_values = [(red_re, 12), (green_re, 13), (blue_re, 14)];

    let mut id_sum = 0;
    let mut power_sum = 0;

    FileReader::process_lines("./input/problem2.txt", &mut |line| {
        let captures = re.captures(line).unwrap();
        let cubes = &captures[2].split(";").collect::<Vec<_>>();

        let mut game_is_possible = true;

        let mut values = first_part_max_values
            .iter()
            .map(|(re, max)| (re, max, 0))
            .collect::<Vec<_>>();

        for selected_cubes in cubes {
            for (regex, first_part_max_values_amount, min) in values.iter_mut() {
                if let Some(amount) = regex
                    .captures(&selected_cubes)
                    .map(|captures| captures[1].parse::<i32>().unwrap())
                {
                    if &amount > first_part_max_values_amount {
                        game_is_possible = false;
                        // break 'outer;
                    }

                    if amount > *min {
                        *min = amount;
                    }
                }
            }
        }

        if game_is_possible {
            let game_id = captures[1].parse::<i32>().unwrap();
            id_sum += game_id;
        }

        power_sum += values.iter().map(|(_, _, min)| min).product::<i32>();
    });

    println!("Part 1 solution: {}", id_sum); // 2076
    println!("Part 2 solution: {}", power_sum); //70950
}
