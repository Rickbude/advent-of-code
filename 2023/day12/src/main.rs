use std::fs::File;
use std::io::prelude::*;
use std::thread::current;
use num_integer;
use std::iter;

fn check_sequence(springs: &Vec<char>, groups: &Vec<usize>) -> bool {
    let splits : Vec<_> = springs.split(|x| x==&'.').filter(|x| x.len() > 0).collect();
    //Check if we have identical number of groups
    if splits.len() != groups.len(){
        return false;
    }
    //Check if the groups are identical
    let mut counter = 0;
    for split in splits{
        if split.len()>0 {
            if split.len() != groups[counter]{
                return false;
            }
            counter += 1;
        }
    }
    true
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, contents: &str) -> Option<i64> {
    let lines = contents.lines();    
    let mut key: i64 = 0;
    let springs : Vec<bool> = Vec::new();
    for line in lines{
        //Separate the map from the groups
        let mut iter = line.split_whitespace();

        let repeats = if part == 1 { 1 } else { 5 };

        //Get the map (the left part). Optionally repeated 5 times, joined with a '?' for part 2
        let map = iter::repeat(iter.next()?).take(repeats).collect::<Vec<_>>().join("?");
        //Get the groups (rhe right part). Optionally repeated 5 times, joined with a ',' for part 2
        let group_string = iter::repeat(iter.next()?).take(repeats).collect::<Vec<_>>().join(",");
        //Parse groups into numbers
        let groups: Vec<usize> = group_string.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        //Get the indices corresponding to the question marks
        let indices = map.chars()
            .enumerate()
            .filter(|(_, c)| c == &'?')
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        
        let num_question_marks = indices.len();
        let mut sequence: Vec<char> = map.chars().collect();
        
        for num in 0..i64::pow(2,num_question_marks as u32){            
            for digit in 0..num_question_marks{
                if (num >> digit) & 1 == 1 {                    
                    sequence[indices[digit]] = '.';
                }else{
                    sequence[indices[digit]] = '#';
                }
            } 
            key += check_sequence(&sequence,&groups) as i64;       
        }
    }
    Some(key)
}

fn main() -> std::io::Result<()> {
    //Open test file
    let mut file = File::open("src/test.txt")?;
    //Open actual input file
    //let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Key part 1: {}", calculate_key(1, &contents).unwrap());
    println!("Key part 2: {}", calculate_key(2, &contents).unwrap());

    Ok(())
}
