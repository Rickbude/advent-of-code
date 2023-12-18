use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

//Position on the map: (x,y)
type Pos = (usize, usize);
//Active paths: (position,direction,consecutive,heat loss)
type State = (Pos, Direction, usize);

//Try to take a step forward. Return None if we collided with the edge of the map
fn step_forward(pos: Pos, direction: Direction, rows: usize, cols: usize) -> Option<Pos> {
    //Define what a step in each direction does to row/col number
    let (d_row, d_col): (i64, i64) = match direction {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    };
    //Check if the new row/column is within bounds, and return if so
    let new_row = pos.0 as i64 + d_row;
    let new_col = pos.1 as i64 + d_col;
    if new_row >= 0 && new_col >= 0 && new_row < rows as i64 && new_col < cols as i64 {
        return Some((new_row as usize, new_col as usize));
    }
    None
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Create map
    let rows = contents.lines().into_iter().count();
    let cols = contents.lines().next()?.len();
    let map: Vec<usize> = contents
        .replace("\n", "")
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    //Minimum and maximum number of consecutive steps
    let min_consecutive = if part == 1 { 0 } else { 4 };
    let max_consecutive = if part == 1 { 3 } else { 10 };

    //Keep track of states we have seen before
    let mut visited_states: HashSet<State> = HashSet::new();

    //List with states under consideration, accompanied by the current shortest
    //distance. Stored on a min-heap
    let mut active_states: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

    //Insert start position and define end position
    let start_pos: Pos = (0, 0);
    let end_pos: Pos = (rows - 1, cols - 1);
    active_states.push(Reverse((0, (start_pos, Direction::East, 0))));
    active_states.push(Reverse((0, (start_pos, Direction::South, 0))));

    loop {
        //All nodes were visited, but the destination was never reached. Stop!
        if active_states.is_empty() {
            break;
        }

        //Get the current state with the lowest distance
        let Reverse((min_dist, min_state)) = active_states.pop()?;
        let ((row, col), direction, consecutive) = min_state;

        //Stop if we reached the end position
        if (row, col) == end_pos {
            return Some(min_dist);
        }

        //List of new states to add to the active states
        let mut new_states: Vec<State> = Vec::new();

        //Less than max_consecutive steps taken in same direction => Try to step further
        if consecutive < max_consecutive {
            let new_pos = step_forward((row, col), direction, rows, cols);
            if new_pos.is_some() {
                new_states.push((new_pos?, direction, consecutive + 1));
            }
        }

        //If we have taken at least min_consecutive steps forwards, turning is allowed
        if consecutive >= min_consecutive {
            //Define what is left and what is right
            let (left, right): (Direction, Direction) = match direction {
                Direction::North => (Direction::West, Direction::East),
                Direction::East => (Direction::South, Direction::North),
                Direction::South => (Direction::East, Direction::West),
                Direction::West => (Direction::North, Direction::South),
            };

            //Try to turn left, and step forward
            let new_pos_left = step_forward((row, col), left, rows, cols);
            if new_pos_left.is_some() {
                new_states.push((new_pos_left?, left, 1));
            }

            //Try to turn right, and step forward
            let new_pos_right = step_forward((row, col), right, rows, cols);
            if new_pos_right.is_some() {
                new_states.push((new_pos_right?, right, 1));
            }
        }

        //Add the new, unseen states to the list with active states
        for state in new_states {
            let new_index = state.0 .0 * cols + state.0 .1;
            let new_heat_loss = min_dist + map[new_index];
            if !visited_states.contains(&state) {
                active_states.push(Reverse((new_heat_loss, state)));
                visited_states.insert(state);
            }
        }
    }

    Some(0)
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 102);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 94);
    }
}
