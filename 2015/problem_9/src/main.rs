use std::{
    cmp::{max, min},
    collections::HashMap,
};

use regex::Regex;
use utils::file_reader::FileReader;

/*
  This solution generates all permutations of destinations and for each calculates the distance of a corresponding path

  Cons:
  - generate_permutations is recurssive
  - we keep all generated permutations in memory simultaneously

  subfolder problem_9_1 presents a solution which uses "backtracking", generating only one path at a time
*/

fn main() {
    let re = Regex::new(r"^(\S+)\s+to\s+(\S+)\s+=\s+(\d+)$").unwrap();

    let mut distances: HashMap<String, HashMap<String, i32>> = HashMap::new();

    // process input to put all distances in the map "distances"
    FileReader::process_lines("./input.txt", &mut |line| {
        let captures = re.captures(line).unwrap();

        let destination1 = &captures[1];
        let destination2 = &captures[2];
        let distance: i32 = captures[3].parse().unwrap();

        let mut add_distance_to_map = |source: &str, destination: &str| {
            let value = distances
                .entry(source.to_string())
                .or_insert_with(|| HashMap::new());
            value.insert(destination.to_string(), distance)
        };

        // to keep logic simple, we put every distance in the map twice, since it can be travelled in both directions
        // we could instead save some memory and save it only once, but that would complicated the logic of extracting the value of distance between two destinations
        add_distance_to_map(destination1, destination2);
        add_distance_to_map(destination2, destination1);
    });

    // collect all destinations in the vector
    let all_destinations: Vec<&String> = distances.keys().into_iter().collect();

    let permutations = generate_permutations_t(&all_destinations);

    let mut smallest_distance: i32 = std::i32::MAX;
    let mut biggest_distance: i32 = -1;

    // for each permutation calculate its length and update smallest_distance and biggest_distance
    for per in &permutations {
        let mut per_distance = 0;

        for i in 1..per.len() {
            let source = per[i - 1];
            let destination = per[i];

            let distance = distances.get(source).unwrap().get(destination).unwrap();

            per_distance += distance;
        }

        smallest_distance = min(smallest_distance, per_distance); // 251
        biggest_distance = max(biggest_distance, per_distance); // 898
    }

    println!("Smallest distance: {}", smallest_distance);
    println!("Biggest distance: {}", biggest_distance);
}

// recurssive generates all permutations of a given vector of references

fn generate_permutations_t<'a, T: PartialEq>(vec: &Vec<&'a T>) -> Vec<Vec<&'a T>> {
    if vec.len() == 1 {
        return vec![vec![vec[0].clone()]];
    }

    let mut result = Vec::new();

    for el in vec {
        let mut smaller_copy = Vec::with_capacity(vec.len() - 1);

        for other_el in vec {
            if *other_el != *el {
                smaller_copy.push(*other_el);
            }
        }

        let smaller_permutations = generate_permutations_t(&smaller_copy);

        for per in smaller_permutations {
            let mut new_permutation = Vec::new();
            new_permutation.push(*el);

            for permuted_el in per {
                new_permutation.push(permuted_el);
            }

            result.push(new_permutation);
        }
    }

    result
}
