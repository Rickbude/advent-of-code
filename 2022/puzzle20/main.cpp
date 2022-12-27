#include <iostream>
#include <string>
#include <fstream>
#include <list>
#include <cassert>
#include <vector>
#include <algorithm>
#include "aoc_utility.hpp"

//Advance an iterator through a list in a circular fashion
void advance_circular(std::list<int>::iterator &it, std::list<int>& list, long long shift){
    int mod_shift = std::abs(shift) % list.size();    
    for(int i = 0; i<mod_shift; i++){
        if(shift>0){
            std::advance(it,1);
            if(it == list.end()){
                it = list.begin();
            }            
        }else{
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

    //Linked list of the initial 
    std::vector<long long> initial_numbers;
    std::list<int> indices;

    //Read in the data
    int index = 0;
    long long decryption_key = (part == 1) ? 1 : 811589153;
    while(std::getline(infs,line)){
        initial_numbers.push_back(std::stoi(line)*decryption_key);
        indices.push_back(index);
        index++;
    }

    //Now do the mixing
    int rounds = (part == 1) ? 1 : 10;
    for(int round = 0; round<rounds; round++){
        for(int index = 0; index<indices.size(); index++){
            //Find this index in the linked list
            std::list<int>::iterator it = std::find(indices.begin(), indices.end(),index);

            //Make a copy of the iterator, and advance it N steps
            std::list<int>::iterator it2 = it;            
            advance_circular(it2,indices,1);
            indices.erase(it);
        
            //Find the new position of this index
            long long number = initial_numbers[index];
            advance_circular(it2,indices,number);

            //Insert the element in the new location
            indices.insert(it2,index);        
        }
    }

    //Find the original index of number 0
    auto it0 = std::find(initial_numbers.begin(), initial_numbers.end(), 0);
    int  index_zero = std::distance(initial_numbers.begin(),it0);
   
    std::cout << "index of number 0: " << index_zero << std::endl;

    std::list<int>::iterator it = std::find(indices.begin(),indices.end(),index_zero);
    std::list<int>::iterator it1000 = it;
    std::list<int>::iterator it2000 = it;
    std::list<int>::iterator it3000 = it;
    advance_circular(it1000,indices,1000);
    advance_circular(it2000,indices,2000);
    advance_circular(it3000,indices,3000);

    std::cout << "1000th number after 0: " << initial_numbers[*it1000] << std::endl;
    std::cout << "2000th number after 0: " << initial_numbers[*it2000] << std::endl;
    std::cout << "3000th number after 0: " << initial_numbers[*it3000] << std::endl;
    std::cout << "Sum: " << initial_numbers[*it1000] + initial_numbers[*it2000] + initial_numbers[*it3000] <<std::endl;

    return 0;
}
