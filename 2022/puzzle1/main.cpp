#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

/*
    The Christmas elves are going on a hike, and need to determine who has the
    most food (calorie-wise).
*/
int main(){
    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");
    
    //Calculate food per elf
    std::vector<int> food;
    int calories = 0;
    std::string line;
    while(std::getline(infs,line)){
        if(line == ""){
            //Empty line = new elf
            food.push_back(calories);
            calories = 0;
        }else{
            calories += std::atoi(line.c_str());
        }
    }

    //Sorting is overkill for part 1, but makes part 2 very simple
    std::sort(food.begin(),food.end(),std::greater<int>());
    std::cout << "Elf with most food has: " << food[0] << " Calories" << std::endl;
    std::cout << "Top three elves combined have: " << food[0] + food[1] + food[2] << " Calories" << std::endl;

    return 0;
}