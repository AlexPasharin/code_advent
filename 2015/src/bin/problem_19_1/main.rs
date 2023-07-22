use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
};

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

    let part_2_start_str = initial_molecule.to_string();

    #[derive(Eq, PartialEq)]
    struct PriorityItem {
        molecule: String,
        distance: i32,
        priority: i32,
    }

    impl Ord for PriorityItem {
        fn cmp(&self, other: &PriorityItem) -> Ordering {
            self.priority
                .cmp(&other.priority)
                .then(self.molecule.cmp(&other.molecule))
        }
    }
    impl PartialOrd for PriorityItem {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut generated_molecules_priority_queue = BTreeSet::new();
    generated_molecules_priority_queue.insert(PriorityItem {
        molecule: part_2_start_str.clone(),
        priority: (part_2_start_str.len() as i32),
        distance: 0,
    });

    let mut generated_molecules = BTreeSet::new();

    let target = "e";

    while let Some(PriorityItem {
        molecule, distance, ..
    }) = generated_molecules_priority_queue.pop_first()
    {
        // println!("{}", molecule.len());

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

                replacement.replace_range(index..index + part.len(), &substitute);

                let ngh_in_queue = generated_molecules_priority_queue
                    .iter()
                    .find(|el| el.molecule == replacement);

                if ngh_in_queue.is_some() {
                    // println!("got it");
                    continue;
                }

                if !generated_molecules.contains(&replacement) {
                    nghs.insert(replacement);
                }

                start_index = index + part.len();
            }
        }

        println!("Hello {}", nghs.len());

        for ngh in nghs {
            // let ngh_in_queue = generated_molecules_priority_queue
            //     .iter()
            //     .find(|el| el.molecule == ngh);

            // if ngh_in_queue.is_some() {
            //     continue;
            // }

            // let replacement_current_distance = match ngh_in_queue {
            //     Some(PriorityItem { distance, .. }) => {
            //         panic!("Boo!")
            //     }
            //     None => i32::MAX,
            // };

            let replacement_new_distance: i32 = distance + 1;

            // if replacement_new_distance < replacement_current_distance {
            let len = ngh.len() as i32;

            // if ngh_in_queue.is_some() {
            //     generated_molecules_priority_queue.retain(|el| el.molecule != ngh);
            // }

            let next_priority = replacement_new_distance + len - 1;

            println!("{}", generated_molecules_priority_queue.len());

            generated_molecules_priority_queue.insert(PriorityItem {
                molecule: ngh,
                distance: replacement_new_distance,
                priority: next_priority,
            });
        }
        // }

        generated_molecules.insert(molecule);
    }
}
