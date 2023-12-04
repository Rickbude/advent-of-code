use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

//Count the numbers 
fn count_matches(card: &str) -> u32{
    //Split card at the pipe: left contains the winning numbers, while right
    //contains your numbers. Split numbers on whitespace
    let mut iter = card.split(" | ");
    let winning_numbers_strings = iter.next().unwrap().split_whitespace();
    let your_numbers_strings = iter.next().unwrap().split_whitespace();

    //Parse the strings as unsigned numbers, put them in hash sets
    let parsing_lambda = |num: &str| num.parse::<u32>().unwrap();
    let winning_numbers: HashSet<u32> = winning_numbers_strings.map(parsing_lambda).collect();
    let your_numbers: HashSet<u32> = your_numbers_strings.map(parsing_lambda).collect();

    //Intersection between the sets gives the number of matches
    winning_numbers.intersection(&your_numbers).count() as u32
}

fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines();

    let mut key_pt1: u32 = 0;

    let mut scratchcards: Vec<u32> = Vec::new();
    scratchcards.resize(lines.clone().count(),1);

    for (card_id,line) in lines.enumerate() {
        //Actual information (aside from trivial card id) starts after the colon
        let card = line.split(":").last().unwrap();
        let matches = count_matches(card);
        if matches>0 {
            //Add card of score for part 1
            key_pt1 += u32::pow(2,matches-1);
            //For part 2, number of matches = number of copies received
            for i in 0 .. matches as usize{
                scratchcards[card_id+i+1] += scratchcards[card_id];
            }
        }
    }
    let key_pt2: u32 = scratchcards.iter().sum();
    
    //Print solution
    println!("Key part 1: {}", key_pt1);
    println!("Key part 2: {}", key_pt2);
    Ok(())
}