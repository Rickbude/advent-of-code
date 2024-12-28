use ndarray::Array2;
use rayon::prelude::*;
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
    //Turn 90 degrees to the right
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
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

fn walk_map(
    map: &mut Array2<char>,
    start_pos: (usize, usize),
    max_steps: usize,
    track_steps: bool,
) -> usize {
    let mut direction = Direction::Up;
    let mut position = start_pos;
    let mut steps = 0;
    loop {
        if track_steps {
            map[position] = 'X';
        }

        let new_position = direction.move_forward(position);

        //Guard left the map or ran into a loop
        if map.get(new_position).is_none() || steps == max_steps {
            break;
        }

        //Guard needs to take another step
        if map[new_position] == '#' {
            direction = direction.turn_right();
        } else {
            position = new_position;
            steps += 1;
        }
    }
    steps
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
    let chars: Vec<char> = lines.join("").chars().collect();

    //Load the map and get the starting position.
    let mut map: Array2<char> = Array2::from_shape_vec((height, width), chars).unwrap();
    let mut position: (usize, usize) = (0, 0);
    for (pos, &c) in map.indexed_iter() {
        if c == '^' {
            position = pos;
            break;
        }
    }

    //Walk through the map, discover the guards original path
    let start_pos = position;
    let max_steps = height * width;
    walk_map(&mut map, start_pos, max_steps, true);

    //For part 1, we are done
    if part == 1 {
        let key = map.iter().filter(|&&v| v == 'X').count();
        return Ok(key);
    }

    //Part 2 : Put obstructions in the path, and see if the guard gets trapped
    //in a loop. This can be done in parallel :) Only consider the squares on
    //which the guard originally walked (marked with 'X').
    let key = map
        .indexed_iter()
        .filter(|(_, &c)| c == 'X')
        .par_bridge()
        .filter(|(obstruction, _)| {
            let mut map_clone = map.clone();
            map_clone[*obstruction] = '#';
            walk_map(&mut map_clone, start_pos, max_steps, false) == max_steps
        })
        .count();
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 41);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 6);
    }
}
