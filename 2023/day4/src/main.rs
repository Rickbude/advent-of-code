use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;


fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines();

    let mut key_pt1: u32 = 0;

    //Lines look something like:
    //Card 1: 41 ... 17 | 83 ... 53
    //Capture card id, numbers left and numbers right of the pipe
    let re = Regex::new(r"^Card\s+(?<card_id>\d+):(?<winning_numbers>[\s\d]+)\|(?<your_numbers>[\s\d]+)$").unwrap();

    let mut scratchcards: Vec<u32> = Vec::new();
    scratchcards.resize(lines.clone().count(),1);

    for line in lines {
        let Some(caps) = re.captures(line) else{
            panic!("Invalid input!");
        };

        //Retrieve named groups from regex
        let card_id = &caps["card_id"].parse::<usize>().unwrap();
        let winning_numbers_string = &caps["winning_numbers"].trim();
        let your_numbers_string = &caps["your_numbers"].trim();

        //Turn string of numbers (split by whitespace) into array of numbers
        let winning_numbers : Vec<u64> = winning_numbers_string.split_whitespace().map(|c| c.trim().parse::<u64>().unwrap()).collect();
        let your_numbers    : Vec<u64> = your_numbers_string.split_whitespace().map(|c| c.trim().parse::<u64>().unwrap()).collect();

        //Count number of matching numbers
        let matches : u32 = your_numbers.iter().map(|c| winning_numbers.contains(c) as u32).sum();

        if matches>0 {
            //Add card of score for part 1
            key_pt1 += u32::pow(2,matches-1);
            //For part 2, number of matches = number of copies received
            for i in 0 .. matches as usize{
                scratchcards[*card_id+i] += scratchcards[*card_id-1];
            }
        }        
    }
    let mut key_pt2: u32 key_pt2 = scratchcards.iter().sum();
    
    //Print solution
    println!("Key part 1: {}", key_pt1);
    println!("Key part 2: {}", key_pt2);
    Ok(())

}