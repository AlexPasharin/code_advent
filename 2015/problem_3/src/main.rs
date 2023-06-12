use std::collections::HashSet;
use utils::file_reader::FileReader;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

fn main() {
    let mut p1 = Point::new(0, 0);

    let mut points1 = HashSet::new();
    points1.insert(p1.clone());

    let mut p2 = Point::new(0, 0);
    let mut p3 = Point::new(0, 0);

    let mut points2 = HashSet::new();
    points2.insert(p2.clone());

    let mut santa_turn = true;

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
            _ => {}
        }

        points1.insert(p1.clone());
        points2.insert(second_year_point.clone());

        santa_turn = !santa_turn;
    });

    println!("Points visited by Santa first year: {}", points1.len()); // 2565
    println!(
        "Points visited by Santa and Robo-Santa second year: {}",
        points2.len() // 2639
    );
}
