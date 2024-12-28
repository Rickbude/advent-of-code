use ndarray::Array2;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

#[derive(PartialEq)]
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

fn find_trails(
    map: &Array2<usize>,
    current_pos: (usize, usize),
    end_positions: &mut HashSet<(usize, usize)>,
) -> usize {
    //We found an end position. Store it!
    if map[current_pos] == 9 {
        end_positions.insert(current_pos);
        return 1;
    }
    //Otherwise, explore all four directions
    let directions = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right
    ];
    let mut retval = 0;
    for direction in directions {
        //Consider only steps that increase the current value by exactly one
        let new_pos: (usize, usize) = direction.move_forward(current_pos);
        if map.get(new_pos).is_some() && map[new_pos] == map[current_pos] + 1 {
            retval += find_trails(map, new_pos, end_positions);
        }
    }
    retval
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let chars: Vec<usize> = lines
        .join("")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    //Load the map and get the starting position.
    let map: Array2<usize> = Array2::from_shape_vec((height, width), chars).unwrap();

    //Get the trailhead positions
    let trailheads: Vec<(usize, usize)> = map
        .indexed_iter()
        .filter(|x| x.1 == &0)
        .map(|x| x.0)
        .collect();

    //For each trailhead get the score
    let mut key = 0;
    for trailhead in trailheads {
        let mut end_positions: HashSet<(usize, usize)> = HashSet::new();
        let trails = find_trails(&map, trailhead, &mut end_positions);
        if part == 1 {
            //Unique end points
            key += end_positions.len();
        } else {
            //Unique trails
            key += trails;
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 36);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 81);
    }
}
