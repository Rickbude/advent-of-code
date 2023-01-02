#include <iostream>
#include <string>
#include <fstream>
#include <list>
#include <cassert>
#include <vector>
#include <algorithm>
#include <unordered_map>
#include <regex>
#include <variant>
#include "aoc_utility.hpp"

//A monkey that will perform some math operation (+,-,*,/,=)
//Inherits from 
struct MathMonkey{
    char operation;
    std::string monkey1;
    std::string monkey2;
};

//A monkey that will only yell a certain number
struct YellMonkey{
    long long number;
};

//Use a map, with as keys the monkey names, and as values the monkey. An
//std::variant is used, such that both types of monkey can be stored in this map.
using monkey_list = std::unordered_map<std::string,std::variant<MathMonkey,YellMonkey>>;

//Recursive function that calculates what value a monkey will yell
//Template this function with a type T, such that we can call it in double "mode" and in long long "mode"
template<typename T>
T resolve_monkey(const monkey_list& monkeys, const std::string& monkey){
    if(monkeys.find(monkey) == monkeys.end()){
        throw std::runtime_error("Could not find monkey " + monkey);
    }
    if(std::holds_alternative<YellMonkey>(monkeys.at(monkey))){
        const auto& yell_monkey = std::get<YellMonkey>(monkeys.at(monkey));
        return yell_monkey.number;
    }else{
        const auto& math_monkey = std::get<MathMonkey>(monkeys.at(monkey));
        T value1 = resolve_monkey<T>(monkeys,math_monkey.monkey1);
        T value2 = resolve_monkey<T>(monkeys,math_monkey.monkey2);
        switch(math_monkey.operation){
            case '+':
                return value1 + value2;
            case '/':
                return value1 / value2;
            case '*':
                return value1 * value2;
            case '-':
                return value1 - value2;
            case '=':
                return value1 - value2;
            default:
                throw std::runtime_error("Math operation not handled yet");
        }
    }
}

//Monkey Math
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle21 1       or      ./puzzle21 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);

    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;

    //Math monkeys follow a pattern like: ^abcd: defg + hijk$
    //Yell monkeys follow a pattern like: ^abcd: 123$
    std::regex math_monkey_regex("^(\\w+): (\\w+) ([+\\-*/]) (\\w+)$");
    std::regex yell_monkey_regex("^(\\w+): (\\d+)$");

    monkey_list monkeys;

    while(std::getline(infs,line)){
        std::smatch matches_math;
        std::smatch matches_yell;
        if(std::regex_match (line,matches_math,math_monkey_regex)){
            //We found a math monkey, doing some operation
            MathMonkey math_monkey;
            std::string name = matches_math[1];
            //In part 2, the root node should be treated as an equality
            if(part == 2 && name == "root"){
                math_monkey.operation = '=';
            }else{
                math_monkey.operation = matches_math[3].str()[0];
            }            
            math_monkey.monkey1 = matches_math[2];
            math_monkey.monkey2 = matches_math[4];
            monkeys[name] = math_monkey;
        }else if(std::regex_match (line,matches_yell,yell_monkey_regex)){
            //We found a yell monkey that only yells a single number
            YellMonkey yell_monkey;
            std::string name = matches_yell[1];
            yell_monkey.number = std::stoi(matches_yell[2]);
            monkeys[name] = yell_monkey;
        }else{
            //Illegal pattern found. (This should not occur)
            throw std::runtime_error("Input line <" + line + "> does not confirm to any known pattern");
        }
    }

    //Get a pointer to the "human monkey"
    if(monkeys.find("humn") == monkeys.end()){
        //Human monkey not found, this should not occur.
        throw std::runtime_error("Could not find monkey 'humn'");
    }
    auto& human = std::get<YellMonkey>(monkeys.at("humn"));
    
    if(part == 2){
        // Use the secant method (https://en.wikipedia.org/wiki/Secant_method) to locate the 
        // roots of the polynomial formed by the polymial 
        // root: left = right (root: left-right = 0)

        //The secant method requires two start guesses, preferably close to the end value.
        //The solution to my puzzle input is of the order 1e13, so we start with that.
        //Play around with these starting values if this does not converge for you.
        double xn_1 = 1e12;
        double xn_2 = 1e13;

        //Obtain the function values at these two start points
        human.number = xn_1;
        double fn_1 = resolve_monkey<double>(monkeys,"root");
        human.number = xn_2;
        double fn_2 = resolve_monkey<double>(monkeys,"root");

        std::cout << "x1: " << xn_1 << ", f(x1): " << fn_1 << std::endl;
        std::cout << "x2: " << xn_2 << ", f(x2): " << fn_2 << std::endl;

        //Now do up to 100 iterations of the secant method (although 3 iterations suffice for my input)
        for(int i=3; i<100; i++){
            //Some debug output..
        
            double xn = (xn_2*fn_1 - xn_1*fn_2)/(fn_1 - fn_2);
            human.number = xn;
            double fn = resolve_monkey<double>(monkeys,"root");
            std::cout << "x"<<i<< ": " << xn << ", f(x" << i <<"): " << fn << std::endl;

            //Root was found, but we are using doubles. Check the few integer
            //values around this value, to make sure that the human shouts a correct integer value
            //(in my input 3 values were correct, and only the lowest value was allowed)
            if(std::abs(fn) == 0 ){
                const long long x = xn;
                std::cout << "Below are the possible values that result in equality (pick the lowest if more than 1): " << std::endl;
                for(int j = -5; j<5; j++){                
                    human.number = x + j;               
                    if(resolve_monkey<long long>(monkeys,"root") == 0){
                       std::cout << x+j << std::endl;
                    }
                }
                break;
            }
            
            //Rotate the values, prepare for next iteration of the secant method
            xn_2 = xn_1;
            fn_2 = fn_1;
            xn_1 = xn;            
            fn_1 = fn;
        }        
    }else{
        //For part 1, recursively calculate the value of the "root" monkey
        long long monkey_value = resolve_monkey<long long>(monkeys,"root");
        std::cout << "Root monkey says: " << monkey_value << std::endl;
    }
    return 0;
}