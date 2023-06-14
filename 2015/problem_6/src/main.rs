use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    const SIZE: usize = 1000;
    let mut points = vec![[(false, 0usize); SIZE]; SIZE]; // cant use [[false; 1000]; 1000]; cause it will allocate on the stack, which will overflow for such a big array

    FileReader::process_lines("input.txt", &mut |line| {
        let capture_match = re
            .captures(line.trim())
            .expect("Could not extract instruction");

        let instruction_type = &capture_match[1];
        let corner1 = parse_corner_coordinates(&capture_match[2], &capture_match[3]);
        let corner2 = parse_corner_coordinates(&capture_match[4], &capture_match[5]);

        let mut corners_x_coords = [corner1.0, corner2.0];
        corners_x_coords.sort();

        let [x_min, x_max] = corners_x_coords;

        let mut corners_y_coords = [corner1.1, corner2.1];
        corners_y_coords.sort();

        let [y_min, y_max] = corners_y_coords;

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                let (light_on, amount_of_light) = points[x][y];

                points[x][y] = if instruction_type == "turn on" {
                    (true, amount_of_light + 1)
                } else if instruction_type == "turn off" {
                    (
                        false,
                        if amount_of_light > 0 {
                            amount_of_light - 1
                        } else {
                            0
                        },
                    )
                } else {
                    (!light_on, amount_of_light + 2)
                }
            }
        }
    });

    let mut amount_of_lights_part1 = 0;
    let mut amount_of_lights_part2 = 0;

    for point in points {
        for (light_on, amount_of_light) in point {
            if light_on {
                amount_of_lights_part1 += 1;
            }

            amount_of_lights_part2 += amount_of_light
        }
    }

    println!("Part 1: {amount_of_lights_part1} lights are on"); //569999
    println!("Part 2: {amount_of_lights_part2} lights are on"); //569999
}

fn parse_corner_coordinates(s1: &str, s2: &str) -> (usize, usize) {
    (parse_int(s1), parse_int(s2))
}

fn parse_int(s: &str) -> usize {
    s.parse().unwrap()
}
