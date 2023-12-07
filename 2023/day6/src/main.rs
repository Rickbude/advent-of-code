use std::fs::File;
use std::io::prelude::*;

//Calculate the answer to part 1 / 2
fn calculate_key(part : usize, contents: &str) -> Option<usize>{
    let mut lines = contents.lines();
    
    //Split times/distance strings on whitespace 
    let times_split = lines.next()?.split(":").last()?.split_whitespace();
    let dists_split = lines.next()?.split(":").last()?.split_whitespace();

    //Load times/distances to a vector
    let mut times : Vec<usize> = Vec::new();
    let mut dists : Vec<usize> = Vec::new();    
    if part == 1 {
        //For part 1: parse each number string as a digit
        times = times_split.map(|x| x.parse::<usize>().unwrap()).collect();
        dists = dists_split.map(|x| x.parse::<usize>().unwrap()).collect();
    }else{
        //For part 2: concatenate the number strings to a single number and
        //parse it
        times.push(times_split.collect::<Vec<_>>().join("").parse::<usize>().unwrap());
        dists.push(dists_split.collect::<Vec<_>>().join("").parse::<usize>().unwrap());
    }

    assert_eq!(times.len(),dists.len());

    //Find zeroes in equation t_hold*(t_race-t_hold-1)=record_dist, or:
    // t_hold^2 - t_race*t_hold + record_dist = 0
    // Solve with the abc formula, with 
    //  a = 1, 
    //  b = -t_race, 
    //  c = record_dist
    let mut margin = 1;
    for (n,dist) in dists.iter().enumerate(){
        let a : f64 = 1.0;
        let b : f64 = -(times[n] as f64);
        let c : f64 = *dist as f64;
        let determinant = b*b - 4.0*a*c; 
        //Add 1e-6 to make sure we round the right direction
        let t_hold_min = ((-b-f64::sqrt(determinant))/(2.0*a) + 1e-6).ceil()  as usize;
        let t_hold_max = ((-b+f64::sqrt(determinant))/(2.0*a) - 1e-6).floor() as usize;
        //let t_hold_max = std::cmp::min(t_hold_max,times[n]);
        let range = t_hold_max - t_hold_min + 1;
        margin *= range;
    }
    Some(margin)
}


fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    println!("Key part 1: {}", calculate_key(1,&contents).unwrap());
    println!("Key part 2: {}", calculate_key(2,&contents).unwrap());

    Ok(())
}