#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <array>
#include <set>
#include <algorithm>
#include "aoc_utility.hpp"

//Type of the node of the rope
struct node{    
    int x = 0;
    int y = 0;    
};

//Returns the sign of the number:
// +1 for positive numbers
// -1 for negative numbers
//  0 for.. 0
int sign(int number){
    if(number < 0){
        return -1;
    }else if(number > 0){
        return 1;
    }else{
        return 0;
    }
}

//Update the tail position. If the tail is touching the 
//preceding 
void update_tail(const node& head, node& tail){
    int dx    = head.x-tail.x;
    int dy    = head.y-tail.y;

    //Calculate squared distance
    int dist2 = dx*dx + dy*dy;
    
    if(dist2 <= 2){
        //Head and tail are touching -> nothing to do
        return;
    }

    //Update tail position
    tail.y += sign(dy);
    tail.x += sign(dx);    
}

//Decode the elves's data stream
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle9 1       or      ./puzzle9 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and read its only line
    std::fstream infs("input.txt");    
    std::string line;

    //Keep track of which squares have been visited by the head and tail nodes
    std::set<std::string> tail_visited;
    std::set<std::string> head_visited;

    //Chain of nodes
    std::vector<node> nodes;
    if(part == 1){
        nodes.resize(2);
    }else{
        nodes.resize(10);
    }

    while(std::getline(infs,line)){
        char direction = line[0];
        int  steps     = std::stoi(line.substr(2));

       
        for(int step = 0; step<steps; step++){
            //Update the head position
            node& head = *nodes.begin();
            switch(direction){
                case 'U':
                    head.y++;
                    break;
                case 'D':
                    head.y--;
                    break;
                case 'L':
                    head.x--;
                    break;
                case 'R':
                    head.x++;
                    break;
                default:
                    throw std::runtime_error("Illegal move direction!!");
            
            } 

            //Update the other nodes
            for(int i = 0; i<nodes.size()-1; i++){
                update_tail(nodes[i],nodes[i+1]);
            }           
            
            //Register the head and tail positions
            node& tail = nodes.back();

            std::string curr_head_pos = std::to_string(head.x)  + "_" + std::to_string(head.y);
            std::string curr_tail_pos = std::to_string(tail.x)  + "_" + std::to_string(tail.y);
            head_visited.insert(curr_head_pos);
            tail_visited.insert(curr_tail_pos);
        }
    }   

    std::cout << "head visited: " << head_visited.size() << " locations" << std::endl;
    std::cout << "tail visited: " << tail_visited.size() << " locations" << std::endl;

    return 0;
}