#![warn(clippy::all, clippy::pedantic)]

use utils::file_reader::FileReader;

use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();

    let mut result1 = 0;
    let mut result2 = 0;

    FileReader::process_lines("./input/problem_2.txt", &mut |line| {
        let capture_match = re
            .captures(line.trim())
            .expect("Could not extract dimensions");

        let get_capture_by_index = |index: usize| capture_match[index].parse::<u32>().unwrap(); // we assume every input line satisfies regex, so we can use unwrap "safely"

        let length = get_capture_by_index(1);
        let width = get_capture_by_index(2);
        let height = get_capture_by_index(3);

        let side1 = length * width;
        let side2 = width * height;
        let side3 = length * height;

        let side_areas = [side1, side2, side3];

        let total_area = 2 * side_areas.iter().sum::<u32>();
        let smallest_side_area = side_areas.iter().min().unwrap(); // we know that array is not empty, so extracting min must be safe

        result1 += total_area + smallest_side_area;

        let mut sides = [length, width, height];
        sides.sort_unstable(); // unstable sort is more efficient and stability does not matter here

        result2 += 2 * (sides[0] + sides[1]) + (length * width * height);
    });

    println!("Part 1 solution {result1}"); //1606483
    println!("Part 2 solution {result2}"); //3842356
}
