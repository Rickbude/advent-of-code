use memoize::memoize;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::iter;

//Count the number of valid configurations. Use memoization to speed this up dramatically.
#[memoize]
fn count_valid(springs: Vec<char>, mut groups: VecDeque<usize>) -> usize {
    //All groups are assigned
    if groups.len() == 0 {
        //Valid, but only if there are no #'s remaining
        return (springs.iter().filter(|x| x == &&'#').count() == 0) as usize;
    }
    //Assigning all groups would exceed the remaining string length -> invalid!
    if springs.len() < groups.iter().sum::<usize>() + groups.len() - 1 {
        return 0;
    }
    //There are not enough spaces left to fit all the groups -> invalid!
    if springs.iter().filter(|c| c != &&'.').count() < groups.iter().sum() {
        return 0;
    }

    let mut valid_count = 0;

    //Try to place the first unplaced group in any of the starting positions (#/?)
    let group = groups[0];
    groups.pop_front().unwrap();

    //Find the location of the first #. This will be the last acceptable
    //starting position for the first group. If no # is present, the maximum
    //start position is this group's size before the end of the string
    let first_hashtag = springs
        .iter()
        .position(|x| x == &'#')
        .unwrap_or(springs.len());
    let max_start_pos = std::cmp::min(first_hashtag, springs.len() - group);

    //Work through the valid starting positions
    for start_pos in 0..max_start_pos + 1 {
        let end_pos = start_pos + group;
        //Check if the entire group would fit here (not interrupted by a . )
        if springs[start_pos..end_pos]
            .iter()
            .filter(|x| x == &&'.')
            .count()
            > 0
        {
            continue;
        }

        //If the character right next to the group we want to place is a # , this
        //is also an invalid placement.
        if end_pos < springs.len() && springs[end_pos] == '#' {
            continue;
        }

        //We can place the new group. Try to put the next groups in the
        //remainder of the string (segment starting at end_pos)
        if end_pos + 1 < springs.len() {
            let new_springs: Vec<char> = springs[end_pos + 1..springs.len()].to_vec();
            valid_count += count_valid(new_springs, groups.clone());
        } else {
            valid_count += count_valid(Vec::new(), groups.clone());
        }
    }
    valid_count
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, contents: &str) -> Option<i64> {
    let lines = contents.lines();
    let mut key: i64 = 0;
    let repeats = if part == 1 { 1 } else { 5 };

    for line in lines {
        //Separate the map from the groups
        let mut iter = line.split_whitespace();

        //Get the map (the left part). Optionally repeated 5 times, joined with a '?' for part 2
        let springs = iter::repeat(iter.next()?)
            .take(repeats)
            .collect::<Vec<_>>()
            .join("?")
            .chars()
            .collect();

        //Get the groups (rhe right part). Optionally repeated 5 times, joined with a ',' for part 2
        let group_string = iter::repeat(iter.next()?)
            .take(repeats)
            .collect::<Vec<_>>()
            .join(",");

        //Parse groups into numbers
        let groups: VecDeque<usize> = group_string
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        //Count valid matches
        key += count_valid(springs, groups) as i64;
    }
    Some(key)
}

fn main() -> std::io::Result<()> {
    //Open input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Calculate the answers to part 1 and 2
    println!("Key part 1: {}", calculate_key(1, &contents).unwrap());
    println!("Key part 2: {}", calculate_key(2, &contents).unwrap());

    Ok(())
}

#[cfg(test)]
mod tests{    
    use super::*;
    #[test]
    fn do_test() {
        //Open test file
        let mut test_file = File::open("src/test.txt").unwrap();
        let mut contents = String::new();
        test_file.read_to_string(&mut contents).unwrap();

        //Check if the tests pass
        assert_eq!(calculate_key(1, &contents).unwrap(),21);
        assert_eq!(calculate_key(2, &contents).unwrap(),525152);
    }
}