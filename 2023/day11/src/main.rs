use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

//Do the "expansion of space"
// in: part : part 1 or 2
// in: max_unmapped: maximum number of input rows/columns
// in: non_empty_space: set containing only the non-empty rows or columns
// out: vector containing for each input row/column the expanded row/column
fn expand_space(part: usize, max_unmapped: usize, non_empty_space: &HashSet<usize>) -> Vec<usize> {
    let mut expanded: Vec<usize> = Vec::new();
    let mut counter = 0;
    for row_or_col in 0..max_unmapped {
        expanded.push(counter);
        if !non_empty_space.contains(&row_or_col) {
            if part == 1 {
                //Part 1: each empty row/column is doubled
                counter += 2;
            } else {
                //Part 2: each empty row/column is duplicated 1.000.000 times
                counter += 1000000;
            }
        } else {
            counter += 1;
        }
    }
    expanded
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, contents: &str) -> Option<i64> {
    let lines = contents.lines();

    //Extract the galaxies
    let mut rows = 0;
    let mut cols = 0;
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut non_empty_rows: HashSet<usize> = HashSet::new();
    let mut non_empty_cols: HashSet<usize> = HashSet::new();
    for (row, line) in lines.enumerate() {
        cols = line.len();
        for (col, symbol) in line.chars().enumerate() {
            if symbol == '#' {
                non_empty_rows.insert(row);
                non_empty_cols.insert(col);
                galaxies.push((row, col));
            }
        }
        rows += 1;
    }

    //Do expansion of space
    let expanded_rows = expand_space(part, rows, &non_empty_rows);
    let expanded_cols = expand_space(part, cols, &non_empty_cols);

    //Calculate the manhattan distance between each pair of galaxies
    let mut distance = 0;
    for galaxy_src in &galaxies {
        for galaxy_dst in &galaxies {
            distance += i64::abs(expanded_rows[galaxy_src.0] as i64 - expanded_rows[galaxy_dst.0] as i64);
            distance += i64::abs(expanded_cols[galaxy_src.1] as i64 - expanded_cols[galaxy_dst.1] as i64);
        }
    }

    //We counted each pair twice, so divide by two to get the key
    Some(distance / 2)
}

fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("Key part 1: {}", calculate_key(1, &contents).unwrap());
    println!("Key part 2: {}", calculate_key(2, &contents).unwrap());

    Ok(())
}
