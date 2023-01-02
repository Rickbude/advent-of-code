#include <iostream>
#include <string>
#include <fstream>
#include <list>
#include <cassert>
#include <vector>
#include <algorithm>
#include "aoc_utility.hpp"

//Advance an iterator in a circular fashion over a list (loops around when the
//pointer reaches list.begin() or list.end()). The iterator always points to an
//existing element. (Keep in mind that list.end() points to a non-existing
//past-the-end-element.)
void advance_circular(std::list<int>::iterator &it, std::list<int>& list, long long shift){
    //Nothing needs to be done if the shift is 0
    if(shift == 0){
        return;
    }

    //Take the modulo to avoid doing unnecessary work when doing large shifts
    int mod_shift = std::abs(shift) % list.size();
    
    //Check if moving the other way is quicker
    int direction = shift > 0 ? 1 : -1;
    if(mod_shift > list.size() / 2){
        direction *= -1;        
        mod_shift = list.size() - mod_shift;
    }

    for(int i = 0; i<mod_shift; i++){
        if(direction>0){
            //For forward shifts, first advance the iterator, and then loop
            //around once the iterator reaches list.end(), to avoid pointing at
            //nothing.
            std::advance(it,1);
            if(it == list.end()){
                it = list.begin();
            }            
        }else{
            //For backward shifts, first loop around when the iterator points to
            //list.begin(), and then decrement the iterator, to avoid pointing
            //at nothing.
            if(it == list.begin()){
                it = list.end();
            }
            std::advance(it,-1);            
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
    std::vector<std::list<int>::iterator> iterators;

    //One iterators for every element in the list.
    //This gets (manually) updated every time that an element is inserted or removed.
    std::list<int>::iterator iterator;

    //Read in the data
    int index = 0;
    long long decryption_key = (part == 1) ? 1 : 811589153;
    while(std::getline(infs,line)){
        initial_numbers.push_back(std::stoi(line)*decryption_key);
        indices.push_back(index);
        //Store an iterator pointing to this list element
        if(index == 0){
            iterator = indices.begin();
        }else{
            iterator++;
        }
        iterators.push_back(iterator);
        index++;
    }

    //Now do the mixing
    int rounds = (part == 1) ? 1 : 10;
    std::list<int> tmp;
    for(int round = 0; round<rounds; round++){
        for(int index = 0; index<indices.size(); index++){
            //Obtain the amount with which the element needs to be shifted
            long long number = initial_numbers[index];

            //Get an iterator pointing to the element to be moved
            std::list<int>::iterator it = iterators[index];

            //Make a copy of the iterator, and advance it one steps, such that
            //it points to the next element (in a circular fashion).
            std::list<int>::iterator it2 = it;            
            advance_circular(it2,indices,1);
            
            //Now erase the original element.
            indices.erase(it);
            
            //Find the new position of the element
            advance_circular(it2,indices,number);
            
            //Insert the element in the new location
            indices.insert(it2,index); 

            //Update the vector of iterators.
            advance_circular(it2,indices,-1);
            iterators[index] = it2;
        }
    }

    //Find the original index of number 0, and an iterator pointing to it
    auto it0 = std::find(initial_numbers.begin(), initial_numbers.end(), 0);
    int  index_zero = std::distance(initial_numbers.begin(),it0);
    std::list<int>::iterator it = iterators[index_zero];

    //Calculate the answer (sum of the 1000th, 2000th and 3000th value)
    long long coordinate_product = 0;
    for(int i = 1000; i<=3000; i+=1000){
        std::list<int>::iterator it2 = it;
        advance_circular(it2,indices,i);
        long long number = initial_numbers[*it2];
        std::cout << i << "th number after 0: " << number << std::endl;
        coordinate_product += number;
    }
    std::cout << "Sum of values (answer): " << coordinate_product <<std::endl;

    return 0;
}