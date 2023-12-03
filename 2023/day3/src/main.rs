use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

//Character is not numeric and not a dot -> its a symbol
fn is_symbol(c: char) -> bool {
    !(c == '.' || c.is_numeric())
}

//2D index (row,col) to 1D linear index
fn get_index(row: usize, col: usize, cols: usize) -> usize {
    row * cols + col
}

fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Turn 2D blueprint into 1D vector of chars, obtain number of rows and columns
    let lines: Vec<String> = contents.split_whitespace().map(str::to_string).collect();
    let rows: usize = lines.len();
    let blueprint: Vec<char> = lines.concat().chars().collect();
    let cols: usize = blueprint.len() / rows;

    let mut key_pt1: i32 = 0;
    let mut key_pt2: i32 = 0;

    //For part 2: keep track of which numbers touch each "gear" (*)
    let mut gears: HashMap<usize, Vec<i32>> = HashMap::new();

    for row in 0..rows {
        //Multi-digit number is accumulated in this vector
        let mut part_number: Vec<char> = Vec::new();

        //This number is a part number (it is touching a symbol)
        let mut is_part_number: bool = false;

        //Keep track of which gears this number is touching (the reverse of the gears hashmap)
        let mut next_to_gears: HashSet<usize> = HashSet::new();

        for col in 0..cols + 1 {
            let index = get_index(row, col, cols);
            if col < cols && blueprint[index].is_numeric() {
                //Accumulate digit in number
                let c = blueprint[index];
                part_number.push(c);

                //Look around this digit to find symbols
                let min_symbol_row = std::cmp::max(0, (row as isize) - 1) as usize;
                let max_symbol_row = std::cmp::min(rows, row + 2);
                let min_symbol_col = std::cmp::max(0, (col as isize) - 1) as usize;
                let max_symbol_col = std::cmp::min(cols, col + 2);
                for symbol_row in min_symbol_row..max_symbol_row {
                    for symbol_col in min_symbol_col..max_symbol_col {
                        let index_part = get_index(symbol_row, symbol_col, cols);
                        //For part 1: check if the number is next to ANY symbol
                        if is_symbol(blueprint[index_part]) {
                            is_part_number = true;
                        }
                        //For part 2: check if the number is next to a gear (*)
                        if blueprint[index_part] == '*' {
                            next_to_gears.insert(index_part);
                        }
                    }
                }
            } else {
                let s: String = part_number.iter().collect();
                if s.len() > 0 {
                    //Store the number
                    let number = s.parse::<i32>().unwrap();
                    if is_part_number {
                        key_pt1 += number;
                    }
                    //Store numbers touching the gears
                    for gearid in next_to_gears.clone() {
                        gears.entry(gearid).or_default().push(number);
                    }
                    //Prepare for the next number
                    part_number.clear();
                    next_to_gears.clear();
                    is_part_number = false;
                }
            }
        }
    }

    //Check which gears have exactly two adjacent numbers, and multiply those together
    for numbers in gears.values() {
        if numbers.len() == 2 {
            key_pt2 += numbers[0] * numbers[1];
        }
    }

    println!("Key part 1: {}", key_pt1);
    println!("Key part 2: {}", key_pt2);
    Ok(())
}
