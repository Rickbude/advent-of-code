use std::cmp::{max, min};
use std::fs::File;
use std::io::prelude::*;

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Split the two columns of numbers into their own vectors
    let (mut left, mut right): (Vec<usize>, Vec<usize>) = contents
        .lines()
        .map(|line| {
            let mut numbers = line.split_whitespace();
            (
                numbers.next().unwrap().parse::<usize>().unwrap(),
                numbers.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip();

    let key: usize;
    if part == 1 {
        left.sort();
        right.sort();
        //Count the absolute distance between the two lists
        key = left
            .iter()
            .zip(right.iter())
            .map(|(x, y)| max(x, y) - min(x, y))
            .sum();
    } else {
        //Count how often the entries in the right list are contained in the
        //left list
        key = left
            .iter()
            .map(|&x| x * (right.iter().filter(|&&y| y == x).count()))
            .sum();
    }

    Some(key)
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 11);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 31);
    }
}
