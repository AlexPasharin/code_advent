#![warn(clippy::all, clippy::pedantic)]

use std::collections::{BTreeSet, HashMap, HashSet};

use pathfinding::prelude::astar;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

use regex::Regex;
use utils::file_reader::FileReader;

fn main() {
    let initial_molecule = "CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";

    let re = Regex::new(r"^(\S+) => (\S+)$").unwrap();

    let mut replacement_strings_part_1 = HashSet::new();

    let mut replacement_pairs = Vec::new();

    FileReader::process_lines("./input/problem_19.txt", &mut |line| {
        let capture_match = re.captures(line.trim()).unwrap();

        let part = String::from(&capture_match[1]);
        let substitute = String::from(&capture_match[2]);

        let mut start_index = 0;

        while let Some(idx) = (initial_molecule[start_index..]).find(&part) {
            let index = idx + start_index;

            let mut replacement = initial_molecule.to_string();
            replacement.replace_range(index..index + part.len(), &substitute);

            replacement_strings_part_1.insert(replacement);
            start_index = index + 1;
        }

        replacement_pairs.push((part, substitute));
    });

    println!(
        "Part 1: {} possible replacements",
        replacement_strings_part_1.len()
    );

    let astar_result = astar(
        &initial_molecule.to_string(),
        |molecule| {
            let mut nghs = BTreeSet::new();

            for (substitute, part) in &replacement_pairs {
                let mut start_index = 0;

                while let Some(idx) = (molecule[start_index..]).find(part) {
                    let index = idx + start_index;

                    let mut replacement = molecule.clone();

                    replacement.replace_range(index..index + part.len(), substitute);

                    nghs.insert(replacement);

                    start_index = index + part.len();
                }
            }

            nghs.into_iter().map(|neighbour| (neighbour, 1))
        },
        |molecule| molecule.len() - 1,
        |molecule| molecule == "e",
    );

    let part_2_result = astar_result.map(|(path, _)| path.len() - 1).unwrap();

    println!("Part 2: shortest path {:?}", part_2_result);

    let part_2_start_str = initial_molecule.to_string();

    let mut generated_molecules_priority_queue = PriorityQueue::new();
    generated_molecules_priority_queue.push(
        part_2_start_str.clone(),
        Reverse(part_2_start_str.len()),
    );

    let mut generated_molecules = BTreeSet::new();
    let mut distances = HashMap::new();
    distances.insert(part_2_start_str, 0usize);

    let target = "e";

    while let Some((molecule, _)) = generated_molecules_priority_queue.pop() {
        println!("{}", molecule.len());
        let distance = *distances.get(&molecule).unwrap();

        if molecule == target {
            println!("Part two answer is {}", distance);
            break;
        }

        let mut nghs = BTreeSet::new();

        for (substitute, part) in &replacement_pairs {
            let mut start_index = 0;

            while let Some(idx) = (molecule[start_index..]).find(part) {
                let index = idx + start_index;

                let mut replacement = molecule.clone();

                replacement.replace_range(index..index + part.len(), substitute);

                if !generated_molecules.contains(&replacement) {
                    nghs.insert(replacement);
                }

                start_index = index + 1;
            }
        }

        for ngh in nghs {
            let replacement_current_distance = *(distances.get(&ngh).unwrap_or(&usize::MAX));

            let replacement_new_distance = distance + 1;

            if replacement_new_distance < replacement_current_distance {
                let len = ngh.len();

                distances.insert(ngh.clone(), replacement_new_distance);

                let next_priority = replacement_new_distance + len - 1;

                generated_molecules_priority_queue.push(ngh, Reverse(next_priority));
            }
        }

        generated_molecules.insert(molecule);
    }
}

fn replace_all_occuriences(
    heystack: &str,
    needle: &str,
    substitute_needle: &str,
) -> HashSet<String> {
    let mut result = HashSet::new();

    let mut start_index = 0;

    while let Some(idx) = (heystack[start_index..]).find(needle) {
        let index = idx + start_index;

        let mut replacement = heystack.to_string().clone();

        replacement.replace_range(index..index + needle.len(), substitute_needle);

        result.insert(replacement);

        start_index = index + 1;
    }
    result
}
