use utils::file_reader::FileReader;

fn main() {
    let mut reader = FileReader::new("./input.txt");

    let mut floor = 0;
    let mut position = 0;

    reader.process_until(&mut |ch| {
        process_next_direction(&mut floor, &ch);
        position += 1;

        floor == -1
    });

    println!("First basement occurance at position {position}"); // 1771

    reader.process_all(&mut |ch| {
        process_next_direction(&mut floor, &ch);
    });

    println!("Final floor is {}", floor); // 138
}

fn process_next_direction(floor: &mut i32, next_direction: &char) {
    if *next_direction == '(' {
        *floor += 1;
    } else if *next_direction == ')' {
        *floor -= 1;
    }
}
