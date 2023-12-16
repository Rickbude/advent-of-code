use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Ray = (i64, i64, Direction);

//Count the number of energized squares on the map (with dimensions rows x
//cols), given a certain starting position.
fn get_energized(map: &Vec<char>, rows: i64, cols: i64, start_ray: Ray) -> usize {
    //Keep track of the energized tiles
    let mut energized = map.clone();
    //Keep track of previously seen rays
    let mut states: HashSet<Ray> = HashSet::new();
    //List of active rays
    let mut rays: Vec<Ray> = Vec::new();
    rays.push(start_ray);

    loop {
        let mut rays_copy: Vec<Ray> = Vec::new();
        for (row, col, direction) in rays {
            //Ray went out of bounds
            if row < 0 || col < 0 || row >= rows || col >= cols {
                continue;
            }
            //From here on only valid indices are present. We can safely convert to usize
            let index = (row * cols + col) as usize;
            energized[index] = '#';

            //Reflect or split the light beam, depending on the mirror
            //orientation and light beam propagation direction.
            let c = map[index];
            match (direction, c) {
                //Mirrors bending beams 90 degrees
                (Direction::North, '/') => rays_copy.push((row, col + 1, Direction::East)),
                (Direction::East, '/') => rays_copy.push((row - 1, col, Direction::North)),
                (Direction::South, '/') => rays_copy.push((row, col - 1, Direction::West)),
                (Direction::West, '/') => rays_copy.push((row + 1, col, Direction::South)),
                (Direction::North, '\\') => rays_copy.push((row, col - 1, Direction::West)),
                (Direction::East, '\\') => rays_copy.push((row + 1, col, Direction::South)),
                (Direction::South, '\\') => rays_copy.push((row, col + 1, Direction::East)),
                (Direction::West, '\\') => rays_copy.push((row - 1, col, Direction::North)),
                //Beam splitters, only has an effect for perpendicular incidence
                (Direction::South | Direction::North, '-') => {
                    rays_copy.push((row, col - 1, Direction::West));
                    rays_copy.push((row, col + 1, Direction::East));
                }
                (Direction::East | Direction::West, '|') => {
                    rays_copy.push((row - 1, col, Direction::North));
                    rays_copy.push((row + 1, col, Direction::South));
                }
                //All other cases: beam travels ahead untouched
                (Direction::North, _) => rays_copy.push((row - 1, col, direction)),
                (Direction::East, _) => rays_copy.push((row, col + 1, direction)),
                (Direction::South, _) => rays_copy.push((row + 1, col, direction)),
                (Direction::West, _) => rays_copy.push((row, col - 1, direction)),
            }
        }

        //Only consider never-seen-before rays in the next iteration
        rays = rays_copy
            .into_iter()
            .filter(|ray| !states.contains(ray))
            .collect();

        //Update the rays we have encountered thus far
        rays.iter().for_each(|ray| {
            states.insert(*ray);
        });

        //We are done if all the rays have died out
        if rays.len() == 0 {
            break;
        }
    }
    //Count the number of energized squares
    energized.iter().filter(|c| c == &&'#').count()
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Create map
    let rows = contents.lines().into_iter().count() as i64;
    let cols = contents.lines().next()?.len() as i64;
    let map: Vec<char> = contents.replace("\n", "").chars().collect();

    if part == 1 {
        //Part 1: only a single starting position is interesting
        let num_energized = get_energized(&map, rows, cols, (0, 0, Direction::East));
        return Some(num_energized);
    } else {
        //part 2: consider all possible starting positions
        let mut start_rays: Vec<Ray> = Vec::new();
        (0..rows).for_each(|row| start_rays.push((row, 0, Direction::East)));
        (0..rows).for_each(|row| start_rays.push((row, cols - 1, Direction::West)));
        (0..cols).for_each(|col| start_rays.push((0, col, Direction::South)));
        (0..cols).for_each(|col| start_rays.push((rows - 1, col, Direction::North)));

        //Find the maximum possible number of energized tiles
        let max_energized = start_rays
            .iter()
            .map(|ray| get_energized(&map, rows, cols, *ray))
            .max()?;
        return Some(max_energized);
    }
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 46);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 51);
    }
}
