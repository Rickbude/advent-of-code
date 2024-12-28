use std::fs::File;
use std::io::{self, Read};

//Check if a particular set of numbers can form a particular target number,
//using a provided set of operators. We will check backwards, to prune away as
//many combinations as possible.
fn can_form(numbers: &[usize], active_operators: &[char], target: usize) -> bool {
    //There are no more numbers (should not happen in principle..)
    if numbers.is_empty() {
        return false;
    }
    //Only one number left: only works if this is equal to the target.
    if numbers.len() == 1 {
        return numbers[0] == target;
    }

    //Split of the last number
    let (&last_number, remaining_numbers) = numbers.split_last().unwrap();

    for &operator in active_operators {
        let remainder: Option<usize> = match operator {
            '*' => {
                //Check if the last operator can be a multiplication, which
                //means the target must be divisible by the last number.
                if target % last_number == 0 {
                    //Dividide
                    Some(target / last_number)
                } else {
                    None
                }
            }
            '+' => {
                //Check if the last operator can be an addition, which only
                //means the target must be larger than the last number.
                if target >= last_number {
                    Some(target - last_number)
                } else {
                    None
                }
            }
            '|' => {
                //Check if the last operator can be a concatenation, which means
                //the digits of the target value must end in the digits of the
                //last number. Can be done with string manipulation, or a bit of
                //math. The latter is faster.
                let magnitude = 10usize.pow(last_number.ilog10() + 1);
                if target % magnitude == last_number {
                    Some(target / magnitude)
                } else {
                    None
                }
            }
            _ => unreachable!(),
        };

        //Operator is valid, check if a smaller subset may form the remainder.
        if remainder.is_some() && can_form(remaining_numbers, active_operators, remainder.unwrap())
        {
            return true;
        }
    }
    //None of the operators provided a valid way forward -> dead end
    false
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut key = 0;

    //Select the operators that we have to take into account
    let active_operators = match part {
        1 => vec!['*', '+'],
        2 => vec!['*', '+', '|'],
        _ => unreachable!(),
    };

    for line in contents.lines() {
        //Parse the line, a colon separates the test value from the other
        //whitespace-separated numbers.
        let parts = line.split_once(":").unwrap();
        let test_value: usize = parts.0.parse().unwrap();
        let numbers: Vec<usize> = parts
            .1
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        //Check if a given combination of numbers can form a certain target
        //value, with a given set of operators
        if can_form(&numbers, &active_operators, test_value) {
            key += test_value;
        }
    }
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 3749);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 11387);
    }
}
