use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, EnumString)]
enum Color{
    red,green,blue
}

fn main() -> std::io::Result<()> {
	//Open test input
    //let mut file = File::open("src/test.txt")?;
	//Open actual input file
	let mut file = File::open("src/input.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let lines = contents.lines();

    let mut key_pt1 :i32 = 0;
    let mut key_pt2: i32 = 0;

    //A game is valid if the number of red/green/blue marbles is smaller than 12/13/14
    let max_valid_values: [i32;3] = [12,13,14];

	//Part 1 and part 2
    //TODO: there must be a nicer way to handle all this parsing
    let mut game_id = 1;
	for line in lines.clone() {
		//Extract the part of the string actually containing the information
        let game_string = line.get(line.find(":").unwrap()+1..).unwrap();

        let mut game_valid: bool = true;
        let mut minvalues: [i32;3] = [0,0,0];

        //Split off the individual "revelations"
        let marble_groups = game_string.split(";");
        for marble_group in marble_groups{
            let marbles = marble_group.split(",");
            for marble in marbles{
                //Remove leading whitespace
                let marble_substr = marble.trim();

                //Split string (something like: "17 red") at the space, and
                //extract color and number seperately
                let separator: usize= marble_substr.find(" ").unwrap();
                let number_str = marble_substr.get(0..separator).unwrap();
                let number = number_str.parse::<i32>().unwrap();
                let color_str= marble_substr.get(separator+1..).unwrap();

                //Translate color string to enum
                let color = Color::from_str(color_str).unwrap();
                let color_id = color as usize;

                //For part 1: check if the number of marbles revealed is less
                //than the threshold
                if number > max_valid_values[color_id]{
                    game_valid = false;
                }
                //For game two: update the minimum amount of marbles of that
                //color in the game
                minvalues[color_id] = std::cmp::max(number,minvalues[color_id]);
            }
        }
        //Update keys for part 1 and part 2
        if game_valid{
            key_pt1 += game_id;
        }
        let power: i32 = minvalues.iter().product();
        key_pt2 += power;
        game_id+=1;
	}
    println!("Key for part 1: {}",key_pt1);
    println!("Key for part 2: {}",key_pt2);
	Ok(())
}