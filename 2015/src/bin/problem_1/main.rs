#![warn(clippy::all, clippy::pedantic)]

use utils::file_reader::FileReader;

fn main() {
    // we calculate both parts problems at the same time
    let mut reader = FileReader::new("./input/problem_1.txt");

    let mut floor = 0; // track floor
    let mut position = 0; // track index of char from input, needed only for part 2

    // process input one char at a time until we get to floor -1 for the first time
    reader.process_until(&mut |ch| {
        process_next_direction(&mut floor, ch);
        position += 1;

        floor == -1
    });

    println!("First basement occurance at position {position}"); // 1771

    // continue processing input, now you don't have to care about tracking position
    reader.process_all(&mut |ch| {
        process_next_direction(&mut floor, ch);
    });

    println!("Final floor is {}", floor); // 138
}

fn process_next_direction(floor: &mut i32, next_direction: char) {
    if next_direction == '(' {
        *floor += 1;
    } else if next_direction == ')' {
        *floor -= 1;
    }
}
