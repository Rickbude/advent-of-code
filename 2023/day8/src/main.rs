use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use num_integer;

//Calculate the answer to part 1 / 2
fn calculate_key(part : usize, contents: &str) -> Option<usize>{
    //Split input at double newline
    let mut blocks = contents.split("\n\n");
    let directions = blocks.next()?;
    let nodes = blocks.next()?.lines();

    //Extract the nodes, put them in a hashmap with the name of the node as key,
    //and a pair of nodenames for the left and right nodes.
    let mut node_map : HashMap<&str,(&str,&str)> = HashMap::new();
    for node in nodes{
        let node_name  = &node[0..3];
        let left_node  = &node[7..10];
        let right_node = &node[12..15];
        node_map.insert(node_name,(left_node,right_node));
    }

    //Set start nodes
    let mut current_nodes: Vec<&str> = Vec::new();
    if part == 1 {
        //Part one starts at AAA
        current_nodes.push("AAA");
    }else{
        //Part two starts at any node ending on A
        for key in node_map.keys(){
            if key.chars().last() == Some('A'){
                current_nodes.push(key);
            }
        }
    }

    //For each of the start nodes, find the number of steps needed to reach the end node
    let mut counters: Vec<usize> = Vec::new();
    for mut current_node in current_nodes.iter().copied(){
        let mut counter = 0;
        loop {
            //Turn left or right based on the next instruction
            let direction = directions.chars().nth(counter % directions.len())?;
            if direction == 'L'{
                current_node = node_map.get(current_node)?.0;
            } else {
                current_node = node_map.get(current_node)?.1;
            }
            counter += 1;
            //Termination condition for part 1 is that the final node is ZZZ
            //Termination condition for part 2 is that the final node ends in Z
            if  (part == 1 && current_node == "ZZZ")  ||
                (part == 2 && current_node.chars().last() == Some('Z'))
            {
                counters.push(counter);
                break;
            }
        }
    }
    //Conveniently, the paths are cyclic if you proceed beyond the end. E.g. in
    // my input: AAA -> (LBL, QGQ) -> .... -> ZZZ -> (QGQ, LBL)
    //Furthermore, the length of each sequence is an exact multiple of the number of directions:
    // counter % directions.len() = 0
    //Then we can quickly calculate when all nodes simultaneously end in Z by
    //calculating their least common multiple (lcm).
    //Were this not the case, a more difficult calculation would be needed here
    let mut key_pt = counters[0];
    for counter in counters{
        key_pt = num_integer::lcm(key_pt,counter);
    }
    Some(key_pt)
}


fn main() -> std::io::Result<()> {
    //Open test files
    //let mut file = File::open("src/test_1.txt")?;
    //let mut file = File::open("src/test_2.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Key part 1: {}", calculate_key(1,&contents).unwrap());
    println!("Key part 2: {}", calculate_key(2,&contents).unwrap());

    Ok(())
}