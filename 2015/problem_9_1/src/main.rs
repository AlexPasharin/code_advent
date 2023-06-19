use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, LinkedList},
};

use regex::Regex;
use utils::file_reader::FileReader;

/*
  This solution uses backtracking instead of generating all permutations recurssively (as in repo problem_9)
  Input is taken from problem_9 subfolder

  First we generate map of mutual distance and the vector of all detinations, giving us the fixed order of them
  We generate possible pathes using indices of destinations in that vector as their ids
*/

struct Path {
    path: LinkedList<usize>, // path currently being constructed. we always add new destinations to the beginning of the list
    used_destinations: HashSet<usize>, // this dublicates previous path as a form of a set, for fast access of information if destination is already in the path
    length: i32, // this is the length (distance() of the path currently being constructed
    next_index: usize, // next index that needs to be visited. if equals to the amount of destinations, that means that all possibilities has been exausted, so we need to backtrack
}

fn main() {
    let re = Regex::new(r"^(\S+)\s+to\s+(\S+)\s+=\s+(\d+)$").unwrap();

    let mut distances: HashMap<String, HashMap<String, i32>> = HashMap::new();

    // process input to put all distances in the map "distances"
    FileReader::process_lines("../problem_9/input.txt", &mut |line| {
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
    let all_destinations: Vec<String> = distances
        .keys()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let amount_of_destinations = all_destinations.len();

    let mut smallest_distance: i32 = std::i32::MAX;
    let mut biggest_distance: i32 = -1;

    let mut path = Path {
        path: LinkedList::new(),
        used_destinations: HashSet::new(),
        next_index: 0,
        length: 0,
    };

    // a bit unsafe helper that returns distance between destinations when they are given by their index in "all_destinations" vector
    // will panic if called with indices out of bounds, but we dont call it in such a situation
    let get_distance_by_indices = |source_index: usize, destination_index: usize| {
        let source = &all_destinations[source_index];
        let destination = &all_destinations[destination_index];

        distances.get(source).unwrap().get(destination).unwrap()
    };

    let update_next_index = |path: &mut Path, starting_index| {
        path.next_index = amount_of_destinations;

        for i in starting_index..amount_of_destinations {
            if !path.used_destinations.contains(&i) {
                path.next_index = i;

                break;
            }
        }
    };

    // closure that backtracks the current path i.e. removes the current front element and gets back to its parent (if there is one) trying to continue with parent's next unprocessed child
    let backtrack = |path: &mut Path| {
        // remove the head from path (should exist, since path is not empty)
        let head = path.path.pop_front().unwrap();
        path.used_destinations.remove(&head);

        // if after that path is still not empty, adjust its length
        if let Some(new_head) = path.path.front() {
            path.length -= get_distance_by_indices(head, *new_head);
        }

        // update next_index - find a next available index after "head" which was just removed
        update_next_index(path, head + 1);
    };

    loop {
        // if we would only be interested in part 1 i.e. the smallest distance, we could backtrack immediately if path gets too big
        // if path.length > smallest_distance {
        //     backtrack(&mut path);

        //     continue;
        // }

        // path is finished, update smallest_distance and biggest_distance
        if path.path.len() == amount_of_destinations {
            smallest_distance = min(smallest_distance, path.length);
            biggest_distance = max(biggest_distance, path.length);
        }

        // case: we can continue constructing current path by adding "next_index" destination to it
        if path.next_index < amount_of_destinations {
            // update path length, if it is not empty - it is current length plus the distance between the current head and new destination (which is "next_index")
            // we could skip that and calculate the whole length only when path is finished, in that case we wouldn't need "length" variable in Path struct
            // this approach saves some recalculations and also has the advantage to make it possible to stop generating a path half way through, if it is for example too long
            // we don't use that advantage here, but would be handy for example if we would only be interested in the shortest path

            // note that it is more convinient to calculate the new distance first and only then actually add a new destination, because we can use the current head in the calculation of distance but it wont be head anymore if we add the new destination first
            if let Some(head) = path.path.front() {
                path.length += get_distance_by_indices(*head, path.next_index);
            }

            // add a new destination to the path
            path.path.push_front(path.next_index);
            path.used_destinations.insert(path.next_index);

            // update next_index - it is the first destination index which is not visited yet (or amount_of_destinations if all are visited already)
            update_next_index(&mut path, 0);
        // case: this path cannot be continued, since all possibilies has already been explored but path is not empty
        } else if path.path.len() > 0 {
            backtrack(&mut path);

        // case: path cannot be continued, since all possibilies has already been explored and path itself is empty
        } else {
            break;
        }
    }

    println!("Smallest distance: {}", smallest_distance); // 251
    println!("Biggest distance: {}", biggest_distance); // 898
}
