use std::fs::File;
use std::io::prelude::*;
use std::iter::zip;

//Rotate the map 90 degrees, so we can use the same algorithm for checking
//horizontal and vertical reflections
fn rotate_map(map: &String) -> Option<String> {
    let mut rotated_map: Vec<char> = Vec::new();
    let cols = map.lines().next()?.len();
    for col in 0..cols {
        for row in map.lines() {
            rotated_map.push(row.chars().nth(col)?);
        }
        rotated_map.push('\n');
    }
    Some(rotated_map.iter().collect())
}

//Look for a horizontal mirror, that mirrors the map, with a set amount of
//allowed imperfections (0 for part 1, 1 for part 2)
fn find_horizontal_mirror(map: &String, allowed_imperfections: usize) -> Option<usize> {
    let lines: Vec<&str> = map.lines().into_iter().collect();
    let rows = lines.len();
    for row in 1..rows {
        //Set up forward and backwards iterators starting at this row
        let fwd_iter = lines[row..rows].iter();
        let bwd_iter = lines[0..row].iter().rev();
        //Count the number of imperfections, we are done if this is equal to the
        //desired number of imperfections
        let mut imperfections = 0;
        for (fwd, bwd) in zip(fwd_iter, bwd_iter) {
            imperfections += zip(fwd.chars(), bwd.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();
        }
        if imperfections == allowed_imperfections {
            return Some(row);
        }
    }
    None
}

//Find a mirror in the input, and return the row/column it is at
fn find_mirror(map: &String, allowed_imperfections: usize) -> Option<(usize, usize)> {
    //Look for a horizontal mirror first
    let horizontal_mirror = find_horizontal_mirror(&map, allowed_imperfections);
    if horizontal_mirror.is_some() {
        return Some((horizontal_mirror?, 0));
    }
    //If not found, flip the map 90 degrees, and try again
    let vertical_mirror = find_horizontal_mirror(&rotate_map(map)?, allowed_imperfections);
    if vertical_mirror.is_some() {
        return Some((0, vertical_mirror?));
    }
    None
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Each map is separated by a double newline from the previous
    let maps = contents.split("\n\n");

    let mut key: usize = 0;
    for map in maps {
        //For part 2, search for a mirror with exactly one imperfection (this
        //will be the smudge)
        let allowed_imperfections = if part == 1 { 0 } else { 1 };
        let mirror = find_mirror(&map.to_string(), allowed_imperfections)?;
        key += 100 * mirror.0 + mirror.1;
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 405);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 400);
    }
}
