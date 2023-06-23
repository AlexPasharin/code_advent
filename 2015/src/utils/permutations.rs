// calculates all different permutations of a vector of references
pub fn generate_permutations<'a, T>(vec: &Vec<&'a T>) -> Vec<Vec<&'a T>> {
    if vec.len() == 1 {
        return vec![vec![vec[0]]];
    }

    let mut result = Vec::new();

    for idx in 0..vec.len() {
        let mut smaller_copy = Vec::with_capacity(vec.len() - 1);

        for jdx in 0..vec.len() {
            if idx != jdx {
                smaller_copy.push(vec[jdx]);
            }
        }

        let smaller_permutations = generate_permutations(&smaller_copy);

        for per in smaller_permutations {
            let mut new_permutation = Vec::new();
            new_permutation.push(vec[idx]);

            for permuted_el in per {
                new_permutation.push(permuted_el);
            }

            result.push(new_permutation);
        }
    }

    result
}
