use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

enum Direction {
    North,
    East,
    South,
    West,
}

//Print the map, useful when debugging
fn print_map(
    rows: usize,
    cols: usize,
    moving_rocks: &HashSet<(usize, usize)>,
    static_rocks: &HashSet<(usize, usize)>,
) {
    for row in 0..rows {
        for col in 0..cols {
            if moving_rocks.contains(&(row, col)) {
                print!("O");
            } else if static_rocks.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
    println!();
}

// col bins: start/end rows of empty space, per column
// row bins: start/end cols of empty space, per row
fn tilt_platform(
    direction: Direction,
    rows: usize,
    cols: usize,
    moving_rocks: &mut HashSet<(usize, usize)>,
    static_rocks: &HashSet<(usize, usize)>,
) {
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
    let mut new_moving_rocks: HashSet<(usize, usize)> = HashSet::new();
    for outer in outer_range {
        let mut place_rock_at = 0;
        for (index, inner) in inner_range.iter().enumerate() {
            let (row, col) = match direction {
                Direction::North | Direction::South => (*inner, outer),
                Direction::West | Direction::East => (outer, *inner),
            };
            let rock = (row, col);
            //Found a moving rock. Move the rock and update the position of the
            //last encountered rock
            if moving_rocks.contains(&rock) {
                let new_rock = match direction {
                    Direction::North | Direction::South => (inner_range[place_rock_at], col),
                    Direction::East | Direction::West => (row, inner_range[place_rock_at]),
                };
                new_moving_rocks.insert(new_rock);
                place_rock_at += 1;
            }
            //We found a new static rock
            if static_rocks.contains(&rock) {
                place_rock_at = index + 1;
            }
        }
    }
    *moving_rocks = new_moving_rocks;
}

//Perform a full counterclockwise cycle of the platform
fn do_cycle(
    rows: usize,
    cols: usize,
    moving_rocks: &mut HashSet<(usize, usize)>,
    static_rocks: &HashSet<(usize, usize)>,
) {
    tilt_platform(Direction::North, rows, cols, moving_rocks, static_rocks);
    tilt_platform(Direction::West, rows, cols, moving_rocks, static_rocks);
    tilt_platform(Direction::South, rows, cols, moving_rocks, static_rocks);
    tilt_platform(Direction::East, rows, cols, moving_rocks, static_rocks);
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Collect the starting positions of the rocks (moving and static)
    let mut moving_rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut static_rocks: HashSet<(usize, usize)> = HashSet::new();

    let rows = contents.lines().into_iter().count();
    let cols = contents.lines().next()?.len();
    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                static_rocks.insert((row, col));
            } else if c == 'O' {
                moving_rocks.insert((row, col));
            }
        }
    }

    if part == 1 {
        //For part 1, all we need to do is tilt the platform to the north
        tilt_platform(
            Direction::North,
            rows,
            cols,
            &mut moving_rocks,
            &static_rocks,
        );
    } else {
        let mut states: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
        let mut cycle = 1;
        loop {
            println!("{}", cycle);
            do_cycle(rows, cols, &mut moving_rocks, &static_rocks);
            let mut rocks: Vec<(usize, usize)> = moving_rocks.iter().map(|x| *x).collect();
            rocks.sort();
            if states.contains_key(&rocks) {
                println!("Repetition found after {} cycles!!", cycle);
                println!(
                    "This state was earlier encountered at cycle {}",
                    states.get(&rocks)?
                );
                let diff = cycle - states.get(&rocks)?;
                let remaining = (1000000000 as i64 - cycle as i64) as usize;
                cycle += (remaining / diff) * diff
            } else {
                states.insert(rocks, cycle);
            }
            let key: usize = moving_rocks.iter().map(|x| rows - x.0).sum();
            println!("key:{}", key);
            if cycle == 1000000000 {
                break;
            }
            cycle += 1;
        }
    }
    let key: usize = moving_rocks.iter().map(|x| rows - x.0).sum();

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
