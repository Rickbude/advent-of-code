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

//submap row/column + position on that submap
type State = (Pos, Pos);

//Try to take a step forward.
fn step_forward(old_pos: State, direction: Direction, rows: usize, cols: usize) -> State {
    //Define what a step in each direction does to row/col number
    let (d_row, d_col): (i64, i64) = match direction {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    };
    //Potentially step to a new map copy / garden tile
    let ((map_row, map_col), (row, col)) = old_pos;
    let mut new_row = row + d_row;
    let mut new_col = col + d_col;
    let mut new_map_row = map_row;
    let mut new_map_col = map_col;
    if new_row >= rows as i64 || new_col >= cols as i64 || new_row < 0 || new_col < 0 {
        new_map_row += d_row;
        new_map_col += d_col;
        new_row = new_row.rem_euclid(rows as i64);
        new_col = new_col.rem_euclid(cols as i64);
    }
    ((new_map_row, new_map_col), (new_row, new_col))
}

fn count_visitable(
    map: &Vec<char>,
    rows: usize,
    cols: usize,
    max_steps: usize,
) -> Option<HashMap<Pos, usize>> {
    let start_index = map.iter().position(|c| c == &'S')?;
    let start_row = start_index / cols;
    let start_col = start_index % cols;

    //Keep track of states we have seen before
    let mut visited_squares: HashSet<State> = HashSet::new();

    //List with states under consideration, accompanied by the current shortest
    //distance. Stored on a min-heap
    let mut active_states: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();
    let mut min_distances: HashMap<State, usize> = HashMap::new();

    //Insert start position and define end position
    let start_pos: Pos = (start_row as i64, start_col as i64);
    let start_state = ((0, 0), start_pos);
    active_states.push(Reverse((0, start_state)));
    min_distances.insert(start_state, 0);

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
        let mut new_states: Vec<State> = Vec::new();

        new_states.push(step_forward(position, Direction::North, rows, cols));
        new_states.push(step_forward(position, Direction::East, rows, cols));
        new_states.push(step_forward(position, Direction::South, rows, cols));
        new_states.push(step_forward(position, Direction::West, rows, cols));

        //Add the new, unseen states to the list with active states
        for state in new_states {
            let (row, col) = state.1;
            let new_distance = distance + 1;
            let new_index = (row * cols as i64 + col) as usize;
            if map[new_index] == '#' {
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

    //Count the visitable squares, group per garden tile
    let mut visitable_per_submap: HashMap<Pos, usize> = HashMap::new();
    for ((submap, _), distance) in min_distances.iter() {
        if distance % 2 == max_steps % 2 {
            visitable_per_submap
                .entry(*submap)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    Some(visitable_per_submap)
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, max_steps: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Read the map
    let rows = contents.lines().into_iter().count();
    let cols = contents.lines().next()?.len();
    let map: Vec<char> = contents.replace("\n", "").chars().collect();

    if part == 1 {
        Some(count_visitable(&map, rows, cols, max_steps)?.values().sum())
    } else {
        //Analysis of the test data and visited squares per garden revealed that
        //the visited square grows like a diamond (the actual input even more so
        //than the test input, where the corners might deviate slightly). There
        //are a few distinct gardens:
        //    /T\       T,L,R,B: top, left, right bottom.    4 x
        //   /<O>\      /\[] : "outer edges"                 N x
        //   LOEOR      <>{} : "inner edges"                 (N-1) x
        //   [{O}]      E/O  : Even and Odd inner blocks     (N-1)^2 x and N^2 x
        //    [B]

        //Get the tiles for the 2*rows + steps % rows, which is the first time we
        //see all of the above tiles
        let n = max_steps / rows;
        let rem = max_steps % rows;
        let mut map = count_visitable(&map, rows, cols, 2 * rows + rem)?;
        //Fill in the empty tiles
        for map_row in -2..3 {
            for map_col in -2..3 {
                map.entry((map_row, map_col)).or_insert(0);
            }
        }

        //E and O tiles
        let core = (n - 1) * (n - 1) * map.get(&(0, 0))? + n * n * map.get(&(1, 0))?;
        // T,L,B,R tiles
        let corners =
            map.get(&(2, 0))? + map.get(&(-2, 0))? + map.get(&(0, 2))? + map.get(&(0, -2))?;
        // <,>,{,} tiles
        let edges_o = (n - 1)
            * (map.get(&(1, 1))? + map.get(&(-1, 1))? + map.get(&(1, -1))? + map.get(&(-1, -1))?);
        // /,\,[,] tiles
        let edges_i = n
            * (map.get(&(1, 2))? + map.get(&(1, -2))? + map.get(&(-1, 2))? + map.get(&(-1, -2))?);

        Some(core + corners + edges_i + edges_o)
    }
}

fn main() -> std::io::Result<()> {
    //Calculate the answers to part 1 and 2
    println!(
        "Key part 1: {}",
        calculate_key(1, 64, "src/input.txt").unwrap()
    );
    println!(
        "Key part 2: {}",
        calculate_key(2, 26501365, "src/input.txt").unwrap()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(calculate_key(1, 6, "src/test.txt").unwrap(), 16);
    }

    #[test]
    fn test_pt2_1() {
        assert_eq!(calculate_key(1, 10, "src/test.txt").unwrap(), 50);
    }

    #[test]
    fn test_pt2_2() {
        assert_eq!(calculate_key(1, 50, "src/test.txt").unwrap(), 1594);
    }

    #[test]
    fn test_pt2_3() {
        assert_eq!(calculate_key(1, 100, "src/test.txt").unwrap(), 6536);
    }

    #[test]
    fn test_pt2_4() {
        assert_eq!(calculate_key(1, 500, "src/test.txt").unwrap(), 167004);
    }

    #[test]
    fn test_pt2_5() {
        assert_eq!(calculate_key(1, 1000, "src/test.txt").unwrap(), 668697);
    }

    #[test]
    fn test_pt2_6() {
        assert_eq!(calculate_key(1, 5000, "src/test.txt").unwrap(), 16733044);
    }
}
