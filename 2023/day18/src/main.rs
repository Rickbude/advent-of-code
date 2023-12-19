use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//Map input character to direction
fn map_direction(s: &str) -> Option<Direction> {
    match s {
        "U" | "3" => Some(Direction::Up),
        "D" | "1" => Some(Direction::Down),
        "L" | "2" => Some(Direction::Left),
        "R" | "0" => Some(Direction::Right),
        _ => None,
    }
}

//Get row/col offset for each direction
fn get_delta(direction: Direction) -> (i64, i64) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Right => (0, 1),
        Direction::Left => (0, -1),
    }
}

//Position on the map: (x,y)
type Pos = (i64, i64);
//Instruction (# of steps in which direction)
type Instruction = (Direction, i64);

//In part 1, read only direction and steps, ignore color code
fn process_line_pt1(line: &str) -> Option<Instruction> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    let direction = map_direction(parts[0])?;
    let steps = parts[1].parse::<i64>().ok()?;
    Some((direction, steps))
}

//In part two: the sixth digit of the color code encodes
//direction, and the first five hex digits the number of steps.
fn process_line_pt2(line: &str) -> Option<Instruction> {
    let color_code = line.trim().split_whitespace().last()?;
    let steps = i64::from_str_radix(&color_code[2..7], 16).ok()?;
    let direction = map_direction(&color_code[7..8])?;
    Some((direction, steps))
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Select function for processing line -> instruction
    let line_processor = if part == 1 {
        process_line_pt1
    } else {
        process_line_pt2
    };

    //Parse input file into vector of instructions
    let instructions: Vec<Instruction> = contents.lines().filter_map(line_processor).collect();

    //Get the corner points of the map, and the total length of the path
    let mut pos: Pos = (0, 0);
    let mut corners: Vec<Pos> = Vec::new();
    let mut path_len = 0;
    for (direction, steps) in instructions {
        let delta = get_delta(direction);
        corners.push(pos);
        pos.0 += delta.0 * steps;
        pos.1 += delta.1 * steps;
        path_len += steps;
    }

    //Use the "Shoelace method" to calculate the polygon area defined by the set
    //of corner points: https://en.wikipedia.org/wiki/Shoelace_formula:
    //              | x1 x2 |   | x2 x2 |
    //    A = 1/2 * | y1 y2 | + | y2 y3 | + ...
    //Note that this assumes a line of 0 width
    let inner_area = corners
        .windows(2)
        .map(|p| p[0].0 * p[1].1 - p[0].1 * p[1].0)
        .sum::<i64>()
        .abs()
        / 2;

    //Account for the finite line width: each straight step along the trail adds
    //0.5 squares extra, so add line_length/2. Furthermore, each corner adds an
    //additional +/- 0.25 squares, depending on whether it curves inwards or
    //outwards. Since we form a closed loop, there are exactly 4 more outer
    //corners than inner corners, so add 1 (4*0.25)
    let area = (inner_area + path_len / 2 + 1) as usize;
    Some(area)
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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 62);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 952408144115);
    }
}
