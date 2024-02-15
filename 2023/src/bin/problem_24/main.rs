#![warn(clippy::all, clippy::pedantic)]

use regex::Regex;
use utils::file_reader::FileReader;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct HailStone {
    position: Point,
    velocity: Velocity,
}

static MIN: f64 = 200000000000000.0;
static MAX: f64 = 400000000000000.0;

fn main() {
    let regex = Regex::new(r"^(\d+), (\d+), (\d+) @ (-?\d+), (-?\d+), (-?\d+)$").unwrap();

    let mut hailstones = Vec::new();

    FileReader::process_lines("./input/problem24.txt", &mut |line| {
        let captures = regex.captures(line).expect(&format!("{}", line));

        let parse_int = |s: &str| s.parse::<i64>().unwrap();

        let [x_coord, y_coord, z_coord, x_velocity, y_velocity, z_velocity] = [
            &captures[1],
            &captures[2],
            &captures[3],
            &captures[4],
            &captures[5],
            &captures[6],
        ]
        .map(parse_int);

        hailstones.push(HailStone {
            position: Point {
                x: x_coord,
                y: y_coord,
                z: z_coord,
            },
            velocity: Velocity {
                x: x_velocity,
                y: y_velocity,
                z: z_velocity,
            },
        });
    });

    // println!("{:?}", hailstones);

    // (x + t*xv, y + t* yv) = (x' + t'* xv', y' + t'* yv')
    // t*xv - t'*xv' = x' - x
    // t*yv - t'*yv' = y' - y

    // let hailstone_eqs = hailstones.iter().map(|HailStone { position, velocity }|
    //   TwoVarsLinearEquarion {a: velocity.x, b: }
    // );

    let mut amount_of_suitable_intersections = 0;

    for idx in 0..hailstones.len() {
        for jdx in (idx + 1)..hailstones.len() {
            let HailStone {
                position: position1,
                velocity: velocity1,
            } = &hailstones[idx];
            let HailStone {
                position: position2,
                velocity: velocity2,
            } = &hailstones[jdx];

            let eq1 = TwoVarsLinearEquarion {
                a: velocity1.x,
                b: -velocity2.x,
                c: position2.x - position1.x,
            };

            let eq2 = TwoVarsLinearEquarion {
                a: velocity1.y,
                b: -velocity2.y,
                c: position2.y - position1.y,
            };

            if solve_two_eq_linear_system(eq1, eq2)
                .filter(|(intersection_time_x, intersection_time_y)| {
                    if *intersection_time_x <= 0.0 || *intersection_time_y <= 0.0 {
                        return false;
                    }

                    let intersection_point_x =
                        position1.x as f64 + velocity1.x as f64 * intersection_time_x;

                    if intersection_point_x < MIN || intersection_point_x > MAX {
                        return false;
                    }

                    let intersection_point_y =
                        position1.y as f64 + velocity1.y as f64 * intersection_time_x;

                    intersection_point_y >= MIN && intersection_point_y <= MAX
                })
                .is_some()
            {
                // println!("{:?}, {:?}", hailstones[idx], hailstones[jdx]);
                amount_of_suitable_intersections += 1
            }
        }
    }

    println!("{}", amount_of_suitable_intersections)
}

struct TwoVarsLinearEquarion {
    a: i64,
    b: i64,
    c: i64,
}

fn solve_two_eq_linear_system(
    eq1: TwoVarsLinearEquarion,
    eq2: TwoVarsLinearEquarion,
) -> Option<(f64, f64)> {
    let TwoVarsLinearEquarion {
        a: a1,
        b: b1,
        c: c1,
    } = eq1;

    let TwoVarsLinearEquarion {
        a: a2,
        b: b2,
        c: c2,
    } = eq2;

    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0 {
        None
    } else {
        let x = (c1 * b2 - c2 * b1) as f64 / determinant as f64;
        let y = (a1 * c2 - a2 * c1) as f64 / determinant as f64;

        Some((x, y))
    }
}
