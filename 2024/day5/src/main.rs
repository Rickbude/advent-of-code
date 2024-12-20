use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::{self, Read};

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut parts = contents.split("\n\n");
    let rules_str = parts.next().expect("Count not parse input");
    let updates_str = parts.next().expect("Count not parse input");

    //Parse the rules (the first section of the input) Looking at the test data,
    //a single key can have multiple values. So we will use a hashmap with a
    //hashset to store the values.
    let mut rules: HashMap<usize, HashSet<usize>> = HashMap::new();
    for rule in rules_str.split_whitespace() {
        let mut rule_parts = rule.split('|');
        let page1 = rule_parts.next().unwrap().parse::<usize>().unwrap();
        let page2 = rule_parts.next().unwrap().parse::<usize>().unwrap();
        rules.entry(page1).or_insert_with(HashSet::new).insert(page2);
    }

    let mut key = 0;

    //Parse and immediately process the updates (second section of the input)
    //Apparently this can be done efficiently with a fancy algorithm called a
    //topological sort, but the runtime for this code is already only a few ms..
    for update in updates_str.split_whitespace() {
        //Split the update string into individual pages
        let mut pages: Vec<usize> = update
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        //In part 1, the loop is aborted after a single iteration. In part 2, we
        //keep looping until all updates are corrected.
        let mut fixed_rule: bool = false;
        let mut in_order;
        loop {
            in_order = true;
            for i in 0..pages.len() {
                for j in i + 1..pages.len() {
                    if let Some(dependencies) = rules.get(&pages[j]) {
                        if dependencies.contains(&pages[i]) {
                            in_order = false;
                            //Update contains out-of-order elements, these need
                            //to be swapped.
                            if part == 2 {
                                pages.swap(i, j);
                                fixed_rule = true;
                            }
                        }
                    }
                }
            }
            if part == 1 || in_order {
                break;
            }
        }
        //Add the middle page number
        if (part == 1 && in_order) || (part == 2 && fixed_rule) {
            key += &pages[pages.len() / 2];
        }
    }

    Ok(key)
}

fn main() -> std::io::Result<()> {
    //Calculate the answers to part 1 and 2
    println!("Key part 1: {}", calculate_key(1, "src/input.txt")?);
    println!("Key part 2: {}", calculate_key(2, "src/input.txt")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt1() {
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 143);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 123);
    }
}
