use std::collections::HashSet;
use utils::file_reader::FileReader;

fn main() {
    let origin = (0, 0);

    // since Point is copy, these are all independent copies
    let mut p1 = origin; // tracks position of Santa in part 1
    let mut p2 = origin; // tracks position of Santa in part 2
    let mut p3 = origin; // tracks position of RoboSanta in part 2

    let mut points1 = HashSet::from([p1]); // set of points visited by Santa in part 1
    let mut points2 = HashSet::from([p2]); // set of points visited by Santa and RoboSanta in part 2

    let mut santa_turn = true; // for part 2, true if Santa turn, RoboSanta turn otherwise

    FileReader::process_file("./input.txt", &mut |ch| {
        let mut second_year_point = if santa_turn { &mut p2 } else { &mut p3 };

        match ch {
            '>' => {
                p1.0 += 1;
                second_year_point.0 += 1;
            }
            '<' => {
                p1.0 -= 1;
                second_year_point.0 -= 1;
            }
            '^' => {
                p1.1 += 1;
                second_year_point.1 += 1;
            }
            'v' => {
                p1.1 -= 1;
                second_year_point.1 -= 1;
            }
            _ => (), // this is not needed logically, but match in Rust be exustive. Perhaps good old fashioned if else would be better in this case
        }

        // this works because (i32, i32) is a copy type, so every time we insert it, we actually insert a copy
        points1.insert(p1);
        // second_year_point on the other hand must be a reference to either p2 or p3, coz otherwise second_year_point will always be a cloned copy of p2 or p3, so any modifications of it wont affect them
        points2.insert(*second_year_point);

        // NOTE: the approach taken, leading to assymetry above (p1 can be value, but second_year_point must be a reference) is left because its a good exercise in understaing Rust copy-move-references mechanics
        // however it had lead to clearly smelly code
        // since we are doing (implicit) copying anyway, better approach stylistically would be to use immutable Point objects

        santa_turn = !santa_turn;
    });

    println!("Points visited by Santa first year: {}", points1.len()); // 2565
    println!(
        "Points visited by Santa and Robo-Santa second year: {}",
        points2.len() // 2639
    );
}
