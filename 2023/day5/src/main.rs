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

//Forward and backward mapping
trait DoMap{
    fn map_fwd(&self,id: i64) -> i64;
    fn map_bwd(&self,id: i64) -> i64;
}
impl DoMap for SeedMap{
    fn map_fwd(&self,id: i64) -> i64{
        for mapping in &self.mappings{
            //Id does not fall in this mapped range
            if id < mapping.start_src || id > mapping.start_src+mapping.range{
                continue;
            }
            return id - mapping.start_src + mapping.start_dest;
        }
        id
    }
    fn map_bwd(&self,id: i64) -> i64{
        for mapping in &self.mappings{
            //Id does not fall in this mapped range
            if id < mapping.start_dest || id > mapping.start_dest+mapping.range{
                continue;
            }
            return id - mapping.start_dest + mapping.start_src;
        }
        id
    }
}

fn find_closest_location(seeds: Vec<i64>, maps: &Vec<SeedMap>) -> i64{
    let mut locations : Vec<i64>  = Vec::new();
    for mut seed_id in seeds{
        for map in maps.iter(){
            seed_id = map.map_fwd(seed_id);
        }
        locations.push(seed_id);
    }
    *locations.iter().min().unwrap()
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
    let mut seeds : Vec<i64> = seeds_string.split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect();
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
    println!("Key part 1: {}", find_closest_location(seeds.clone(),&maps));

    //For part 2, make a collection of "range starting points", by backtracking
    //each range transition towards the seed ids.
    let mut starting_points : Vec<i64> = Vec::new();
    starting_points.push(0);
    for map in maps.iter().rev(){
        //Collect all the range boundaries for this map
        for mapping in &map.mappings{
            starting_points.push(mapping.start_dest);
            starting_points.push(mapping.start_dest+mapping.range+1);
        }
        //Back-map the range boundaries to one layer up
        for i in 0 .. starting_points.len(){
            starting_points[i] = map.map_bwd(starting_points[i]);
        }
    }
    starting_points.sort();

    //Collect the potentially interesting seed numbers (seed ids at range
    //boundaries)
    seeds.clear();
    for seed_range in seed_ranges{
        seeds.push(seed_range[0]);
        for starting_point in &starting_points{
            if starting_point > &seed_range[0] && *starting_point < &seed_range[0] + &seed_range[1]{
                seeds.push(*starting_point);
            }
        }
    }

    //Find the seed id corresponding to the closest mapped location
    println!("Key part 2: {}", find_closest_location(seeds,&maps));
    Ok(())
}
