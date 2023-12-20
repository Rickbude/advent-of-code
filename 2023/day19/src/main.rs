use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Part = HashMap<char, usize>;

//Parse line containing information about a part into a hashmap. The input
//information is formatted like: {x=787,m=2655,a=1222,s=2876}
fn line_to_part(line: &str) -> Option<Part> {
    let result: Part = line
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .filter_map(|pair| {
            let mut iter = pair.split('=');
            let key = iter.next()?.to_string().chars().nth(0)?;
            let value = iter.next()?.parse::<usize>().ok()?;
            Some((key, value))
        })
        .collect();
    Some(result)
}

#[derive(Debug)]
enum Rule {
    More {
        category: char,
        value: usize,
        target: String,
    },
    Less {
        category: char,
        value: usize,
        target: String,
    },
    Pass {
        target: String,
    },
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

//Parse a rule
fn parse_rule(rule_str: &str) -> Option<Rule> {
    //Split rule at semicolon. It is a passthrough rule if there are two parts,
    //otherwise its a comparison rule
    let mut iter = rule_str.split(":");
    let rule_str = iter.next()?;
    if let Some(target_str) = iter.next() {
        //Comparison rule found. Split at the > or < sign, and extract threshold
        let cmp_pos = rule_str.find(|c| c == '<' || c == '>');
        let (category, value_str) = rule_str.split_at(cmp_pos?);
        let value = value_str[1..].parse::<usize>().ok()?;
        if rule_str.contains('>') {
            //Set up greater-than rule
            Some(Rule::More {
                category: category.chars().nth(0)?,
                value,
                target: target_str.to_string(),
            })
        } else {
            //Set up smaller-than rule
            Some(Rule::Less {
                category: category.chars().nth(0)?,
                value,
                target: target_str.to_string(),
            })
        }
    } else {
        //Pass-through rule found
        Some(Rule::Pass {
            target: rule_str.to_string(),
        })
    }
}

// Parse line containing Workflow information
fn line_to_workflow(line: &str) -> Option<Workflow> {
    //Extract workflow name and get rid of curly brackets
    let mut parts = line.split('{');
    let workflow_name = parts.next()?.to_string();
    let workflow_str = parts.next()?.trim_end_matches('}');

    //Parse the rules strings into rule structures
    let rules_list: Vec<Rule> = workflow_str.split(',').filter_map(parse_rule).collect();

    Some(Workflow {
        name: workflow_name,
        rules: rules_list,
    })
}

fn run_workflow(
    workflow: String,
    range: &(Part, Part),
    workflows: &HashMap<String, Workflow>,
    part: usize,
) -> Option<usize> {
    //Workflow was rejected
    if workflow == "R" {
        return Some(0);
    }

    //Workflow was accepted
    if workflow == "A" {
        let mut result = 1;
        if part == 1 {
            result = range.0.values().sum();
        } else {
            for cat in "xmas".chars() {
                result *=
                    ((*range.1.get(&cat)? as i64 - *range.0.get(&cat)? as i64).abs() + 1) as usize;
            }
        }
        return Some(result);
    }

    let mut score: usize = 0;
    let mut current_range = range.clone();
    //Process rules (possibly recursively)
    for rule in workflows.get(&workflow)?.rules.iter() {
        match rule {
            Rule::Pass { target } => {
                score += run_workflow(target.clone(), &current_range, workflows, part)?;
                break;
            }
            Rule::More {
                category,
                value,
                target,
            } => {
                //Entire range passes
                if current_range.0.get(category)? > &value {
                    score += run_workflow(target.clone(), &current_range, workflows, part)?;
                    break;
                }
                //Entire range fails
                if current_range.1.get(category)? <= &value {
                    continue;
                }

                //Range needs to be split in two
                let mut split_part_1 = current_range.1.clone();
                let mut split_part_2 = current_range.0.clone();
                split_part_1.entry(*category).and_modify(|v| *v = *value);
                split_part_2
                    .entry(*category)
                    .and_modify(|v| *v = *value + 1);

                let new_range: (Part, Part) = (split_part_2, current_range.1.clone());
                current_range.1 = split_part_1;
                score += run_workflow(target.clone(), &new_range, workflows, part)?;
                continue;
            }
            Rule::Less {
                category,
                value,
                target,
            } => {
                //Entire range passes
                if current_range.1.get(category)? < &value {
                    score += run_workflow(target.clone(), &current_range, workflows, part)?;
                    break;
                }
                //Entire range fails
                if current_range.0.get(category)? >= &value {
                    continue;
                }

                //Range needs to be split in two
                let mut split_part_1 = current_range.1.clone();
                let mut split_part_2 = current_range.0.clone();
                split_part_1
                    .entry(*category)
                    .and_modify(|v| *v = *value - 1);
                split_part_2.entry(*category).and_modify(|v| *v = *value);

                let new_range: (Part, Part) = (current_range.0.clone(), split_part_1);
                current_range.0 = split_part_2;
                score += run_workflow(target.clone(), &new_range, workflows, part)?;
                continue;
            }
        }
    }

    Some(score)
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Separate workflows and machine parts
    let mut blocks = contents.split("\n\n");
    let workflows_string = blocks.next()?;
    let parts_string = blocks.next()?;

    let workflows: HashMap<String, Workflow> = workflows_string
        .split_whitespace()
        .filter_map(line_to_workflow)
        .map(|w| (w.name.clone(), w))
        .collect();

    if part == 1 {
        let key: usize = parts_string
            .split_whitespace()
            .filter_map(line_to_part)
            .filter_map(|part| {
                run_workflow(
                    "in".to_string(),
                    &(part.clone(), part.clone()),
                    &workflows,
                    1,
                )
            })
            .sum();
        Some(key)
    } else {
        let range_start: Part = Part::from([('x', 1), ('m', 1), ('a', 1), ('s', 1)]);
        let range_end: Part = Part::from([('x', 4000), ('m', 4000), ('a', 4000), ('s', 4000)]);
        let key = run_workflow("in".to_string(), &(range_start, range_end), &workflows, 2)?;
        Some(key)
    }
}

fn main() -> std::io::Result<()> {
    //Calculate the answers to part 1 and 2
    println!("Key part 1: {}", calculate_key(1, "src/input.txt").unwrap());
    println!("Key part 2: {}", calculate_key(2, "src/input.txt").unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 19114);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 167409079868000);
    }
}
