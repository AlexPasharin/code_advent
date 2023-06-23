use std::collections::HashMap;

use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let split_by_arrow = Regex::new(r"\s+->\s+").unwrap();

    let mut wire_data = HashMap::new();

    FileReader::process_lines("./input/problem_7.txt", &mut |line| {
        let line_parsed = split_by_arrow.split(line).collect::<Vec<&str>>();

        let wire_name = String::from(line_parsed[1]);
        let source = String::from(line_parsed[0]);

        wire_data.insert(wire_name, source);
    });

    let mut wire_values = HashMap::new();

    let result1 = calc_signal_memo("a", &wire_data, &mut wire_values);

    println!("Part 1 result: {result1}"); // 46065

    let mut updated_wire_values = HashMap::<String, u16>::new();
    updated_wire_values.insert(String::from("b"), result1);

    let result2 = calc_signal_memo("a", &wire_data, &mut updated_wire_values);

    println!("Part 2 result: {result2}"); // 14134
}

// memoization is needed bc otherwise there are too many recalculations and solutions is too slow
fn calc_signal_memo(
    wire_name: &str,
    wire_data: &HashMap<String, String>,
    wire_values: &mut HashMap<String, u16>,
) -> u16 {
    // if value is already memoized return it
    if let Some(value) = wire_values.get(wire_name) {
        return *value;
    }

    // if wire_name is actually a number, return it
    if let Ok(value) = wire_name.parse::<u16>() {
        return value;
    }

    let mut value_fn = || {
        let source = wire_data
            .get(wire_name)
            .expect(&format!("Wire {wire_name} not found from the spec"));

        // if source for wire is a number, return it
        if let Ok(value) = source.parse::<u16>() {
            return value;
        }

        let split_by_white_space = Regex::new(r"\s+").unwrap();

        let parse_as_gate = split_by_white_space.split(source).collect::<Vec<&str>>();

        // case when source is another wire
        if parse_as_gate.len() == 1 {
            return calc_signal_memo(parse_as_gate[0], wire_data, wire_values);
        }

        // case when source is NOT gate
        if parse_as_gate[0] == "NOT" {
            let wire_name = parse_as_gate[1];

            return !calc_signal_memo(wire_name, wire_data, wire_values);
        }

        // otherwise source is a gate of types AND, OR, LSHIFT or RSHIFT

        let wire_1 = parse_as_gate[0];
        let wire_2 = parse_as_gate[2];

        let operator = parse_as_gate[1];

        let val1 = calc_signal_memo(wire_1, wire_data, wire_values);
        let val2 = calc_signal_memo(wire_2, wire_data, wire_values);

        return match operator {
            "AND" => val1 & val2,
            "OR" => val1 | val2,
            "LSHIFT" => val1 << val2,
            "RSHIFT" => val1 >> val2,
            _ => panic!("Could not calculate signal"),
        };
    };

    let value = value_fn();
    wire_values.insert(wire_name.to_string(), value);

    return value;
}
