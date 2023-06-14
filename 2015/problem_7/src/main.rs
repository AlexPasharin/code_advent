use std::collections::HashMap;

use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let split_by_arrow = Regex::new(r"\s+->\s+").unwrap();

    let mut wire_data = HashMap::<String, String>::new();

    FileReader::process_lines("./input.txt", &mut |line| {
        let line_parsed = split_by_arrow.split(line).collect::<Vec<&str>>();

        let wire_name = line_parsed[1].to_string();
        let source = line_parsed[0].to_string();

        wire_data.insert(wire_name, source);
    });

    let mut wire_values = HashMap::<String, u16>::new();

    let result1 = calc_signal_memo("a", &wire_data, &mut wire_values);

    println!("Part 1 result: {result1}"); // 46065

    let mut updated_wire_values = HashMap::<String, u16>::new();
    updated_wire_values.insert("b".to_string(), result1);

    let result2 = calc_signal_memo("a", &wire_data, &mut updated_wire_values);

    println!("Part 2 result: {result2}"); // 14134
}

fn calc_signal_memo(
    wire_name: &str,
    wire_data: &HashMap<String, String>,
    wire_values: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(value) = wire_values.get(wire_name) {
        return *value;
    }

    let mut value_fn = || {
        if let Ok(value) = wire_name.parse::<u16>() {
            return value;
        }

        let source = wire_data
            .get(wire_name)
            .expect(&format!("Wire {wire_name} not found from the spec"));

        if let Ok(value) = source.parse::<u16>() {
            return value;
        }

        let split_by_white_space = Regex::new(r"\s+").unwrap();

        let parse_as_gate = split_by_white_space.split(source).collect::<Vec<&str>>();

        if parse_as_gate.len() == 1 {
            return calc_signal_memo(parse_as_gate[0], wire_data, wire_values);
        }

        if parse_as_gate[0] == "NOT" {
            let wire_name = parse_as_gate[1];

            return !calc_signal_memo(wire_name, wire_data, wire_values);
        }

        let wire_1 = parse_as_gate[0];
        let wire_2 = parse_as_gate[2];

        let operator = parse_as_gate[1];

        if operator == "AND" {
            return calc_signal_memo(wire_1, wire_data, wire_values)
                & calc_signal_memo(wire_2, wire_data, wire_values);
        }

        if operator == "OR" {
            return calc_signal_memo(wire_1, wire_data, wire_values)
                | calc_signal_memo(wire_2, wire_data, wire_values);
        }

        if operator == "LSHIFT" {
            return calc_signal_memo(wire_1, wire_data, wire_values)
                << wire_2.parse::<u16>().unwrap();
        }

        if operator == "RSHIFT" {
            return calc_signal_memo(wire_1, wire_data, wire_values)
                >> wire_2.parse::<u16>().unwrap();
        }

        panic!("Could not calculate signal");
    };

    let value = value_fn();
    wire_values.insert(wire_name.to_string(), value);

    return value;
}
