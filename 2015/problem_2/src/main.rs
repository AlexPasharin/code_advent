use utils::file_reader::FileReader;

use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+)x(\d+)x(\d+)$").unwrap();

    let mut result1 = 0;
    let mut result2 = 0;

    FileReader::process_lines("input.txt", &mut |line| {
        let capture_match = re
            .captures(line.trim())
            .expect("Could not extract dimensions");

        let length = &capture_match[1].parse::<u32>().unwrap();
        let width = &capture_match[2].parse::<u32>().unwrap();
        let height = &capture_match[3].parse::<u32>().unwrap();

        let side1 = length * width;
        let side2 = width * height;
        let side3 = length * height;

        let side_areas = [side1, side2, side3];

        let total_area = 2 * side_areas.iter().sum::<u32>();
        let smallest_side_area = side_areas.iter().min().unwrap();

        result1 += total_area + smallest_side_area;

        let mut sides = [length, width, height];
        sides.sort();

        result2 += 2 * (sides[0] + sides[1]) + (length * width * height);
    });

    println!("Part 1 solution {result1}"); //1606483
    println!("Part 2 solution {result2}"); //3842356
}
