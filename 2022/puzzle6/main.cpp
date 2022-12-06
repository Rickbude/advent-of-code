#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <algorithm>
#include <set>

//Check if all characters in the buffer are unique by placing them into
//an std::set. A std::set doesn't allow duplicates. Another solution is
//to use std::unique, but that requires the data to be sorted first.
//Both methods have roughly NlogN complexity however (inserting into a 
//set has a complexity of logN according to cppreference)
bool all_chars_unique(std::vector<char> buffer){
    std::set<char> unique_chars;
    for(char c:buffer){
        unique_chars.insert(c);
    }
    return unique_chars.size() == buffer.size();
}

int main(int argc, char *argv[]){

    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");
    
    std::string line;
    //Load the only line from the input file
    std::getline(infs,line);

    //The "buffer". length 4 for part 1, length 14 for part 2
    //Length is set using input parameter:
    //./puzzle6 4        or        ./puzzle6 14
    //For part 1 and 2 respectively
    std::vector<char> buffer;
    if(argc == 2){
        buffer.resize(std::stoi(argv[1]));
        std::fill(buffer.begin(), buffer.end(),'0');
    }else{
        throw std::runtime_error("Provide a buffer size. Call this script as:  ./puzzle6 <buffer_size> ");
    }
    
    int counter = 1;
    for(char c:line){
        //Shift all characters in the buffer one to the left
        std::rotate(buffer.begin(),buffer.begin()+1,buffer.end());
        //Add the new character at the end
        buffer.back() = c;
        //Check if we found the marker
        if(all_chars_unique(buffer) && counter > buffer.size()){
            std::cout << "marker found at " << counter << std::endl;
            break;
        }  
        counter++;
    }
    return 0;
}