use std::fs::File;
use std::io::prelude::*;

//Position on the map: (x,y,z)
type Pos = (i64, i64, i64);

//A block is defined by its 2 opposite corner points.
type Block = (Pos, Pos);

//Parse a coordinate string like 12,13,14 into a tuple(x,y,z)
fn parse_coordinate(coordinate_string: &str) -> Pos {
    let coordinates: Vec<i64> = coordinate_string
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    assert!(coordinates.len() == 3);
    (coordinates[0], coordinates[1], coordinates[2])
}

//Parse a coordinate pair like 12,13,14~16,17,18 into a pair of positions
fn parse_block(line: &str) -> Block {
    let parts: Vec<Pos> = line.split("~").map(parse_coordinate).collect();
    assert!(parts.len() == 2);
    assert!(parts[1].0 >= parts[0].0);
    assert!(parts[1].1 >= parts[0].1);
    assert!(parts[1].2 >= parts[0].2);
    (parts[0], parts[1])
}

//Check if two blocks collide (partial overlap)
fn blocks_collide(block_1: &Block, block_2: &Block) -> bool {
    let x_overlap = block_1.1 .0 >= block_2.0 .0 && block_1.0 .0 <= block_2.1 .0;
    let y_overlap = block_1.1 .1 >= block_2.0 .1 && block_1.0 .1 <= block_2.1 .1;
    let z_overlap = block_1.1 .2 >= block_2.0 .2 && block_1.0 .2 <= block_2.1 .2;
    x_overlap && y_overlap && z_overlap
}

fn something_can_move(blocks: &mut Vec<Block>, moving: &mut Vec<bool>, do_move: bool) -> bool {
    for i in 0..blocks.len() {
        //Block reached the floor and can not move anymore
        if blocks[i].0 .2 <= 1 {
            if do_move {
                moving[i] = false;
            }
            continue;
        }

        //Block is already non-moving
        if !moving[i] {
            continue;
        }

        //Attempt to drop the block by 1 z-value
        let mut new_block = blocks[i];
        new_block.0 .2 -= 1;
        new_block.1 .2 -= 1;

        //See if the block has collided with any other block
        let mut can_move = true;
        for j in 0..blocks.len() {
            if i == j {
                continue;
            }
            if blocks_collide(&new_block, &blocks[j]) {
                can_move = false;
                //Block collided with a stationary block
                if !moving[j] && do_move {
                    moving[i] = false;
                }
                break;
            }
        }
        //Block can move -> do it
        if can_move {
            if do_move {
                blocks[i] = new_block;
            }
            return true;
        }
    }
    false
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    //Blocks in their starting positions
    let mut blocks: Vec<Block> = contents.lines().map(parse_block).collect();

    //Sort blocks on ascending z-coordinate
    blocks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    //Keep track of which blocks have become stationary
    let mut moving: Vec<bool> = Vec::new();
    moving.resize(blocks.len(), true);

    //Let the blocks fall until there is no more movement
    loop {
        if !something_can_move(&mut blocks, &mut moving, true) {
            break;
        }
    }
    
    let mut key = 0;

    //Back-up the start state of the simulation
    let original_blocks = blocks.clone();
    for i in 0..blocks.len() {

        //Take the block away (change its position to an "illegal" position where physics are disabled)
        blocks[i].0 .2 = -1;
        blocks[i].1 .2 = -1;

        moving.clear();
        moving.resize(blocks.len(), true);
        if part == 1 {            
            //Check what would happen: is movement possible?
            key += !something_can_move(&mut blocks, &mut moving, false) as usize;
        }else{
            //Run the simulation until all blocks have settled again
            loop{
                if !something_can_move(&mut blocks, &mut moving, true){
                    break;
                }
            }
            //Determine which blocks have moved
            for j in 0..blocks.len(){
                if blocks[j] != original_blocks[j] && i != j{
                    key += 1;
                }
            }
        }

        //Reset the simulation
        blocks = original_blocks.clone();
    }

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
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 5);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 7);
    }
}
