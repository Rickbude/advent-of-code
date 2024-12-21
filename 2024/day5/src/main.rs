use std::collections::{HashMap, HashSet};
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
    let mut rules = HashMap::<usize, HashSet<usize>>::new();
    for rule in rules_str.lines() {
        let (page1_str, page2_str) = rule.split_once('|').unwrap();
        let page1: usize = page1_str.parse().unwrap();
        let page2: usize = page2_str.parse().unwrap();
        rules.entry(page1).or_default().insert(page2);
    }

    let mut key = 0;

    //Parse and immediately process the updates (second section of the input)
    for update in updates_str.split_whitespace() {
        //Split the update string into individual pages
        let mut pages: Vec<usize> = update
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        //Use built-in is_sorted_by and sort_by functions, which is an
        //improvement over the previous self-built bubble sort. Found this trick
        //in the solution megathread (SuperSmurfen)
        if pages.is_sorted_by(|a, b| !rules.contains_key(b) || !rules[b].contains(a)) {
            if part == 1 {
                key += &pages[pages.len() / 2];
            }
        } else {
            if part == 2 {
                pages.sort_by(|a, b| (!rules.contains_key(b) || !rules[b].contains(a)).cmp(&true));
                key += &pages[pages.len() / 2];
            }
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
