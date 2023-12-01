use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

//Get the key from a single line, which is a concatenation of the first and last
//digit. E.g. a1b2c3d4e5f -> 15
fn get_subkey(part: &str) -> i32 {
    let mut iter = part.chars().filter_map(|c| c.to_digit(10));
    let firstint = iter.next().unwrap_or(0) as i32;
    let lastint = iter.last().unwrap_or(firstint as u32) as i32;
    firstint * 10 + lastint
}

fn main() -> std::io::Result<()> {
	//Open test file for part 1
	//let mut file = File::open("src/test_pt1.txt")?;
	//Open test file for part 2
	//let mut file = File::open("src/test_pt2.txt")?;
	//Open actual input file
	let mut file = File::open("src/input.txt")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	let parts = contents.split_whitespace();

	//Part 1
	let mut key_pt1 : i32 = 0;
	for part in parts.clone() {
		key_pt1 += get_subkey(part);
	}
	println!("Key part 1:{}",key_pt1);

	//"Translation table" from string digit to string containing the digit.
	//Keeps overlap valid after replace, e.g.
	// threeight -> thr3eei8ght -> 38
	let string_to_digit = HashMap::from([
		("one","o1e"),
		("two","t2o"),
		("three","t3e"),
		("four","f4r"),
		("five","f5e"),
		("six","s6x"),
		("seven","s7n"),
		("eight","e8t"),
		("nine","n9e")
	]);

	//Part 2
	let mut key_pt2 : i32 = 0;
	for part in parts.clone() {
		//Replace the first "string digits" with actual digits, while keeping
		//overlap between digits valid
		let mut mod_part : String = part.to_string();
		for (digit_str_key, digit_str_val) in &string_to_digit{
			mod_part = mod_part.replace(digit_str_key,digit_str_val);
		}
		key_pt2 += get_subkey(&mod_part);
	}
	println!("Key part 2:{}",key_pt2);
	Ok(())
}
