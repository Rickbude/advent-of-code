use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

//Hash algorithm for part 1
fn do_hash(input: &str) -> usize {
    let mut hash = 0;
    for c in input.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> Option<usize> {
    //Open input file
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut key: usize = 0;
    if part == 1 {
        key = contents.split(",").map(|x| do_hash(x)).sum();
    } else {
        //Each box contains a list of labels
        let mut boxes: Vec<Vec<String>> = Vec::new();
        boxes.resize(256, Vec::new());

        //Focal lengths are stored separately in a hashmap of label => focal length
        let mut focal_lengths: HashMap<String, usize> = HashMap::new();

        //Instructions either follow the pattern:
        //  label=len
        //  label-
        let re = Regex::new(r"^([\w\d]+)([=-])(\d+)?$").unwrap();
        for instruction in contents.split(",") {
            //Get label and operation type from the instruction
            let caps = re.captures(instruction)?;
            let label = &caps[1];

            //Hash the label, see in which box the lens belongs. Determine the slot the lens is in
            let box_id = do_hash(label);
            let lens_index = boxes[box_id].iter().position(|x| x == &label);
            if &caps[2] == "-" {
                //Remove the lens from the box, if present
                if lens_index.is_some() {
                    boxes[do_hash(label)].remove(lens_index?);
                }
            } else {
                let focal_len = &caps[3].parse::<usize>().unwrap();
                //if there is a lens in the box (indicated by the hash) with this label, replace it.
                //this can be done by changing the focal length of that hash in the hash map.
                focal_lengths
                    .entry(label.to_string())
                    .and_modify(|x| *x = *focal_len)
                    .or_insert(*focal_len);

                //if there is no lens in the box at all, add it
                if lens_index.is_none() {
                    boxes[box_id].push(label.to_string());
                }
            }
        }

        //Calculate the key
        for (box_id, lenses) in boxes.iter().enumerate() {
            for (lens_slot, lens) in lenses.iter().enumerate() {
                let focal_length = focal_lengths.get_key_value(lens)?.1;
                key += (box_id + 1) * (lens_slot + 1) * focal_length;
            }
        }
    }
    Some(key)
}

fn main() -> std::io::Result<()> {
    //Calculate the answers to part 1 and 2
    println!("Key part 1: {}", calculate_key(1, "src/input.txt").unwrap());
    println!("Key part 2: {}", calculate_key(2, "src/input.txt").unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        assert_eq!(do_hash("HASH"), 52);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(calculate_key(1, "src/test.txt").unwrap(), 1320);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(calculate_key(2, "src/test.txt").unwrap(), 145);
    }
}
