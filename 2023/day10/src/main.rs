use std::fs::File;
use std::io::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter,Copy,Clone)]
enum Direction{
    Up,Left,Down,Right
}

struct Step{
    position: (usize,usize),
    direction: Direction
}

//Traverse a pipe, causing a potential orientatino change. Only if the pipe is
//entered from an "open" side, the pipe is traversed. All other cases are deemed
//invalid.
fn traverse_pipe(orientation: &Direction, pipe: char) -> Option<Direction> {
    match (pipe,orientation) {
        ('L',Direction::Down)  => Some(Direction::Right),
        ('L',Direction::Left)  => Some(Direction::Up),
        ('J',Direction::Down)  => Some(Direction::Left),
        ('J',Direction::Right) => Some(Direction::Up),
        ('7',Direction::Right) => Some(Direction::Down),
        ('7',Direction::Up)    => Some(Direction::Left),
        ('F',Direction::Left)  => Some(Direction::Down),
        ('F',Direction::Up)    => Some(Direction::Right),
        ('|',Direction::Up)    => Some(Direction::Up),
        ('|',Direction::Down)  => Some(Direction::Down),
        ('-',Direction::Left)  => Some(Direction::Left),
        ('-',Direction::Right) => Some(Direction::Right),
        _                      => None
    }
}

//Take a step on the map, changing the position
fn take_step(position: (usize,usize), direction: &Direction, rows: usize, cols: usize) -> Option<(usize,usize)> {
    let new_position = match &direction {
        Direction::Up    => (position.0 as i64 - 1,position.1 as i64),
        Direction::Down  => (position.0 as i64 + 1,position.1 as i64),
        Direction::Left  => (position.0 as i64,position.1 as i64 - 1),
        Direction::Right => (position.0 as i64,position.1 as i64 + 1),
    };
    //Make sure we did not go out of the map
    if  new_position.0 < 0 || new_position.0 >= rows as i64 || 
        new_position.1 < 0 || new_position.1 >= cols as i64 {
        return None;
    }else{
        return Some((new_position.0 as usize,new_position.1 as usize));
    }
}

//Step through the map, record the traversed path length
fn walk_path(map:&Vec<Vec<char>>,start_position:(usize,usize),start_direction: Direction) -> Option<Vec<Step>>{
    //Get rows and columns in the map (implicitly assumes each row has the same
    //number of columns)
    let rows = map.len();
    let cols = map[0].len();

    //Do the map traversal
    let mut position = start_position;
    let mut direction = start_direction;

    let mut steps: Vec<Step> = Vec::new();
    loop {
        //Take a step forward, if possible
        position = match take_step(position,&direction,rows,cols) {
            Some(new_position) => new_position,
            None => break
        };
        let step: Step = Step{position,direction};
        steps.push(step);
        //Get the new pipe shape
        let pipe = map[position.0][position.1];
        //Traverse the pipe, if possible
        direction = match traverse_pipe(&direction,pipe) {
            Some(new_direction) => new_direction,
            None => break
        };        
    }
    Some(steps)
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, contents: &str) -> Option<usize> {
    let lines = contents.lines();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start_position = (0,0);
    for (row,line) in lines.enumerate(){
        map.push(line.chars().collect());
        //Register the start-position
        let col = line.find('S');
        if col.is_some() {
            start_position = (row,col?);
        }
    }

    //Check each starting direction, find the longest path
    for direction in Direction::iter(){
        let path = walk_path(&map,start_position,direction)?;
        //Stub path found, this is not a loop
        if path.len()<4 {
            continue;
        }
        if part == 1{
            return Some(path.len()/2);
        }else{
            //Mark our steps in a clone of the map. Start with detecting which
            //character 'S' is masquerading as, and replacing it with the actual
            //character
            let mut surface_map = map.clone();
            let real_s = match (path.last()?.direction,path[0].direction){
                (Direction::Up,Direction::Up)       => Some('|'),
                (Direction::Down,Direction::Down)   => Some('|'),
                (Direction::Left,Direction::Left)   => Some('-'),
                (Direction::Right,Direction::Right) => Some('-'),
                (Direction::Up,Direction::Right)    => Some('F'),
                (Direction::Left,Direction::Down)   => Some('F'),
                (Direction::Right,Direction::Down)  => Some('7'),
                (Direction::Up,Direction::Left)     => Some('7'),
                (Direction::Down,Direction::Left)   => Some('J'),
                (Direction::Right,Direction::Up)    => Some('J'),
                (Direction::Down,Direction::Right)  => Some('L'),
                (Direction::Left,Direction::Up)     => Some('L'),
                _ => None
            };
            surface_map[start_position.0][start_position.1] = real_s?;
            
            //Now mark the path in the map, but only for pipes that cross vertically 
            //between 0 and 0.5 block height. E.g. J and 7 can be seen as:
            //     [    #   ]           [         ]
            // J = [#####   ]     7  =  [#####    ]
            //     [        ]           [    #    ]  <--- must cross
            //Of these two, only 7 cross vertically in the lower half. Similar for F vs L
            //Mark L and J with a different marker
            let magic_chars = vec!['|','7','F'];
            for step in path.iter(){
                if magic_chars.contains(&surface_map[step.position.0][step.position.1]){
                    surface_map[step.position.0][step.position.1] = '#';
                }else{
                    surface_map[step.position.0][step.position.1] = '*';
                }
            }

            //Use the point-in-polygon algorithm to determine whether a point is
            //inside, or outside the outlined route. An odd number of crossings
            //indicates that the point is inside.
            let mut inside = 0;
            for row in surface_map {
                let mut crossings = 0;
                for mark in row{
                    if mark == '#'{
                        crossings += 1;
                    }else{                        
                        if crossings%2 == 1 && mark != '*'{
                            inside+=1;
                        }
                    }
                }
            }
            return Some(inside)
        }
    }
    Some(0)
}

fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test_3.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Key part 1: {}", calculate_key(1, &contents).unwrap());
    println!("Key part 2: {}", calculate_key(2, &contents).unwrap());

    Ok(())
}