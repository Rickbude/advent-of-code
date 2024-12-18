use std::fs::File;
use std::io::{self, Read};

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str, search_word: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Read the map. Maybe use the 2DArray or ndarray crate?
    let rows = contents.lines().into_iter().count();
    let map: Vec<char> = contents.replace("\n", "").chars().collect();
    let cols = map.len() / rows;

    //Search directions: only diagonals in part 2, all 8 directions in part 1
    let mut directions: Vec<(i64, i64)> = Vec::new();
    directions.push((1, 1)); // Down-Right
    directions.push((1, -1)); // Down-Left
    directions.push((-1, -1)); // Up-Left
    directions.push((-1, 1)); // Up-Right
    if part == 1 {
        directions.push((0, 1)); // Right
        directions.push((1, 0)); // Down
        directions.push((0, -1)); // Left
        directions.push((-1, 0)); // Up
    }

    //'pivot character' of the word: this is what in the end determines the
    //position of a word.
    let pivot = 'A';

    //Record where we found the matches for part 2
    let mut match_pos: Vec<usize> = Vec::new();

    //Hmm maybe this can be done more elegantly
    for row in 0..rows {
        for col in 0..cols {
            for &(d_row, d_col) in &directions {
                //Maybe move this inner loop to a separate function
                let mut found_it = true;
                let mut index_of_pivot = 0;
                for k in 0..search_word.len() {
                    let x = row as i64 + k as i64 * d_row;
                    let y = col as i64 + k as i64 * d_col;
                    //Don't go out of bounds..
                    if x < 0 || y < 0 || x >= rows as i64 || y >= cols as i64 {
                        found_it = false;
                        break;
                    }
                    let index = x as usize * rows + y as usize;
                    //Not the right letter..
                    if map[index] != search_word.chars().nth(k).unwrap() {
                        found_it = false;
                        break;
                    }
                    //Store the index where we found the "A"
                    if map[index] == pivot {
                        index_of_pivot = index;
                    }
                }
                //Hurray, we found the full word!!!
                if found_it {
                    match_pos.push(index_of_pivot);
                }
            }
        }
    }

    let key: usize;
    if part == 1 {
        key = match_pos.len();
    } else {
        match_pos.sort();
        key = match_pos.windows(2).filter(|w| w[0] == w[1]).count();
    }

    Ok(key)
}

fn main() -> std::io::Result<()> {
    //Calculate the answers to part 1 and 2
    println!("Key part 1: {}", calculate_key(1, "src/input.txt", "XMAS")?);
    println!("Key part 2: {}", calculate_key(2, "src/input.txt", "MAS")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt1() {
        assert_eq!(calculate_key(1, "src/test.txt", "XMAS").unwrap(), 18);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt", "MAS").unwrap(), 9);
    }
}
