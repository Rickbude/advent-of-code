use std::fs::File;
use std::io::{self, Read};
use memoize::memoize;

//Use the memoize crate to cache the result of this operation (the number of
//distinct stones is limited). We could also do the caching ourselves; all N
//zeros will turn into N ones at the next step etc.
#[memoize]
fn handle_stone(number: usize, remaining_steps: usize) -> usize {
    if remaining_steps == 0 {
        return 1;
    }
    if number == 0 {
        //Replace 0 by 1
        handle_stone(1, remaining_steps - 1)
    } else {
        let num_digits = number.ilog10() + 1;
        if num_digits % 2 == 0 {
            //Even number of digits: split into two stones
            let half = 10usize.pow(num_digits/2);
            handle_stone(number / half, remaining_steps - 1)
                + handle_stone(number % half, remaining_steps - 1)
        } else {
            //Odd number of digits, multiply by 2024
            handle_stone(number * 2024, remaining_steps - 1)
        }
    }
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let blinks =  if part == 1 { 25 } else { 75 };

    let key = contents
        .split_whitespace()
        .flat_map(|x| x.parse())
        .map(|x| handle_stone(x, blinks))
        .sum();

    Ok(key)
}

//Run the script on the full input (both part 1 and part 2)
fn main() -> std::io::Result<()> {
    println!("Key part 1: {}", calculate_key(1, "src/input.txt")?);
    println!("Key part 2: {}", calculate_key(2, "src/input.txt")?);
    Ok(())
}

//Run the script on the test input (run with cargo test)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt1() {
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 55312);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 65601038650482);
    }
}
