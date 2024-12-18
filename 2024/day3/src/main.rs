use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Match mul(\d+,\d+), do() and don't(), the latter are relevant for part 2.
    let re = Regex::new(r"don't\(\)|do\(\)|mul\((\d+),(\d+)\)").expect("Invalid regex");

    //Keep track of whether we should multiply + add or not (part 2)
    let mut mult_enabled = true;
    let mut key: usize = 0;
    for cap in re.captures_iter(&contents) {
        match &cap[0] {
            s if s.starts_with("mul") && mult_enabled => {
                //Base case: perform the multiplication
                let left: usize = cap[1].parse().unwrap();
                let right: usize = cap[2].parse().unwrap();
                key += left * right;
            }
            "don't()" if part == 2 => mult_enabled = false, //Disable multiply
            "do()" if part == 2 => mult_enabled = true,     //Enable  multiply
            _ => {}
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
        assert_eq!(calculate_key(1, "src/test_pt1.txt").unwrap(), 161);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test_pt2.txt").unwrap(), 48);
    }
}
