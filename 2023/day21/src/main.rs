use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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
type Pos = (i64, i64);

//Try to take a step forward.
fn step_forward(old_pos: Pos, direction: Direction) -> Pos {
    //Define what a step in each direction does to row/col number
    let (d_row, d_col): (i64, i64) = match direction {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    };
    let (row, col) = old_pos;
    (row + d_row, col + d_col)
}

fn count_visitable(map: &Vec<char>, rows: usize, cols: usize, max_steps: usize) -> Option<usize> {
    let start_index = map.iter().position(|c| c == &'S')?;
    let start_row = start_index / cols;
    let start_col = start_index % cols;

    //Keep track of states we have seen before
    let mut visited_squares: HashSet<Pos> = HashSet::new();

    //List with states under consideration, accompanied by the current shortest
    //distance. Stored on a min-heap
    let mut active_states: BinaryHeap<Reverse<(usize, Pos)>> = BinaryHeap::new();
    let mut min_distances: HashMap<Pos, usize> = HashMap::new();

    //Insert start position and define end position
    let start_pos: Pos = (start_row as i64, start_col as i64);
    active_states.push(Reverse((0, start_pos)));
    min_distances.insert(start_pos, 0);

    loop {
        //All nodes were visited, but the destination was never reached. Stop!
        if active_states.is_empty() {
            break;
        }

        //Get the current state with the lowest distance
        let Reverse((distance, position)) = active_states.pop()?;

        //Stop exploring this path if we exceed the maximum number of steps
        if distance >= max_steps {
            continue;
        }

        //List of new states to add to the active states
        let mut new_states: Vec<Pos> = Vec::new();

        new_states.push(step_forward(position, Direction::North));
        new_states.push(step_forward(position, Direction::East));
        new_states.push(step_forward(position, Direction::South));
        new_states.push(step_forward(position, Direction::West));

        //Add the new, unseen states to the list with active states
        for state in new_states {
            let new_distance = distance + 1;
            let row = state.0.rem_euclid(rows as i64) as usize;
            let col = state.1.rem_euclid(cols as i64) as usize;
            if map[row * cols + col] == '#' {
                continue;
            }
            let old_distance = min_distances.entry(state).or_insert(new_distance);
            if new_distance < *old_distance {
                *old_distance = new_distance;
            }
            if !visited_squares.contains(&state) {
                active_states.push(Reverse((new_distance, state)));
                visited_squares.insert(state);
            }
        }
    }

    Some(
        min_distances
            .values()
            .filter(|v| *v % 2 == max_steps % 2)
            .count(),
    )
}

//Calculate the answer to part 1 / 2
fn calculate_key(
    part: usize,
    max_steps: usize,
    n_values: Option<Vec<usize>>,
    filename: &str,
) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Read the map
    let rows = contents.lines().into_iter().count();
    let cols = contents.lines().next()?.len();
    let map: Vec<char> = contents.replace("\n", "").chars().collect();

    if part == 1 || n_values.is_none() {
        Some(count_visitable(&map, rows, cols, max_steps)?)
    } else {
        //For part 2, assume that the pattern eventually settles into a
        //quadratic equation:   y = ax^2 + bx + c
        //For the test input this pattern is achieved for n>=4, while
        //for the actual input, this pattern is already found from n=0

        //Determine a,b and c, by sampling the problem with 3 different
        //values for x (n1 -> y1, n2 -> y2, n3 -> y3), and use linear
        //algebra to find a,b,c by solving the Vandermonde matrix:
        // [ n1^2  n1  1 ] [a]   [ y1 ]
        // [ n2^2  n2  1 ] [b] = [ y2 ]
        // [ n3^3  n3  1 ] [c]   [ y3 ]

        // xn = n*rows + steps%rows
        let n_values_copy = n_values?.clone();
        let y_values: Vec<usize> = n_values_copy
            .clone()
            .iter()
            .map(|n| n * rows + max_steps % rows)
            .map(|x| count_visitable(&map, rows, cols, x).unwrap())
            .collect();

        //Unpack the n and y vectors for shorter mathematical expressions below
        let n1 = n_values_copy[0] as i64;
        let n2 = n_values_copy[1] as i64;
        let n3 = n_values_copy[2] as i64;
        let y1 = y_values[0] as i64;
        let y2 = y_values[1] as i64;
        let y3 = y_values[2] as i64;

        //These expressions have been derived using Matlab's symbolic math
        //package. A shorter equivalent might be found with other extrapolation methods
        let determinant = (n1 - n2) * (n1 - n3) * (n2 - n3);
        let a = n1 * (y3 - y2) + n2 * (y1 - y3) + n3 * (y2 - y1);
        let b = n1 * n1 * (y2 - y3) + n2 * n2 * (y3 - y1) + n3 * n3 * (y1 - y2);
        let c = n1 * n1 * (y3 * n2 - y2 * n3)
            + n2 * n2 * (y1 * n3 - y3 * n1)
            + n3 * n3 * (y2 * n1 - y1 * n2);

        let n = (max_steps / rows) as i64;
        let y = (a * n * n + b * n + c) / determinant;
        Some(y as usize)
    }
}

fn main() -> std::io::Result<()> {
    //Calculate the answers to part 1 and 2
    println!(
        "Key part 1: {}",
        calculate_key(1, 64, None, "src/input.txt").unwrap()
    );
    println!(
        "Key part 2: {}",
        calculate_key(2, 26501365, Some(vec![0, 1, 2]), "src/input.txt").unwrap()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(calculate_key(1, 6, None, "src/test.txt").unwrap(), 16);
    }

    #[test]
    fn test_pt2_1() {
        assert_eq!(calculate_key(2, 10, None, "src/test.txt").unwrap(), 50);
    }

    #[test]
    fn test_pt2_2() {
        assert_eq!(
            calculate_key(2, 50, Some(vec![4, 5, 6]), "src/test.txt").unwrap(),
            1594
        );
    }

    #[test]
    fn test_pt2_3() {
        assert_eq!(
            calculate_key(2, 100, Some(vec![4, 5, 6]), "src/test.txt").unwrap(),
            6536
        );
    }

    #[test]
    fn test_pt2_4() {
        assert_eq!(
            calculate_key(2, 500, Some(vec![4, 5, 6]), "src/test.txt").unwrap(),
            167004
        );
    }

    #[test]
    fn test_pt2_5() {
        assert_eq!(
            calculate_key(2, 1000, Some(vec![4, 5, 6]), "src/test.txt").unwrap(),
            668697
        );
    }

    #[test]
    fn test_pt2_6() {
        assert_eq!(
            calculate_key(2, 5000, Some(vec![4, 5, 6]), "src/test.txt").unwrap(),
            16733044
        );
    }
}
