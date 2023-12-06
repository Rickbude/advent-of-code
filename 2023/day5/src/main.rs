use std::fs::File;
use std::io::prelude::*;

//A single map, from one source property to one destination property.
struct Mapping{
    start_src: i64,
    start_dest: i64,
    range: i64
}

//Complete map, from seed id to location
struct SeedMap {
    name: String,
    mappings: Vec<Mapping>
}

//Map forward, from source to destination. For a smart part 2, a backward
//mapping is probably also needed.
trait DoMap{
    fn do_map(&self,id: i64) -> i64;
}
impl DoMap for SeedMap{
    fn do_map(&self,id: i64) -> i64{
        for mapping in &self.mappings{
            //Id does not fall in this mapped range
            if id < mapping.start_src || id > mapping.start_src+mapping.range{
                continue;
            }
            return id - mapping.start_src + mapping.start_dest;
        }
        id
    }
}

fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Split input at double newline
    let mut blocks = contents.split("\n\n");

    //Get the input array of seeds
    let seeds_string = blocks.next().unwrap().split(":").last().unwrap();
    let seeds : Vec<i64> = seeds_string.split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect();
    let seeds_clone = seeds.clone();
    let seed_ranges: Vec<_>  = seeds_clone.chunks(2).collect();

    //Process the various maps
    let mut maps : Vec<SeedMap> = Vec::new();
    for block in blocks{

        let mut iter = block.split(":");
        let block_name = iter.next().unwrap();
        let block_content = iter.next().unwrap().trim();

        //Create a new map
        let mut seedmap = SeedMap{
            name: block_name.to_string(),
            mappings: Vec::new()
        };

        //Add the mapping rules one-by-one
        for line in block_content.lines(){
            let numbers : Vec<i64> = line.split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect();
            let mapping = Mapping{
                start_dest: numbers[0],
                start_src: numbers[1],
                range: numbers[2]
            };
            seedmap.mappings.push(mapping);
        }
        maps.push(seedmap);
    }

    //Find the seed id corresponding to the closest mapped location
    let mut locations_pt1 : Vec<i64>  = Vec::new();
    for mut seed_id in seeds{
        for map in maps.iter(){
            seed_id = map.do_map(seed_id);
        }
        locations_pt1.push(seed_id);
    }
    println!("Key part 1: {}", locations_pt1.iter().min().unwrap());

    //Part 2 can be bruteforced, but it will long (hours, I didn't time it)
    //This will be replaced with a smart solution later
    println!("Brute-forcing key for part 2. Strap in, this will take a while..");
    let mut min_location = i64::MAX;
    for seed_range in seed_ranges{
        println!("Processing range: {} - {} (length: {})",seed_range[0],seed_range[0]+seed_range[1],seed_range[1]);
        for mut seed_id in seed_range[0] .. seed_range[0]+seed_range[1]{
            for map in maps.iter(){
                seed_id = map.do_map(seed_id);
            }
            min_location = std::cmp::min(min_location,seed_id);
        }
    }
    println!("Key part 2: {}",min_location);
    Ok(())
}
