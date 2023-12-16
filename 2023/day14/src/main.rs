use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

enum Direction {
    North,
    East,
    South,
    West,
}

// col bins: start/end rows of empty space, per column
// row bins: start/end cols of empty space, per row
fn tilt_platform(direction: Direction, rows: usize, cols: usize, map: &mut Vec<char>) {
    //Outer loop runs over columns or rows, depending on direction
    let inner_range: Vec<usize> = match direction {
        Direction::North => (0..rows).into_iter().collect(),
        Direction::South => (0..rows).into_iter().rev().collect(),
        Direction::East => (0..cols).into_iter().rev().collect(),
        Direction::West => (0..cols).into_iter().collect(),
    };

    //Inner loop runs over columns or rows, depending on direction
    let outer_range = match direction {
        Direction::North | Direction::South => (0..cols).into_iter(),
        Direction::East | Direction::West => (0..rows).into_iter(),
    };

    //Update rock positions
    let mut new_map = map.clone();
    for outer in outer_range {
        //Insertion position for moving rocks
        let mut place_rock_at = 0;
        for (inner_index, inner) in inner_range.iter().enumerate() {
            //Get linear index into the map. Depending on whether we iterate
            //N<->S or E<->W, the inner loop may be rows/cols
            let (row, col) = match direction {
                Direction::North | Direction::South => (*inner, outer),
                Direction::West | Direction::East => (outer, *inner),
            };
            let linear_index = row * cols + col;

            match map[linear_index] {
                'O' => {
                    //Found a moving rock. Move the rock in the map copy, and
                    //update the position of the last encountered rock
                    let new_linear_index = match direction {
                        Direction::North | Direction::South => {
                            inner_range[place_rock_at] * cols + col
                        }
                        Direction::East | Direction::West => {
                            row * cols + inner_range[place_rock_at]
                        }
                    };
                    new_map[linear_index] = '.';
                    new_map[new_linear_index] = 'O';
                    place_rock_at += 1;
                }
                '#' => {
                    //Found a new static rock. Update the insertion position
                    place_rock_at = inner_index + 1;
                }
                _ => {
                    continue;
                }
            };
        }
    }
    *map = new_map;
}

//Perform a full counterclockwise cycle of the platform
fn do_cycle(rows: usize, cols: usize, map: &mut Vec<char>) {
    tilt_platform(Direction::North, rows, cols, map);
    tilt_platform(Direction::West, rows, cols, map);
    tilt_platform(Direction::South, rows, cols, map);
    tilt_platform(Direction::East, rows, cols, map);
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let rows = contents.lines().into_iter().count();
    let cols = contents.lines().next()?.len();
    let mut map: Vec<char> = contents.replace("\n", "").chars().collect();

    if part == 1 {
        //For part 1, all we need to do is tilt the platform to the north
        tilt_platform(Direction::North, rows, cols, &mut map);
    } else {
        //For part 2, the platform is rotated periodically. Keep track over
        //which states (maps including moving rocks) we have seen. Note that we
        //are hashing way too much, we would only have to hash the positions of
        //the moving rocks.
        let mut states: HashMap<Vec<char>, usize> = HashMap::new();
        let number_of_cycles = 1000000000;
        for cycle in 1..number_of_cycles {
            //Update the map
            do_cycle(rows, cols, &mut map);
            if states.contains_key(&map) {
                //We encountered this state before. Lookup the projected final state
                let prev_cycle = *states.get(&map)?;
                let cycle_length = cycle - prev_cycle;
                let remaining_cycles = (number_of_cycles as i64 - cycle as i64) as usize;
                let final_cycle = remaining_cycles % cycle_length + prev_cycle;
                map = states
                    .iter()
                    .find_map(|(key, &val)| if val == final_cycle { Some(key) } else { None })?
                    .to_vec();
                break;
            } else {
                //We have not seen this state before, add it to the list.
                states.insert(map.clone(), cycle);
            }
        }
    }
    //Extract answer (inverted row positions) from the map
    let key: usize = map
        .iter()
        .enumerate()
        .map(|(index, c)| if c == &'O' { rows - (index / cols) } else { 0 })
        .sum();
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 136);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 64);
    }
}
