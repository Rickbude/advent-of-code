use ndarray::Array2;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    //Move one step forward.
    fn move_forward(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (position.0.wrapping_sub(1), position.1),
            Direction::Down => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1.wrapping_sub(1)),
            Direction::Right => (position.0, position.1 + 1),
        }
    }
}

//Flood fill map, starting from a given starting position (pos). Produces filled
//area as output, and keeps track of list of fences (position + direction)
//around the filled area
fn flood_fill(
    map: &Array2<char>,
    pos: (usize, usize),
    visited: &mut Array2<bool>,
    fences: &mut Vec<(usize, usize, Direction)>,
) -> usize {
    let directions = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    visited[pos] = true;
    let mut area = 1;
    for direction in directions {
        //Consider only steps that increase the current value by exactly one
        let new_pos: (usize, usize) = direction.move_forward(pos);
        //Stepped out of bounds or new position is not part of this group. This
        //is precisely where we need a fence!
        if map.get(new_pos).is_none() || map[new_pos] != map[pos] {
            fences.push((pos.0, pos.1, direction));
            continue;
        }
        //New neighbor, continue the flood fill from here
        if !visited[new_pos] {
            area += flood_fill(map, new_pos, visited, fences);
        }
    }
    area
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Load the map
    let lines: Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let chars: Vec<char> = lines.join("").chars().collect();
    let map: Array2<char> = Array2::from_shape_vec((height, width), chars).unwrap();

    //Keep track of which plots have been assigned
    let mut visited: Array2<bool> = Array2::from_elem((height, width), false);

    let mut key = 0;
    for ((row, col), &_c) in map.indexed_iter() {
        if !visited[(row, col)] {
            let mut fences: Vec<(usize, usize, Direction)> = Vec::new();
            let area = flood_fill(&map, (row, col), &mut visited, &mut fences);
            let perim = fences.len();
            if part == 1 {
                //Part 1: just count the number of fences
                key += area * perim;
            } else {
                //Part 2: count unique sides: fences facing the same direction
                //that are only a distance of 1 apart (either row or col) are
                //directly next to each other, and we get a discount for those
                let mut discount = 0;
                for &(row1, col1, dir1) in &fences {
                    for &(row2, col2, dir2) in &fences {
                        let d_row = usize::abs_diff(row1, row2);
                        let d_col = usize::abs_diff(col1, col2);
                        if dir1 == dir2 && d_row + d_col == 1 {
                            discount += 1;
                        }
                    }
                }
                key += area * (perim - discount / 2);
            }
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
    fn test_pt1_1() {
        assert_eq!(calculate_key(1, "src/test1.txt").unwrap(), 140);
    }

    #[test]
    fn test_pt1_2() {
        assert_eq!(calculate_key(1, "src/test2.txt").unwrap(), 772);
    }

    #[test]
    fn test_pt1_3() {
        assert_eq!(calculate_key(1, "src/test3.txt").unwrap(), 1930);
    }

    #[test]
    fn test_pt2_1() {
        assert_eq!(calculate_key(2, "src/test1.txt").unwrap(), 80);
    }

    #[test]
    fn test_pt2_2() {
        assert_eq!(calculate_key(2, "src/test2.txt").unwrap(), 436);
    }

    #[test]
    fn test_pt2_3() {
        assert_eq!(calculate_key(2, "src/test3.txt").unwrap(), 1206);
    }

    #[test]
    fn test_pt2_4() {
        assert_eq!(calculate_key(2, "src/test4.txt").unwrap(), 236);
    }

    #[test]
    fn test_pt2_5() {
        assert_eq!(calculate_key(2, "src/test5.txt").unwrap(), 368);
    }
}
