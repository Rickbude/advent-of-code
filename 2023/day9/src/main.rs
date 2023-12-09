use std::fs::File;
use std::io::prelude::*;

//Predict the next value (part 1), or the previous value (part 2) recursively
fn predict_value(part: usize, numbers: &Vec<i64>) -> Option<i64> {
    if numbers.iter().all(|x| x == &0) {
        //All elements are 0 -> we reached the bottom and can start working our
        //way upwards
        Some(0)
    } else {
        //Calculate the difference vector ( vec[i] - vec[i-1])
        let diff_vec: Vec<i64> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
        let predicted_value = predict_value(part, &diff_vec)?;
        if part == 1 {
            //Part 1: predict the next value in the sequence
            Some(predicted_value + numbers.iter().last()?)
        } else {
            //Part 2: predict the previous value in the sequence
            Some(numbers.iter().next()? - predicted_value)
        }
    }
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, contents: &str) -> Option<i64> {
    let lines = contents.lines();
    let mut key: i64 = 0;
    for line in lines {
        //Parse numbers to signed integers
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        //Calculate the desired value and add it to the answer
        key += predict_value(part, &numbers)?;
    }
    Some(key)
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
