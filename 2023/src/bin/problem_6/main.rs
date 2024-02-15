#![warn(clippy::all, clippy::pedantic)]

struct Record {
    time: u64,
    distance: u64,
}

impl Record {
    fn new(spec: (u64, u64)) -> Self {
        Self {
            time: spec.0,
            distance: spec.1,
        }
    }
}

fn main() {
    let input = [(61, 430), (67, 1036), (75, 1307), (71, 1150)];
    let winning_intervals_lengths_prod: u64 = input // [(7, 9), (15, 40), (30, 200)]
        .iter()
        .map(|(time, distance)| {
            let record = Record::new((*time, *distance));

            winning_interval_length(record)
        })
        .product();

    println!("Product: {}", winning_intervals_lengths_prod);

    let part_2_input = input.iter().fold(
        (String::new(), String::new()),
        |result, (time, distance)| {
            (
                result.0 + &time.to_string(),
                result.1 + &distance.to_string(),
            )
        },
    );

    let part_2_time = part_2_input.0.parse::<u64>().unwrap();
    let part_2_distance = part_2_input.1.parse::<u64>().unwrap();

    let part_2_record = Record::new((part_2_time, part_2_distance));

    println!(
        "Part 2 winning interval length: {}",
        winning_interval_length(part_2_record)
    );
}

fn winning_interval_length(record: Record) -> u64 {
    let Record { time, distance } = record;

    // the formula for total_difference is x*(time - x) where x is amount of milliseconds one holds the button
    // hence we solve unequality x*(time - x) > distance i.e. -x^2 + x * time - distance > 0
    // if x_1 and x_2 are roots of  -x^2 + x * time - distance = 0, then the range is (x_1, x_2) (exclusive) is the range of win

    let discriminant = time * time - 4 * distance;

    if discriminant <= 0 {
        return 0;
    }

    let discriminant_sqrt = (discriminant as f64).sqrt();

    let time = time as f64;

    let x1 = (time - discriminant_sqrt) / 2.0;
    let x2 = (time + discriminant_sqrt) / 2.0;

    let mut range_start = x1.ceil() as u64;

    if range_start == x1.floor() as u64 {
        range_start += 1;
    }

    let mut range_end = x2.floor() as u64;

    if range_end == x2.ceil() as u64 {
        range_end -= 1;
    }

    (range_end - range_start + 1).max(0)
}
