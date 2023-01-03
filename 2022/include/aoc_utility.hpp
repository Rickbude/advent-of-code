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

    //Return a mod b, but in contrast to the built-in modulo (%), the result is
    //guaranteed to be between 0 and b. A practical example:
    // -7 % 3    = -1
    // mod(-7,3) =  2
    template<typename T>
    T mod(T a, T b){
        return (b + (a % b)) % b;
    }
}