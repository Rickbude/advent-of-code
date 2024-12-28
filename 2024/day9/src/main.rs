use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, Read};

//Calculate the answer to part 1 / 2
fn calculate_key(part: usize, filename: &str) -> io::Result<usize> {
    //Open input file
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Determine the "actual layout" from the disk map
    let mut files: Vec<(usize, usize, usize)> = Vec::new();
    let mut spaces: Vec<BinaryHeap<Reverse<usize>>> = Vec::new();
    spaces.resize(10, BinaryHeap::new());

    let mut cnt = 0;
    for (i, c) in contents.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            //File, insert as (position,ID,size) pair
            files.push((cnt, i / 2, size));
        } else {
            //Free space, store location and size (implicit in array index)
            spaces[size].push(Reverse(cnt));
        }
        cnt += size;
    }

    //Build the final list of file locations. This is technically not needed: we
    //could immediately calculate the key at the positions where we push to this
    //array.
    let mut final_files: Vec<(usize, usize, usize)> = Vec::new();
    while let Some((file_position, file_id, file_size)) = files.pop() {
        //Search a place where we can leave our file. The available space must
        //lay further to the left than the file itself. Search for the minimum
        //position among all heaps with minimal size:
        // - In part 1, any space will do, files will fragment
        // - In part 2, only consider spaces larger then the file size.
        let min_size = if part == 1 { 1 } else { file_size };
        let best_space = spaces
            .iter()
            .enumerate()
            .filter_map(|(size, heap)| {
                if !heap.is_empty() {
                    Some((size, heap.peek().unwrap()))
                } else {
                    None
                }
            })
            .filter(|(size, &Reverse(position))| size >= &min_size && position < file_position)
            .max_by(|a, b| a.1.cmp(b.1));

        //Apparently there is no space left that satisfies our criteria, so we
        //are done with this file.
        if best_space.is_none() {
            final_files.push((file_position, file_id, file_size));
            continue;
        }

        let space_size = best_space.unwrap().0;
        if let Some(Reverse(space_position)) = spaces[space_size].pop() {
            let mut moved = file_size;
            if file_size <= space_size {
                //When the available space is larger than the file, add the
                //remaining space back to pool of available spaces (the array
                //with 0-sized space will gradually grow)
                spaces[space_size - file_size].push(Reverse(space_position + file_size));
            } else {
                //File is larger or equal in size to the space: add back the
                //remainder of the file to the back of the queue. This will
                //cause the file to become fragmented.
                files.push((file_position, file_id, file_size - space_size));
                moved = space_size;
            }
            final_files.push((space_position, file_id, moved));
        }
    }

    //Calculate the checksum. This could be done with triangle numbers..
    let mut key = 0;
    for (file_position, file_id, file_size) in final_files {
        for i in 0..file_size {
            key += (file_position + i) * file_id;
        }
    }

    Ok(key)
}

//Run the script on the full input (both part 1 and part 2)
fn main() -> std::io::Result<()> {
    println!("Key part 1: {}", calculate_key(1, "src/input.txt")?);
    println!("Key part 2: {}", calculate_key(2, "src/input.txt")?);
    Ok(())
}

//Run the script on the test input (run with cargo test)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pt1_1() {
        assert_eq!(calculate_key(1, "src/test1.txt").unwrap(), 1928);
    }

    #[test]
    fn test_pt1_2() {
        assert_eq!(calculate_key(1, "src/test2.txt").unwrap(), 60);
    }

    #[test]
    fn test_pt2_1() {
        assert_eq!(calculate_key(2, "src/test1.txt").unwrap(), 2858);
    }

    #[test]
    fn test_pt2_2() {
        assert_eq!(calculate_key(2, "src/test2.txt").unwrap(), 132);
    }
}
