#include <iostream>
#include <string>

/*
    Small collection of some utility functions that should make coding for
    aoc just a bit more easy.
*/
namespace aoc{
    //Parse the program arguments, and extract the desired part number
    //Throws an error if no part number is provided, or if the part number is not 1 or 2
    int get_part_number(int argc, char* argv[]){
        if(argc == 2){
            int part = std::stoi(argv[1]);
            if(part != 1 && part != 2){
                std::string error_message;
                error_message = "Illegal part number (" + std::to_string(part) + ") used, should be 1 or 2!";
                throw std::runtime_error(error_message);
            }else{
                return part;
            }
        }else{
            throw std::runtime_error("This puzzle requires a part number (1 or 2) as input. Call this script as:  ./puzzle6 <part_number> ");
        }
    }
}