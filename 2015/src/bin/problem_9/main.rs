use regex::Regex;
use std::{
    cmp::{max, min},
    collections::HashMap,
    i32,
};

use utils::{file_reader::FileReader, permutations::generate_permutations};

/*
  This solution generates all permutations of destinations and for each calculates the distance of a corresponding path

  Cons:
  - generate_permutations is recurssive
  - we keep all generated permutations in memory simultaneously
  - we go through all permutations, altrough there a permutation and its reverse have the same length, so we calculate that twice

  subfolder problem_9_1 presents a solution which uses "backtracking", generating only one path at a time.
  it also tries to optimize by not exploring further pathes that would lead to a reverse of a path already considered.
*/

fn main() {
    let re = Regex::new(r"^(\S+)\s+to\s+(\S+)\s+=\s+(\d+)$").unwrap();

    let mut distances: HashMap<String, HashMap<String, i32>> = HashMap::new();

    // process input to put all distances in the map "distances"
    FileReader::process_lines("./input/problem_9.txt", &mut |line| {
        let captures = re.captures(line).unwrap();

        let destination1 = &captures[1];
        let destination2 = &captures[2];
        let distance: i32 = captures[3].parse().unwrap();

        let mut add_distance_to_map = |source: &str, destination: &str| {
            let value = distances
                .entry(String::from(source))
                .or_insert_with(|| HashMap::new());
            value.insert(String::from(destination), distance)
        };

        // to keep logic simple, we put every distance in the map twice, since it can be travelled in both directions
        // we could instead save some memory and save it only once, but that would complicated the logic of extracting the value of distance between two destinations
        add_distance_to_map(destination1, destination2);
        add_distance_to_map(destination2, destination1);
    });

    // collect all destinations in the vector
    let all_destinations: Vec<&String> = distances.keys().collect();

    let permutations = generate_permutations(&all_destinations);

    let mut smallest_distance: i32 = i32::MAX;
    let mut biggest_distance: i32 = -1;

    // for each permutation calculate its length and update smallest_distance and biggest_distance
    for per in permutations {
        let mut permutation_distance = 0;

        for i in 1..per.len() {
            let source = per[i - 1];
            let destination = per[i];

            let distance = distances.get(source).unwrap().get(destination).unwrap();

            permutation_distance += distance;

            // if we would only be interested in smallest distance (part 1), we could cut the calculations if distance gets too big here with following:
            // if permutation_distance > smallest_distance {
            //     break;
            // }
        }

        smallest_distance = min(smallest_distance, permutation_distance); // 251
        biggest_distance = max(biggest_distance, permutation_distance); // 898
    }

    println!("Smallest distance: {}", smallest_distance);
    println!("Biggest distance: {}", biggest_distance);
}
