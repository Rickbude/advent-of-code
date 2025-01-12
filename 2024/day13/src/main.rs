use std::fs::File;
use std::io::{self, Read};
use regex::Regex;

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let blocks = contents.split("\n\n");

    //Regexes for button A, button B and the price line
    let regex_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").expect("Invalid regex");
    let regex_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").expect("Invalid regex");
    let regex_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)").expect("Invalid regex");

    let mut key = 0;
    for block in blocks{
        let mut lines = block.lines();
        let line_a = lines.next().unwrap();
        let line_b = lines.next().unwrap();
        let line_p = lines.next().unwrap();

        //Capture the effect of button A
        let caps_a = regex_a.captures(line_a).unwrap();
        let a : i64 = caps_a[1].parse().unwrap(); //x a
        let c : i64 = caps_a[2].parse().unwrap(); //y a

        //Capture the effect of button B
        let caps_b = regex_b.captures(line_b).unwrap();
        let b : i64 = caps_b[1].parse().unwrap(); //x b
        let d : i64 = caps_b[2].parse().unwrap(); //y b

        //Determine the goal
        let caps_p = regex_p.captures(line_p).unwrap();
        let mut x_p : i64 = caps_p[1].parse().unwrap(); //b1
        let mut y_p : i64 = caps_p[2].parse().unwrap(); //b2
        if part == 2 {
            x_p += 10000000000000;
            y_p += 10000000000000;
        }

        //Decompose the goal point into the A and B base vectors. To do this, we
        //will solve the small 2x2 linear system
        //  [x_a x_b][a_presses]   [x_p]
        //  [y_a y_b][b_presses] = [y_p]
        //First, determine the determinant of this matrix:
        let det = a*d - b*c;

        //Invert matrix and multiply with right-hand side
        let a_presses = d*x_p - b*y_p; // div by det done later
        let b_presses = a*y_p - c*x_p; // div by det done later

        //Check if we EXACTLY end up at the desired point. If so -> success. The
        //decomposition should be unique, so we don't have to check if this one
        //is the cheapest option.
        if a_presses % det == 0 && b_presses % det == 0{
            key += ((3*a_presses + b_presses) / det) as usize;
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 480);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 875318608908);
    }
}
