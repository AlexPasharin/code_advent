#![warn(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, ops::Range};

use regex::Regex;
use utils::file_reader::FileReader;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

enum ComparisonOperator {
    Less,
    More,
}

enum UnconditionalRule {
    Accepted,
    Rejected,
    Workflow(String),
}

struct ComparisonRule {
    category: Category,
    operator: ComparisonOperator,
    limit: u64,
    result: UnconditionalRule,
}

enum Rule {
    Comparison(ComparisonRule),
    UnconditionalRule(UnconditionalRule),
}

// struct Restriction {
//     category: Category,
//     range: Range<i32>,
// }

struct Node<'a> {
    restrictions: HashMap<Category, Range<u64>>,
    next_worflow: &'a str,
}

// struct Restrictions {
//   X: Restriction,
//   M,
//   A,
//   S,
// }

use Category::*;
use ComparisonOperator::*;
use Rule::*;
use UnconditionalRule::*;

fn main() {
    let workflow_regex = Regex::new(r"^([^{]+)\{([^}]+)\}$").unwrap();
    let part_regex = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
    let conditional_rule_regex: Regex = Regex::new(r"^(x|m|a|s)(<|>)(\d+):(.+)$").unwrap();

    let rule_from = |instruction_text: &str| {
        if let Some(conditional_rule_parts) = conditional_rule_regex.captures(instruction_text) {
            Comparison(ComparisonRule {
                category: letter_to_category(&conditional_rule_parts[1]),
                operator: if &conditional_rule_parts[2] == ">" {
                    More
                } else {
                    Less
                },
                limit: conditional_rule_parts[3].parse::<u64>().unwrap(),
                result: str_to_result(&conditional_rule_parts[4]),
            })
        } else {
            UnconditionalRule(str_to_result(instruction_text))
        }
    };

    let mut workflows = HashMap::new();

    let mut workflows_reading_mode = true;

    let mut accepted_parts_rating_sum = 0;

    FileReader::process_lines("./input/problem19.txt", &mut |line| {
        if line.is_empty() {
            workflows_reading_mode = false;
            return;
        }

        if workflows_reading_mode {
            let captures = workflow_regex.captures(line).unwrap();

            let workflow_name = captures[1].to_string();
            let workflow_instructions = captures[2].split(',').map(rule_from).collect::<Vec<_>>();

            workflows.insert(workflow_name, workflow_instructions);
        } else {
            let captures = part_regex.captures(line).unwrap();

            let ratings = [(X, 1), (M, 2), (A, 3), (S, 4)]
                .map(|(category, idx)| (category, captures[idx].parse::<u64>().unwrap()));

            let ratings = HashMap::from(ratings);

            let mut workflow_name = String::from("in");

            let part_accepted = 'outer: loop {
                let rules = workflows.get(&workflow_name).unwrap();

                let applied_rule = rules
                    .iter()
                    .find_map(|rule| match rule {
                        Comparison(ComparisonRule {
                            category,
                            operator,
                            limit,
                            result,
                        }) => {
                            let rating = ratings[category];

                            let rule_applies = match operator {
                                More => rating > *limit,
                                Less => rating < *limit,
                            };

                            if rule_applies {
                                Some(result)
                            } else {
                                None
                            }
                        }
                        UnconditionalRule(rule) => Some(rule),
                    })
                    .unwrap();

                match applied_rule {
                    Rejected => break 'outer false,
                    Accepted => break 'outer true,
                    Workflow(workflow) => workflow_name = workflow.to_string(),
                }
            };

            if part_accepted {
                accepted_parts_rating_sum += ratings.values().sum::<u64>();
            }
        }
    });

    println!(
        "Sum of accepted parts rating: {}",
        accepted_parts_rating_sum //287054
    );

    let mut tree_nodes_queue = Vec::new();

    let initial_restrictions = HashMap::from([X, M, A, S].map(|c| (c, 1..(4000 + 1))));

    tree_nodes_queue.push(Node {
        restrictions: initial_restrictions,
        next_worflow: "in",
    });

    let mut accepted_restrictions_sum = 0;

    while !tree_nodes_queue.is_empty() {
        let mut next_tree_nodes_queue = Vec::new();

        for Node {
            restrictions,
            next_worflow,
        } in tree_nodes_queue
        {
            let workflow_rules = workflows.get(next_worflow).unwrap();

            let mut current_restrictions = restrictions.clone();

            for rule in workflow_rules {
                match rule {
                    Comparison(ComparisonRule {
                        category,
                        operator,
                        limit,
                        result,
                    }) => {
                        let current_range = current_restrictions.get(category).unwrap();
                        let start = current_range.start;
                        let end = current_range.end;
                        let limit = *limit;

                        let (range, complementary_range) = match operator {
                            Less => {
                                let middle = end.min(limit);

                                (start..middle, middle..end)
                            }
                            More => {
                                let middle = start.max(limit + 1);

                                (middle..end, start..middle)
                            }
                        };

                        // let (range, complementary_range) = (start..middle, middle..end);

                        if !range.is_empty() {
                            match result {
                                Accepted => {
                                    let mut accepted_restrictions = current_restrictions.clone();
                                    accepted_restrictions.insert(*category, range);

                                    accepted_restrictions_sum += accepted_restrictions
                                        .values()
                                        .map(|range| range.end - range.start)
                                        .product::<u64>();
                                }
                                Workflow(name) => {
                                    let mut restrictions = current_restrictions.clone();
                                    restrictions.insert(*category, range);

                                    next_tree_nodes_queue.push(Node {
                                        restrictions,
                                        next_worflow: name,
                                    })
                                }
                                _ => {}
                            }
                        }

                        if complementary_range.is_empty() {
                            break;
                        }

                        current_restrictions.insert(*category, complementary_range);
                    }
                    UnconditionalRule(rule) => {
                        match rule {
                            Accepted => {
                                accepted_restrictions_sum += current_restrictions
                                    .values()
                                    .map(|range| range.end - range.start)
                                    .product::<u64>();
                            }
                            Workflow(name) => next_tree_nodes_queue.push(Node {
                                restrictions: current_restrictions,
                                next_worflow: name,
                            }),
                            _ => {}
                        }

                        break;
                    }
                }
            }
        }

        tree_nodes_queue = next_tree_nodes_queue;
    }

    println!(
        "Sum of accepted parts rating, part 2: {}",
        accepted_restrictions_sum
    ); // 131619440296497
}

fn letter_to_category(letter: &str) -> Category {
    match letter {
        "x" => X,
        "m" => M,
        "a" => A,
        "s" => S,
        _ => unreachable!(),
    }
}

fn str_to_result(text: &str) -> UnconditionalRule {
    match text {
        "R" => Rejected,
        "A" => Accepted,
        _ => Workflow(text.to_string()),
    }
}
