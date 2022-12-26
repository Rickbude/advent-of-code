#include <iostream>
#include <string>
#include <fstream>
#include <list>
#include <regex>
#include <cassert>
#include <Eigen/Dense>
#include "aoc_utility.hpp"

//Grove Positioning System
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle20 1       or      ./puzzle20 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);

    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;

    //Input follows pattern: xx,yy,zz
    std::regex line_expr("^Blueprint (\\d+): Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.$");

    std::list<int> numbers;
    //Read in the data
    while(std::getline(infs,line)){
        numbers.push_back(std::stoi(line));
    }

    //Now do the circular shifts
    for(int i = 0; i<numbers.size(); i++){
        int number = *numbers.begin();
        std::iterator it;
        if(number > 0){
            it = numbers.begin();
        }else{
            it = numbers.end();
        }
        std::advance(it,number);
        numbers.insert(it,number);
    }

    for(const auto& it : numbers){
        std::cout << it << std::endl;
    }

    
    return 0;
}