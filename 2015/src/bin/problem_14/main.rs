#![warn(clippy::all, clippy::pedantic)]

use std::cmp::min;

use regex::Regex;

use utils::file_reader::FileReader;

const OVERALL_RUN_TIME: u32 = 2503;

struct Reindeer {
    // name: String,
    speed: u32,
    continuous_running_time: u32,
    continuous_cycle_length: u32,
    current_place_in_cycle: u32,
    distance_ran: u32,
    points: u32,
}

impl Reindeer {
    fn next_state(&mut self) {
        let Reindeer {
            speed,
            continuous_running_time,
            continuous_cycle_length,
            current_place_in_cycle,
            distance_ran,
            ..
        } = self;

        if current_place_in_cycle <= continuous_running_time {
            *distance_ran += *speed;
        }

        if current_place_in_cycle == continuous_cycle_length {
            *current_place_in_cycle = 1;
        } else {
            *current_place_in_cycle += 1;
        }
    }
}

fn main() {
    let re = Regex::new(
        r"^(?:\S+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
    )
    .unwrap();

    let mut max_distance = 0;

    let mut reindeers: Vec<Reindeer> = Vec::new();

    FileReader::process_lines("./input/problem_14.txt", &mut |line| {
        let capture_match = re.captures(line.trim()).unwrap();

        let parse_int = |idx: usize| capture_match[idx].parse::<u32>().unwrap();

        let speed = parse_int(1);
        let continuous_running_time = parse_int(2);
        let rest_seconds = parse_int(3);

        let continuous_cycle_length = continuous_running_time + rest_seconds;

        reindeers.push(Reindeer {
            speed,
            continuous_running_time,
            continuous_cycle_length,
            current_place_in_cycle: 1,
            distance_ran: 0,
            points: 0,
        });

        let km_for_one_cycle = speed * continuous_running_time;
        let amount_of_full_cycles = OVERALL_RUN_TIME / continuous_cycle_length;
        let reminder = OVERALL_RUN_TIME % continuous_cycle_length;

        let total_distance = amount_of_full_cycles * km_for_one_cycle
            + min(reminder, continuous_running_time) * speed;

        if total_distance > max_distance {
            max_distance = total_distance;
        }
    });

    println!("Part 1: Longest distance is {}", max_distance); // 2655

    for _ in 0..OVERALL_RUN_TIME {
        let mut max_distance = 0u32;
        let mut max_distance_reindeers = Vec::new();

        for reindeer in &mut reindeers {
            reindeer.next_state();

            if reindeer.distance_ran > max_distance {
                max_distance = reindeer.distance_ran;
                max_distance_reindeers = vec![reindeer];
            } else if reindeer.distance_ran == max_distance {
                max_distance_reindeers.push(reindeer);
            }
        }

        for reindeer in max_distance_reindeers {
            reindeer.points += 1;
        }
    }

    let max_distance = reindeers.iter().map(|r| r.distance_ran).max().unwrap();
    let max_points = reindeers.iter().map(|r| r.points).max().unwrap();

    println!(
        "Part 1: Longest distance, calculated another way: {}",
        max_distance
    ); // 2655
    println!("Part 2: Max points: {}", max_points); // 1059
}
