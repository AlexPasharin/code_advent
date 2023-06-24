use std::{
    cmp::{max, min},
    collections::HashMap,
    i32,
};

use utils::{file_reader::FileReader, permutations::generate_permutations};

use regex::Regex;

/*
  Here we use simple approach to track all pathes using all permutations, as in problem_9 bin
  We could certainly also have "more clever" approach used in problem_9_1 bin
*/

fn main() {
    let re =
        Regex::new(r"^(\S+) would (gain|lose) (\d+) happiness units by sitting next to (\S+).$")
            .unwrap();

    let mut happiness_units_map: HashMap<String, HashMap<String, i32>> = HashMap::new();

    FileReader::process_lines("./input/problem_13.txt", &mut |line| {
        let capture_match = re.captures(line.trim()).unwrap();

        let name1 = String::from(&capture_match[1]);
        let name2 = String::from(&capture_match[4]);

        let would_lose = &capture_match[2] == "lose";

        let mut happiness_units = *&capture_match[3].parse::<i32>().unwrap();

        if would_lose {
            happiness_units = -happiness_units;
        }

        happiness_units_map
            .entry(name1)
            .or_default()
            .insert(name2, happiness_units);
    });

    /*
      without cloned below this will be a vector of &String which entries are (shared) references to the same strings that are keys of happiness_units_map
      this will prevent attempts to borrow happiness_units_map as mutable later in the for loop, since it also uses "people"
    */
    let mut people: Vec<String> = happiness_units_map.keys().cloned().collect();

    /*
      Currently happiness_units_map contains "one sided" happiness units
      Next we calculate the "mutual" symmetric distance between two persons and store it in the corresponding entry in happiness_units_map

      for loop to go throgh all pairs of people exactly ones
    */
    for i in 0..people.len() {
        for j in (i + 1)..people.len() {
            // first we borrow values of happiness_units_map at people[i] and people[j] unmutably so that we can read the happiness units and add them together
            // note - we cannot use closure that uses happiness_units_map directly here as defined and used below, because that would mean happiness_units_map would have to be borrowed immutabily for the whole for loop, which wont work because we also borrow it mutably here in the same loop
            // such clojure could be defined here locally, inside j loop, but that is hardly worth it
            let happiness_units_1 =
                get_happiness_units(&happiness_units_map, &people[i], &people[j]);
            let happiness_units_2 =
                get_happiness_units(&happiness_units_map, &people[j], &people[i]);

            let happiness_units = happiness_units_1 + happiness_units_2;

            // to actually update the values we have to borrow again mutably
            // note that we couldnt use get_mut originally to read the values as well, coz that would result into two mutable borrows of the whole map
            // for the same reason we cannot here first take two mutable borrows and then insert
            // we have to take first mutable borrow, update it, then take the second
            // essentially this is because we know we update two DIFFERENT entries, but compiler does not know that, could be same entries

            update_happiness_units(
                &mut happiness_units_map,
                &people[i],
                &people[j],
                happiness_units,
            );

            update_happiness_units(
                &mut happiness_units_map,
                &people[j],
                &people[i],
                happiness_units,
            );
        }
    }

    // since people will sit around the table and we calculate happiness units in cycle, we can fix one person and permutate only the rest
    // this will save some calculations
    let fixed_person = people.pop().unwrap();

    let permutations = generate_permutations(&people.iter().collect()); // people is not a vector of references, so we have to map it to one first, using iter.collect trick

    let mut part1_max_happiness: i32 = i32::MIN;
    let mut part2_max_happiness: i32 = i32::MIN;

    let happiness_units = |source: &String, destination: &String| {
        get_happiness_units(&happiness_units_map, source, destination)
    };

    for per in permutations {
        // we start by adding happiness score between fixed person and its neighbours - first and the last elements of the permutation
        let mut permutation_overall_happiness_score = happiness_units(per[0], &fixed_person)
            + happiness_units(per[per.len() - 1], &fixed_person);

        let mut smallest_subdistance = i32::MAX;

        // then we add all other scores
        // we also keep track of the smallest happiness score between two persons, in smallest_subdistance
        // the smallest score corresponds to a place where "me" person from part 2 would wanna come in between, to maximaze the overall happiness score in part 2
        for i in 1..per.len() {
            let source = per[i - 1];
            let destination = per[i];

            let distance = happiness_units(source, destination);
            permutation_overall_happiness_score += distance;

            smallest_subdistance = min(smallest_subdistance, distance);
        }

        part1_max_happiness = max(part1_max_happiness, permutation_overall_happiness_score);
        part2_max_happiness = max(
            part2_max_happiness,
            permutation_overall_happiness_score - smallest_subdistance,
        );
    }

    println!("Part 1 max happiness: {}", part1_max_happiness); // 733
    println!("Part 2 max happiness: {}", part2_max_happiness); // 725
}

fn get_happiness_units(
    happiness_units_map: &HashMap<String, HashMap<String, i32>>,
    source: &String,
    destination: &String,
) -> i32 {
    *happiness_units_map
        .get(source)
        .unwrap()
        .get(destination)
        .unwrap()
}

fn update_happiness_units(
    happiness_units_map: &mut HashMap<String, HashMap<String, i32>>,
    source: &String,
    destination: &String,
    new_value: i32,
) {
    happiness_units_map
        .get_mut(source)
        .unwrap()
        .insert(destination.clone(), new_value);
}
