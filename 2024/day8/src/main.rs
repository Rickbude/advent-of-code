use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Collect all the nodes, determine the size of the map
    let mut rows = 0;
    let mut cols = 0;
    let mut nodes: HashMap<(i64, i64), char> = HashMap::new();
    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                nodes.insert((row as i64, col as i64), c);
            }
            cols = col as i64 + 1;
        }
        rows = row as i64 + 1;
    }

    //Positions where antinodes may appear
    let harmonics = match part {
        1 => vec![-2, 1],
        2 => (-rows..rows).collect(),
        _ => unreachable!(),
    };

    //Create a list of antinodes
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    for (&(x1, y1), &frequency_1) in &nodes {
        for (&(x2, y2), &frequency_2) in &nodes {
            //Only antennas with the same frequency generate antinodes, and
            //antennas should not interact with themselves
            if frequency_1 != frequency_2 || (x1, y1) == (x2, y2) {
                continue;
            }

            //Calculate distance between antennas
            let (dx, dy) = (x2 - x1, y2 - y1);

            //Extend outwards. Check whether the potential antinodes still fall
            //within the grid.
            for harmonic in &harmonics {
                let x = x1 - harmonic * dx;
                let y = y1 - harmonic * dy;
                if x >= 0 && x < rows && y >= 0 && y < cols {
                    antinodes.insert((x, y));
                }
            }
        }
    }

    Ok(antinodes.len())
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 14);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 34);
    }
}
