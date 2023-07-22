use std::collections::HashSet;

fn main() {
    let s = "abbababa";

    let occ = replace_all_occuriences(s, vec![("ab", ""), ("ba", "xxxx")]);

    for o in occ {
        println!("{}", o);
    }

    println!();
    println!("{}", s);
}

fn replace_all_occuriences(
    heystack: &str,
    replacement_pairs: Vec<(&str, &str)>,
) -> HashSet<String> {
    let mut result = HashSet::new();

    for (needle, substitute_needle) in replacement_pairs {
        let mut start_index = 0;

        while let Some(idx) = (heystack[start_index..]).find(needle) {
            let index = idx + start_index;

            let mut replacement = heystack.to_string().clone();

            replacement.replace_range(index..index + needle.len(), substitute_needle);

            result.insert(replacement);

            start_index = index + 1;
        }
    }

    result
}
