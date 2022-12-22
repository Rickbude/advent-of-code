#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>
#include <regex>
#include <unordered_map>
#include <cmath>
#include <set>

//The map is a matrix with dimensions Nx7
//Matrix dimensions grow as needed.
const int N_cols   = 7;
using pos_type = std::pair<long long,long long>;
struct Map{
    std::vector<std::array<bool,N_cols>> blocks;
};

//General structure that holds a shape
struct Shape{
    int rows = 0;
    int cols = 0;
    std::vector<bool> blocks;
};

//Detect of this block currently collides with the map
//Pos is the left lower corner of the shape!
bool detect_collision(const Map& map, const Shape& shape, const pos_type& pos){
    //Shape collided with bottom of the map
    if(pos.second < 0){
        return true;
    }
    //Shape collided with the left edge of the map
    if(pos.first < 0){
        return true;
    }
    //Shape collided with the right edge of the map
    if(pos.first > N_cols - shape.cols){
        return true;
    }
    //Check if shape collided with another piece in the map
    for(int i = 0; i<shape.blocks.size(); i++){
        if(!shape.blocks[i]){
            continue;
        } 
        int row = pos.second + (i / shape.cols);
        int col = pos.first  + (i % shape.cols);                 
        if(map.blocks[row][col]){
            return true;
        }
    }
    return false;
}

//Write a shape to the matrix once it reached its final destination
void write_shape(Map& map, const Shape& shape, const pos_type& pos){
    for(int i = 0; i<shape.blocks.size(); i++){
        if(!shape.blocks[i]){
            continue;
        }
        int row = pos.second + (i / shape.cols);
        int col = pos.first  + (i % shape.cols);        
        map.blocks[row][col] = true; 
    }
}

//Draw the map. Mostly for debug purposes
//(If you uncomment all print statements in main(), the 
//output will be similar to the text in the example)
void draw_map(const Map& map){
    for(size_t row = map.blocks.size(); row!=0; --row){
        std::cout << "|";
        for(int col = 0; col<N_cols; col++){
            if(map.blocks[row-1][col]){
                std::cout << "#";
            }else{
                std::cout << " ";
            }
        }
        std::cout << "|" << std::endl;
    }
    std::cout << "+-------+" << std::endl;
}

//Find the current highest occupied block in the matrix
int highest_block(const Map& map){
    for(size_t row = map.blocks.size(); row!=0; --row){
        for(int col = 0; col<N_cols; col++){
            if(map.blocks[row-1][col]){
                return row-1;
            }
        }
    }
    return 0;
}

//Pyroclastic Flow
int main(){    
    
    //Obtain the single line of input (wind directions)
    std::fstream infs("input.txt");    
    std::string line;
    std::getline(infs,line);
    std::size_t N_directions = line.length();

    //Define the shapes
    const int N_shapes = 5;
    std::array<Shape,N_shapes> shapes;

    //The "horizontal dash"
    shapes[0].rows   = 1;
    shapes[0].cols   = 4;
    shapes[0].blocks = {1,1,1,1};
    
    //The "cross"
    shapes[1].rows   = 3;
    shapes[1].cols   = 3;
    shapes[1].blocks = {0,1,0,1,1,1,0,1,0};

    //The "left L"
    shapes[2].rows   = 3;
    shapes[2].cols   = 3;
    shapes[2].blocks = {1,1,1,0,0,1,0,0,1};

    //The "vertical line"
    shapes[3].rows   = 4;
    shapes[3].cols   = 1;
    shapes[3].blocks = {1,1,1,1};

    //The "square"
    shapes[4].rows   = 2;
    shapes[4].cols   = 2;
    shapes[4].blocks = {1,1,1,1};

    //Map that holds the "board". Grows dynamically as needed by adding empty rows
    Map map;
    std::array<bool,N_cols> empty_row;
    empty_row.fill(false);

    //For part 2: keep track of unique board positions (key = "hash" of game state and board state)
    std::unordered_map<std::string,int> keys; 
    std::string repeat_key;

    //Start dropping shapes   
    int current_height = 0;                 //Current maximum height achieved
    pos_type      pos{2,current_height+3};  //Position of piece while dropping
    pos_type prev_pos{2,current_height+3};  //Position of piece during previous time step
    int direction_counter = 0;              //Wind direction index
    std::vector<int> heights;               //Height achieved after every dropped piece

    int rock;
    for(rock = 0; rock<10000; rock++){

        //std::cout << "Rock " << rock+1 << " begins falling. (" << pos.first << ","<< pos.second << ")" << std::endl;

        //Make sure the map is large enough to receive our piece
        while(map.blocks.size() < current_height + 10){
            map.blocks.push_back(empty_row);
        }
        
        //Block index cycles through the shapes in a fixed order
        int block_index = rock%N_shapes;
        
        while(1){
            //Move to the left or to the right
            char direction = line[direction_counter % N_directions];
            direction_counter = (direction_counter+1)%N_directions;
            if(direction == '>'){
                pos.first += 1;
                //std::cout << "Jet of gas pushes rock right (" << pos.first << ","<< pos.second << "):" ;                              
            }else{ 
                pos.first -= 1;
                //std::cout << "Jet of gas pushes rock left(" << pos.first << ","<< pos.second << "):";
            }

            //Check if the jet push was legal
            if(detect_collision(map,shapes[block_index],pos)){
                //Reject move
                pos = prev_pos;                
                //std::cout << ", but nothing happens." << std::endl;   
            }else{
                //Accept move
                prev_pos = pos;
                //std::cout << std::endl;             
            }

            //Try to move the block down
            pos.second -= 1;

            //Check if moving down was legal
            if(detect_collision(map,shapes[block_index],pos)){
                //Rock got stuck
                //std::cout << "Rock falls 1 unit, causing it to come to rest" << std::endl;
                //std::cout << pos.first << "," << pos.second << std::endl;
                write_shape(map,shapes[block_index],prev_pos);
                break;
            }else{
                //Accept move
                //std::cout << "Rock falls one unit(" << pos.first << ","<< pos.second << ")" << std::endl;               
                prev_pos = pos;
            }    
                  
        }

        //Store current stack height
        current_height = highest_block(map) + 1;
        heights.push_back(current_height);

        //Find a repeating section in the input file and in the map data.
        //Search for repeating combinations of
        // - shape index
        // - wind direction
        // - top 20 rows of the board
        //20 rows might not be strictly enough to cover all cases. Raising this number
        //will increase how sure the match is, but impacts runtime. If you get
        //incorrect answers, increase this number
        int search_repeat = 20;
        if(current_height > search_repeat){
            std::string key;
            //"Hash" the game state (probably something better could be done, but this works)
            key = std::to_string(block_index) + "_" + std::to_string(direction_counter) + "_";        
            for(int r = 0; r<search_repeat; r++){
                for(int c = 0; c<7; c++){
                    key += std::to_string(map.blocks[highest_block(map)-r][c]);
                }                
            }   
            //Check if this game state is unique, if not, add it to the hash map
            if(keys.find(key) == keys.end()){
                keys[key] = rock;
            }else{
                repeat_key = key;
                std::cout << "Sequence repeats after " << rock << " rocks!" << std::endl;
                break;
            }
        }
        
        //Break out of the loop when we have found the key
        if(repeat_key != ""){
            break;
        }

        //Reset the shape, prepare for dropping a new one
        pos.first = 2;
        pos.second = current_height + 3;
        //std::cout << "Map after rock has come to stand still:: " << std::endl;
        //draw_map(map);
        //std::cout << "current highest block: " << current_height << std::endl;
        
    }

    //We can at least answer part 1, but part 2 can only run if a repeating section is found
    if(repeat_key == ""){        
        std::cout << "Tower after 2022 blocks is " << heights[2021] << " blocks tall" << std::endl;
        throw std::runtime_error("No repeating section was found!!");
    }

    //Calculate length of repeating sections in terms of number of rocks dropped and 
    //height difference obtained during that section.
    long long drock = rock - keys[repeat_key];
    long long dh    = current_height - heights[keys[repeat_key]];

    //Part 1 answer    
    long long Nrocks  = 2022;
    long long start   = keys[repeat_key];
    long long repeats = (Nrocks-start)/drock;
    long long height  = heights[Nrocks - drock*repeats - 1] + repeats*dh ; 
    std::cout << "Tower after 2022 blocks is " << height << " blocks tall" << std::endl;  

    //Part 2 answer
    Nrocks  = 1000000000000;
    start   = keys[repeat_key];
    repeats = (Nrocks-start)/drock;
    height  = heights[Nrocks - drock*repeats - 1] + repeats*dh ; 
    std::cout << "Tower after 1000000000000 blocks is " << height << " blocks tall" << std::endl;    

    return 0;
}