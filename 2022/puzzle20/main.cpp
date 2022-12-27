#include <iostream>
#include <string>
#include <fstream>
#include <list>
#include <cassert>
#include <algorithm>
#include "aoc_utility.hpp"

void advance_circular(std::list<int>::iterator &it, std::list<int>& list, int shift){
    if(shift>0){
        for(int i = 0; i<=shift; i++){
            std::advance(it,1);
            if(it == list.end()){
                it = list.begin();
            }
        }
    }else{
        for(int i = 0; i>shift; i--){
            std::advance(it,-1);
            if(it == list.begin()){
                it = list.end();
            }
        }
    }
}

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

    std::list<int> numbers;
    //Read in the data
    while(std::getline(infs,line)){
        numbers.push_back(std::stoi(line));
    }

    std::list<int> initial_numbers = numbers;
    
    //Now do the circular shifts
    for(int number : initial_numbers){
        std::list<int>::iterator it = std::find(numbers.begin(), numbers.end(),number);
        std::list<int>::iterator it2 = it;
        advance_circular(it2,numbers,number%numbers.size());
        numbers.splice(it2,numbers,it);
    }

    std::list<int>::iterator it = std::find(numbers.begin(), numbers.end(), 0);
    std::list<int>::iterator it1000 = it;
    std::list<int>::iterator it2000 = it;
    std::list<int>::iterator it3000 = it;
    advance_circular(it1000,numbers,1000-1);
    advance_circular(it2000,numbers,2000-1);
    advance_circular(it3000,numbers,3000-1);

    std::cout << "1000th number after 0: " << *it1000 << std::endl;
    std::cout << "2000th number after 0: " << *it2000 << std::endl;
    std::cout << "3000th number after 0: " << *it3000 << std::endl;
    std::cout << "Sum: " << (*it1000) + (*it2000) + (*it3000) <<std::endl;

    //Too high: 11645

    return 0;
}