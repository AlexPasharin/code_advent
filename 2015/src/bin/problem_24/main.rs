use utils::file_reader::FileReader;

fn main() {
    let mut packages_weights = vec![];
    FileReader::process_lines("./input/problem_24.txt", &mut |line| {
        let number: u64 = line.parse().unwrap();
        packages_weights.push(number);
    });

    packages_weights.sort();

    let minimal_quantum_entaglement_for_three = devide_packages(&packages_weights, 3);
    println!(
        "Minimal entaglement when divide into 3 parts:  {}",
        minimal_quantum_entaglement_for_three
    ); // 10439961859

    let minimal_quantum_entaglement_for_four = devide_packages(&packages_weights, 4);
    println!(
        "Minimal entaglement when divide into 4 parts:  {}",
        minimal_quantum_entaglement_for_four
    ); // 72050269
}

fn devide_packages(packages_weights: &Vec<u64>, number_of_parts: u64) -> u64 {
    let third_of_total_weights = packages_weights.iter().sum::<u64>() / number_of_parts;

    let amount_of_packages = packages_weights.len();

    struct WeightSet {
        weight_indices: Vec<usize>,
        sum: u64,
        product: u64,
    }

    let mut previous_subsets: Vec<WeightSet> = Vec::with_capacity(amount_of_packages);

    for (idx, weight) in packages_weights.iter().enumerate() {
        let weight_value = *weight;

        let set = WeightSet {
            weight_indices: vec![idx],
            sum: weight_value,
            product: weight_value,
        };

        previous_subsets.push(set);
    }

    let mut minimal_quantum_entaglement = u64::MAX;

    while previous_subsets.len() > 0 {
        let mut current_subsets: Vec<WeightSet> = vec![];

        for s in previous_subsets {
            let WeightSet {
                weight_indices,
                sum,
                product,
            } = s;

            let max_index = weight_indices.last().unwrap();

            for idx in max_index + 1..amount_of_packages {
                let weight = packages_weights[idx];
                let new_weight = weight + sum;
                let new_product = product * weight;

                if new_product >= minimal_quantum_entaglement {
                    break;
                }

                if new_weight < third_of_total_weights {
                    let mut new_weight_indices = weight_indices.clone();
                    new_weight_indices.push(idx);

                    current_subsets.push(WeightSet {
                        weight_indices: new_weight_indices,
                        sum: new_weight,
                        product: new_product,
                    });
                } else if new_weight == third_of_total_weights {
                    // check if rest can be devided into number_of_parts amount of parts evenly

                    let mut new_weight_indices = weight_indices.clone();
                    new_weight_indices.push(idx);

                    let mut rest_weights = Vec::with_capacity(packages_weights.len());
                    let mut running_index = 0;

                    for (idx, weight) in packages_weights.iter().enumerate() {
                        if running_index < new_weight_indices.len()
                            && idx == new_weight_indices[running_index]
                        {
                            running_index += 1;
                        } else {
                            rest_weights.push(*weight);
                        }
                    }

                    if has_combination_with_sum(rest_weights, third_of_total_weights) {
                        minimal_quantum_entaglement = new_product;
                    }
                } else {
                    break;
                }
            }
        }

        if minimal_quantum_entaglement < u64::MAX {
            break;
        }

        previous_subsets = current_subsets;
    }

    return minimal_quantum_entaglement;
}

fn has_combination_with_sum(numbers: Vec<u64>, goal_sum: u64) -> bool {
    let mut previous_subsets = Vec::with_capacity(numbers.len());

    struct IndexSet {
        indices: Vec<usize>,
        sum: u64,
    }

    for (idx, number) in numbers.iter().enumerate() {
        let number_value = *number;

        let set = IndexSet {
            indices: vec![idx],
            sum: number_value,
        };

        previous_subsets.push(set);
    }

    let amount_of_numbers = numbers.len();

    while previous_subsets.len() > 0 {
        let mut current_subsets = vec![];

        for IndexSet { indices, sum } in previous_subsets {
            let max_index = indices.last().unwrap();

            for idx in max_index + 1..amount_of_numbers {
                let number = numbers[idx];
                let new_sum = sum + number;

                if new_sum < goal_sum {
                    let mut new_indices = indices.clone();
                    new_indices.push(idx);

                    current_subsets.push(IndexSet {
                        indices: new_indices,
                        sum: new_sum,
                    });
                } else if new_sum == goal_sum {
                    return true;
                } else {
                    break;
                }
            }
        }

        previous_subsets = current_subsets;
    }

    return false;
}
