#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;

use num::integer::lcm;
use regex::Regex;
use utils::file_reader::FileReader;

struct Node {
    left: String,
    right: String,
}

fn main() {
    let direction_instructions = "LRLRLLRRLRRRLRLRRLRLLRRLRRRLRLRLRLRRLRLLRRRLRRRLLRRLRRLRLRRRLLLRRLRLRLRLRLRLLRRRLRLRRRLRRRLRRRLRRRLRRRLRRRLRRRLRRLRRRLLRLLRRLRRLRRLRRRLLRLRRLRLRLRRLLRLRRRLRRLLRLRLRRRLRRLRRLRRLRLLRLRRRLLLRRRLLLLRRLRRRLLLRRLLRLRLRLLLRRRLLRRRLLLRLRRLLRRRLRRRLRLLRRRLRLRLRLLRRLLRRLRRRLRLRRRLRRLRLRRLRRRR";

    let re = Regex::new(r"^([\S]+) = \(([\S]+), ([\S]+)\)$").unwrap();

    let mut nodes = HashMap::new();

    FileReader::process_lines("./input/problem8.txt", &mut |line| {
        let captures = re.captures(line).unwrap();

        let node = captures[1].to_string();
        let left_ngh = captures[2].to_string();
        let right_ngh = captures[3].to_string();

        nodes.insert(
            node,
            Node {
                left: left_ngh,
                right: right_ngh,
            },
        );
    });

    let calc_steps = |initial_node: &str, termination_condition: fn(&str) -> bool| {
        let mut steps = 0u64;
        let mut node = initial_node;

        let mut direction_instructions_cycle = direction_instructions.chars().cycle();

        while !termination_condition(node) {
            let next_instruction = direction_instructions_cycle.next().unwrap();
            let node_spec = nodes.get(node).unwrap();

            node = if next_instruction == 'L' {
                node_spec.left.as_str()
            } else {
                node_spec.right.as_str()
            };

            steps += 1;
        }

        steps
    };

    let steps = calc_steps("AAA", |node| node == "ZZZ");

    println!("Steps to reach ZZZ: {}", steps); // 20659

    let ending_with_z_cond = |node: &str| node.ends_with('Z');

    let part_two_nodes_steps = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| calc_steps(node, ending_with_z_cond))
        // .map(|node| node.as_str())
        .collect::<Vec<_>>();

    let common_amount_of_steps = part_two_nodes_steps
        .into_iter()
        .reduce(|acc, n| lcm(acc, n))
        .unwrap();

    println!("part 2 answer: {:?}", common_amount_of_steps); // 15690466351717
}
