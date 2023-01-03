#include <iostream>
#include <string>
#include <fstream>
#include <list>
#include <cassert>
#include <vector>
#include <algorithm>
#include <unordered_map>
#include "aoc_utility.hpp"
#include <Eigen/Dense>

enum Facing{
    right=0,
    down=1,
    left=2,
    up=3
};

bool try_move(const Eigen::Matrix<char,-1,-1>& map, int& row, int& col, Facing facing){
    int new_row = row;
    int new_col = col;
    switch(facing){
        case up:
            new_row--;
            if(new_row==-1){
                new_row = map.rows()-1;
            }
            break;
        case down:
            new_row++;
            if(new_row==map.rows()){
                new_row = 0;
            }
            break;
        case left:            
            new_col--;
            if(new_col==-1){
                new_col = map.cols()-1;
            }
            break;
        case right:
            new_col++;
            if(new_col==map.cols()){
                new_col = 0;
            }
            break;
    }

    if(map(new_row,new_col) == '#'){
        //We hit a wall if we would take this step -> stop
        return false;
    }else if(map(new_row,new_col) == ' '){
        //We hit open space -> loop around to the other edge of the map
        if(try_move(map,new_row,new_col,facing)){
            row = new_row;
            col = new_col;
            return true;
        }else{
            return false;
        }
    }else{
        //attempted step was successful, take it
        row = new_row;
        col = new_col;
        return true;
    }
}

//Monkey Map
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle22 1       or      ./puzzle22 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);

    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;

    std::vector<std::pair<int,int>> instructions;

    //Load the map into a matrix, and simultaneously determine the maximum matrix dimensions
    Eigen::Matrix<char,-1,-1> map (1000,1000);
    map.setConstant(' ');
    int max_x = 0;
    int max_y = 0;
    int x = 0;
    while(std::getline(infs,line)){
        if(line == ""){
            //Part 2 starts: get the instructions
            std::getline(infs,line);
            std::string digit = "";
            std::pair<int,int> direction;
            for(char c : line){
                if(c >= '0' && c <= '9'){
                    digit += c;
                }else{
                    switch(c){
                        case 'R':
                            direction = { 1,std::stoi(digit)};
                            break;
                        case 'L':
                            direction = {-1,std::stoi(digit)};
                            break;
                        default:
                            throw std::runtime_error("Illegal direction encountered while parsing input");
                    }   
                    digit = "";
                    instructions.push_back(direction);
                }                
            }
            //Make sure the final instruction is also captured.
            direction = {0,std::stoi(digit)};
            instructions.push_back(direction);
            break;
        }else{
            //Part 1 contains the map
            for(int y = 0; y<line.size(); y++){
                map(x,y) = line[y];
                max_y = std::max(y,max_y);
            }
            max_x = std::max(x,max_x);
            x++;            
        }
    }
    //Resize the matrix to only fit the current map
    map.conservativeResize(max_x+1,max_y+1);
    
    //Move to the starting coordinates
    int start_row = 0;
    int start_col = 0;
    Facing facing = right;
    try_move(map,start_row,start_col,facing);
    std::cout << "starting at (" <<start_row << "," << start_col << ")"  << std::endl;

    //Mark our steps in the map  
    std::string signs = ">v<^";

    //Step through the instructions one at a time
    int row = start_row;
    int col = start_col;
    for(const auto& instruction : instructions){

        int steps     = instruction.second;
        int new_row   = row;
        int new_col   = col;
        
        //First make N steps
        for(int step = 0; step<steps; step++){
            map(row,col) = signs[facing];
            bool did_move = try_move(map,new_row,new_col,facing);
            if(did_move){                
                row = new_row;
                col = new_col;
            }else{
                break;
            }
        }

        //Change facing in place
        facing = Facing( (4 + (facing + instruction.first)%4) % 4 );
    }

    //Print the map
    std::cout << map << std::endl;

    std::cout << "Password: " << std::to_string((row+1)*1000 + (col+1)*4 + facing) << std::endl;

    return 0;
}
