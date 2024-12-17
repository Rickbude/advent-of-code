use std::fs::File;
use std::io::{self, Read};

//Check if a given sequence of numbers is "safe" (all increasing, or all
//decreasing, with a step size between 1 and 3)
fn sequence_is_safe(sequence: &[i64]) -> bool {
    let cum_diff: Vec<i64> = sequence.windows(2).map(|w| w[1] - w[0]).collect();
    let safe_increasing = cum_diff.iter().all(|&x| (x >= 1 && x <= 3));
    let safe_decreasing = cum_diff.iter().all(|&x| (x >= -3 && x <= -1));
    safe_increasing || safe_decreasing
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut key: usize = 0;
    for line in contents.lines() {
        //Split each line into individual numbers
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();

        if part == 1 {
            //All numbers must satisfy the requirements.
            if sequence_is_safe(&numbers) {
                key += 1;
            }
        } else {
            for i in 0..numbers.len() {
                //Create a slice containing all items, except the i-th one, and
                //see if that satisfies the requirements.
                let new_numbers = [&numbers[..i], &numbers[i + 1..]].concat();
                if sequence_is_safe(&new_numbers) {
                    key += 1;
                    break;
                }
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 2);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 4);
    }
}
