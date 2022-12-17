#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>
#include <Eigen/Dense>
#include "aoc_utility.hpp"

// The data in the map is stored as [row,col] = [x,y],
// where y = 0 represents the top of the map.
using map_type = Eigen::Matrix<char,-1,-1>;
using pos_type = Eigen::Vector2i;


//Move a grain of sand 1 step down.
//Returns true if the sand could move, false if it could not move
//Note: sand is dropped from y = 0, with and falls towards y = n_rows
bool move_sand(const map_type& map, pos_type& sand_pos){
    
    //Sand reached bottom
    if(sand_pos[1] > (map.rows()-2)){
        return false;
    }

    //Attempt to fall straight down
    if(map(sand_pos[0],sand_pos[1]+1) == '.'){
        //Free space --> fall        
        sand_pos[1]++;
        return true;      
    }

    //Attempt to fall diagonally to the left
    if(map(sand_pos[0]-1,sand_pos[1]+1) == '.'){
        //Free space --> fall        
        sand_pos[0]--;
        sand_pos[1]++;
        return true;   
    }

    //Attempt to fall diagonally to the right
    if(map(sand_pos[0]+1,sand_pos[1]+1) == '.'){
        //Free space --> fall        
        sand_pos[0]++;
        sand_pos[1]++;
        return true;   
    }

    //Sand can't fall anymore -> we are done
    return false;
}

//Draw the map
void draw_map(const map_type& map,const pos_type& lower_left,const pos_type& upper_right){    
    for(int col = lower_left[1]; col<upper_right[1]+1; col++){    
        for(int row = lower_left[0]; row<upper_right[0]+1; row++){    
            std::cout << map(row,col);
        }
        std::cout << std::endl; 
    }       
}

//Regolith Reservoir
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle14 1       or      ./puzzle14 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;
    
    //This matrix is - of course - way too big.
    //In principle you could go through the input file, determine min and max
    //x/y positions, and accommodate only for that.
    Eigen::Matrix<char,-1,-1> map(1000,250);
    map.setConstant('.');

    //The position of the "sand spawn"
    pos_type start_pos{500,0};

    pos_type lower_left  = start_pos;
    pos_type upper_right = start_pos;

    //Read in the data
    while(std::getline(infs,line)){
        bool do_draw = false;
        std::array<int,2> prev;
        std::string delimiter = " -> ";
        size_t delim_pos = 0;
        while(1){

            //Split each line on " -> "
            size_t delim_pos = line.find(delimiter);
            std::string segment = line.substr(0,delim_pos);
            line.erase(0,delim_pos + delimiter.length());

            //Split each segment on ','
            size_t pos = segment.find(',');
            int x = std::stoi(segment.substr(0,pos));
            int y = std::stoi(segment.substr(pos+1));

            if(do_draw){
                //Draw a line (technically speaking a block)
                for(int i = std::min(x,prev[0]); i!=std::max(x,prev[0])+1; i++){
                    for(int j = std::min(y,prev[1]); j!=std::max(y,prev[1])+1; j++){
                        map(i,j) = '#';
                    }
                }
            }
            prev[0] = x;
            prev[1] = y;
            do_draw = true;

            //Establish the draw limits of the map
            upper_right[0] = std::max(x,upper_right[0]);
            upper_right[1] = std::max(y,upper_right[1]);
            lower_left[0]  = std::min(x,lower_left[0]);
            lower_left[1]  = std::min(y,lower_left[1]);

            //No -> was found -> last coordinate pair of this line is drawn
            if(delim_pos == std::string::npos){
                break;
            }
        }
    }

    //For part 2, add a "shelf" below the map
    if(part == 2){
        int y = upper_right[1]+2;
        for(int x = 0; x<map.rows(); x++){
            map(x,y) = '#';
        }
        //Increase the draw boundaries to include (part of) the shelf
        upper_right[1] += 3;
        upper_right[0] += 3;
        lower_left[0]  -= 3;
    }

    //Add the "sand dropper"
    map(start_pos[0],start_pos[1]) = '+';

    //Draw the map
    std::cout << "Initial map:" << std::endl;    
    draw_map(map,lower_left,upper_right);

    //Drop sand
    for(int i = 0; i<1000000; i++){
        bool sand_rest = false;
        pos_type sand_pos = start_pos;
        while(move_sand(map,sand_pos)){
           //std::cout << sand_pos.transpose() << std::endl;
        }
        map(sand_pos[0],sand_pos[1]) = 'o';

        //For part 2, adjust the plot boundaries, to make sure all the 
        //sand fits in the picture
        if(part == 2){
            lower_left[0]   = std::min(sand_pos[0],lower_left[0]);
            upper_right[0]  = std::max(sand_pos[0],upper_right[0]);
        }
        
        if(sand_pos[1] == map.rows()-1){   
            //End condition for part 1 reached (sand is flowing out of the map)
            sand_pos = start_pos;        
            //Add the final trail of sand for fun
            while(move_sand(map,sand_pos)){
                map(sand_pos[0],sand_pos[1]) = '~';                
            }
            std::cout << "Final map:" << std::endl;
            draw_map(map,lower_left,upper_right);
            std::cout << "Sand starts falling into the abbyss after " << i << " grains of sand" << std::endl;
            break;
        }else if(sand_pos == start_pos){
            //End condition for part 2 reached (sand reached source)
            std::cout << "Final map:" << std::endl;
            draw_map(map,lower_left,upper_right);
            std::cout << "Sand stopped falling (reached source) after " << i+1 << " grains of sand" << std::endl;
            break;
        }
    }
    
    return 0;
}