use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let double_quote_re = Regex::new("^\"(.*)\"$").unwrap();
    let escaped_backslash_re = Regex::new(r"\\\\").unwrap();
    let escaped_quote_re = Regex::new("\\\"").unwrap();
    let escaped_hexa_char_re = Regex::new(r"\\x[0-9a-f]{2}").unwrap();

    let mut part1_result = 0;
    let mut part2_result = 0;

    FileReader::process_lines("./input/problem_8.txt", &mut |line| {
        let line = &double_quote_re.captures(line).unwrap()[1]; // by assumption every line starts and ends with "", we strip those away so that simple matching with \" later won't bring false positives (if line ends with \"")

        // closure that counts how many times a given regex matches in a line
        let count_matches = |re: &Regex| -> usize { re.find_iter(line).count() };

        let escaped_backslash_amount = count_matches(&escaped_backslash_re);
        let escaped_quotes_amount = count_matches(&escaped_quote_re);
        let escaped_hexa_char_amount = count_matches(&escaped_hexa_char_re);

        part1_result +=
            2 + escaped_backslash_amount + escaped_quotes_amount + 3 * escaped_hexa_char_amount;

        part2_result +=
            4 + 2 * (escaped_backslash_amount + escaped_quotes_amount) + escaped_hexa_char_amount;
    });

    println!("Part 1: {}", part1_result); // 1371
    println!("Part 2: {}", part2_result); // 2117
}
