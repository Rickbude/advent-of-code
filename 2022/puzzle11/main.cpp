#include <iostream>
#include <string>
#include <fstream>
#include <deque>
#include <sstream>
#include <vector>
#include <algorithm>
#include <set>
#include "aoc_utility.hpp"
#include <cmath>

struct Monkey{  
    int              id;                // Monkey ID
    std::vector<long> items;            // List of items
    char             operation;         // "+" or "*"
    long              arg1 = -1;        // number, or -1 for "old"
    long              arg2 = -1;        // number, or -1 for "old"
    int              divisible_by;      // Test: divisible by this number??
    int              monkey_if_true;    // Throw to this monkey if test is true
    int              monkey_if_false;   // Throw to this monkey if test is false
    int              activity=0;        // Number of inspected items
};

//Monkey business
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle11 1       or      ./puzzle11 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and parse the input
    std::fstream infs("input.txt");    
    std::string line;

    std::vector<Monkey> monkeys;
    Monkey monkey;

    //"super modulo": use this to keep the worry levels under control in part 2
    int supermod = 1;
    while(std::getline(infs,line)){

        //Split the input line at the ','
        size_t delim_pos = line.find(':');
        std::string part1 = line.substr(0,delim_pos);
        std::string part2 = line.substr(delim_pos+1);

        if(part1.substr(0,6) == "Monkey"){
            //Monkey \d --> monkey ID
            monkey.id = part1[7] - '0';
        }else if(part1 == "  Starting items"){
            //starting items
            monkey.items.clear();
            std::stringstream starting_items(part2);
            std::string item;
            while(std::getline(starting_items, item, ','))
            {
                monkey.items.push_back(std::stoi(item));
            }
        }else if(part1 == "  Operation"){
            //Operation
            monkey.operation = part2[11];
            monkey.arg1      = -1;
            std::string arg2 = part2.substr(13);
            if(arg2 == "old"){
                monkey.arg2 = -1;
            }else{
                monkey.arg2 = std::stoi(arg2);
            }
        }else if(part1 == "  Test"){
            //Test. Only checks if divisible by
            monkey.divisible_by    = std::stoi(part2.substr(14));
            supermod *= monkey.divisible_by;
        }else if(part1 == "    If true"){
            //Throw to monkey X if true
            monkey.monkey_if_true  = part2[17] - '0';
        }else if(part1 == "    If false"){
            //Throw to monkey Y if false. 
            monkey.monkey_if_false = part2[17] - '0';
            //We are done with this monkey
            monkeys.push_back(monkey);
        }              
    }
    
    //Run the 20 (part 1) or 10.000 (part 2) rounds
    int Nrounds = (part == 1) ? 20 : 10000;
    for(int round = 0; round<Nrounds; round++){
        for(Monkey& monkey: monkeys){
            for(long item: monkey.items){
                long old_val = item;
                long new_val;                
                long arg1 = (monkey.arg1 == -1) ? old_val : monkey.arg1;
                long arg2 = (monkey.arg2 == -1) ? old_val : monkey.arg2;

                //Do an addition or multiplication. Use the modulo to keep
                //the operands small enough to avoid overflow
                if(monkey.operation == '+'){
                    new_val = (arg1%supermod) + (arg2%supermod);
                }else{
                    new_val = (arg1%supermod) * (arg2%supermod);
                }

                //Only in part 1, the worry level decreases
                if(part == 1){
                    new_val = new_val/3;
                }                

                //Add this item to the proper monkey's inventory
                if(new_val % monkey.divisible_by == 0){
                    monkeys[monkey.monkey_if_true ].items.push_back(new_val % supermod);
                }else{
                    monkeys[monkey.monkey_if_false].items.push_back(new_val % supermod);
                } 

                //Increase the monkey activity
                monkey.activity++;               
            }
            //Clear this monkey's inventory (all items have been transferred)
            monkey.items.clear();
        }
    }

    //Print the activity and currently held items for each monkey
    std::vector<long> activities;
    for(Monkey& monkey: monkeys){
        activities.push_back(monkey.activity);
        std::cout << "monkey " << monkey.id << " has activity " << monkey.activity << " and holds: ";
        for(long item: monkey.items){
            std::cout << item << ", ";
        }
        std::cout << std::endl;
    }
    
    //Calculate the Monkey business score
    std::sort(activities.begin(), activities.end(),std::greater<long>());
    std::cout << "monkey activity score " << activities[0]*activities[1] << std::endl; 

    return 0;
}