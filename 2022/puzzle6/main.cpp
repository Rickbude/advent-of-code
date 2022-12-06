#include <iostream>
#include <string>
#include <fstream>
#include <deque>
#include <algorithm>
#include <set>
#include "aoc_utility.hpp"

//Check if all characters in the buffer are unique by placing them into
//an std::set. A std::set doesn't allow duplicates. Another solution is
//to use std::unique, but that requires the data to be sorted first.
//Both methods have roughly NlogN complexity however (inserting into a 
//set has a complexity of logN according to cppreference)
bool all_chars_unique(std::deque<char> buffer){
    std::set<char> unique_chars;
    for(char c:buffer){
        unique_chars.insert(c);
    }
    return unique_chars.size() == buffer.size();
}

//Decode the elves's data stream
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle6 1       or      ./puzzle6 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and read its only line
    std::fstream infs("input.txt");    
    std::string line;
    std::getline(infs,line);    

    //Create the buffer of appropriate length (4 for part 1, 14 for part 2)
    std::deque<char> buffer;
    if(part == 1){
        buffer.resize(4);
    }else{
        buffer.resize(14);
    }
    
    int counter = 1;
    for(char c:line){
        //Shift all characters in the buffer one to the left (by pushing the new
        //character at the back, and popping the oldest character from the front)
        buffer.push_back(c);
        buffer.pop_front();
        //Check if we found the marker
        if(all_chars_unique(buffer) && counter > buffer.size()){
            std::cout << "marker found at " << counter << std::endl;
            break;
        }  
        counter++;
    }
    return 0;
}