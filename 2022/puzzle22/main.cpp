#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <algorithm>
#include <unordered_map>
#include "aoc_utility.hpp"

enum Facing{
    right=0,
    down=1,
    left=2,
    up=3
};

struct Node{
    int row;
    int col;
    std::array<int,4> neighbours;
    char tile;
};

//"hashing function" for (row,col) to id. This assumes there are less than 10.000
//columns in the map
int row_col_to_id(int row, int col){
    return 10000*row + col;
}
int id_to_row(int id){
    return id/10000;
}
int id_to_col(int id){
    return id%10000;
}

//Try to take a step in a given direction (facing). Return false if the step
//would hit a wall ('#'), true otherwise.
bool try_move(std::unordered_map<int,Node>& map, int& row, int& col, Facing facing){
    int id = row_col_to_id(row,col);
    const Node& neighbour = map.at(map.at(id).neighbours[facing]);
    if(neighbour.tile != '#'){
        row = neighbour.row;
        col = neighbour.col;
        return true;
    }else{
        return false;
    }
}

//Print the map (for debugging purposes)
void print_map(std::unordered_map<int,Node>& map, int Nrows, int Ncols){
    for(int row = 0; row<Nrows; row++){
        std::string line;
        for(int col = 0; col<Ncols; col++){
            int id = row_col_to_id(row,col);
            if(map.find(id) != map.end()){
                line += map.at(id).tile;
            }else{
                line += ' ';
            }
        }
        std::cout << line << std::endl;
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
    std::unordered_map<int,Node> map;
    Node node;
    int Nrows = 0;
    int Ncols = 0;
    int row = 0;
    
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
            for(int col = 0; col<line.size(); col++){
                Ncols = std::max(col+1,Ncols);
                if(line[col] != ' '){
                    node.row  = row;
                    node.col  = col;
                    node.tile = line[col];
                    int id    = row_col_to_id(row,col);   
                    map[id]  = node;
                }
            }
            Nrows = std::max(row+1,Nrows);
            row++;            
        }
    }

    //Determine for each node its neighbours. This process is the main difference between part 1 and 2
    for(auto& node : map){
        for(int facing = right; facing<=up; facing = Facing(facing+1)){
            int new_row = node.second.row;
            int new_col = node.second.col;
            if(part == 1){
                int d_row = (facing%2 == 1) ? 2 - facing : 0;
                int d_col = (facing%2 == 0) ? 1 - facing : 0;
                do{
                    new_row = aoc::mod(new_row+d_row,Nrows);
                    new_col = aoc::mod(new_col+d_col,Ncols);
                }while(map.find(row_col_to_id(new_row,new_col)) == map.end());                                                    
            }else{

            } 
            node.second.neighbours[facing] = row_col_to_id(new_row,new_col);
        }          
    }
    
    //Move to the starting coordinates
    int start_row = 0;
    int start_col;
    Facing facing = right;
    for(start_col = 0; start_col<10000; start_col++){
        int id = row_col_to_id(start_row,start_col);
        if(map.find(id) != map.end() && map.at(id).tile == '.'){
            std::cout << "starting at (" <<start_row << "," << start_col << ")"  << std::endl;
            break;
        }
    }    

    

    //Step through the instructions one at a time
    row = start_row;
    int col = start_col;
    for(const auto& instruction : instructions){

        int steps     = instruction.second;
        int new_row   = row;
        int new_col   = col;
        
        //First make N steps
        for(int step = 0; step<steps; step++){
            //Mark our steps in the map  
            const std::string signs = ">v<^";
            map.at(row_col_to_id(row,col)).tile = signs[facing];
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
    print_map(map,Nrows,Ncols);

    std::cout << "Password: " << std::to_string((row+1)*1000 + (col+1)*4 + facing) << std::endl;

    return 0;
}
