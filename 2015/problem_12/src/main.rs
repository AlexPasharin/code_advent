use utils::file_reader::FileReader;

fn main() {
    let mut sum: i32 = 0;
    let mut current_number: i32 = 0;
    let mut current_number_is_negative = false;

    let mut reading_number = false;

    FileReader::process_file("./input.txt", &mut |ch| {
        if *ch == '-' {
            current_number_is_negative = true;

            return;
        }

        if let Some(digit) = ch.to_digit(10) {
            reading_number = true;

            current_number = current_number * 10 + (digit as i32);
        } else if reading_number {
            reading_number = false;

            if current_number_is_negative {
                current_number = -current_number;
                current_number_is_negative = false
            }

            sum += current_number;
            current_number = 0;
        }
    });

    println!("Sum of numbers in given json: {sum}");
}
