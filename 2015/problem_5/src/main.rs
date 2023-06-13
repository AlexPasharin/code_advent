use utils::file_reader::FileReader;

fn main() {
    let mut amount_of_nice_strings = 0;

    FileReader::process_lines("./input.txt", &mut |line| {
        let mut chars = line.chars();

        let prev = chars.next();

        if prev == None {
            return;
        }

        let mut prev = prev.unwrap();

        let mut amount_of_vowels = if is_vowel(&prev) { 1 } else { 0 };
        let mut has_dublicates = false;

        while let Some(curr) = chars.next() {
            if is_vowel(&curr) {
                amount_of_vowels += 1;
            }

            if prev == curr {
                has_dublicates = true;
            } else if (prev == 'a' && curr == 'b')
                || (prev == 'c' && curr == 'd')
                || (prev == 'p' && curr == 'q')
                || (prev == 'x' && curr == 'y')
            {
                return;
            }

            prev = curr
        }

        if amount_of_vowels > 2 && has_dublicates {
            amount_of_nice_strings += 1;
        }
    });

    println!("Amount of nice strings: {amount_of_nice_strings}");
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_vowel(ch: &char) -> bool {
    VOWELS.contains(ch)
}
