#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <string>
#include <deque>
#include <regex>

//To store the crate, use a vector of deques. A deque is nice for this 
//assignment, because you can push/pop elements from both sides
using crate_stacks = std::vector<std::deque<char>>;

//TODO: would be nice if this was determined from input
const int N_crates = 9;

//Print the current crate configuration, and the "message" (the top crates)
void print_configuration(const crate_stacks &crates){
    //Print the crate configuration (top crates are the leftmost crates)
    for(int i = 0; i<crates.size(); i++){
        std::cout << i << ": ";
        for(int j = 0; j<crates[i].size(); j++){
            std::cout << "["<<crates[i][j] << "] , ";
        }
        std::cout << std::endl;
    }
    //Print the message / solution
    std::cout << "message: ";
    for(int i = 0; i<crates.size(); i++){
        std::cout << crates[i][0];
    }
    std::cout << std::endl;
}

/*
    Stacking crates..
*/
int main(){

    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");    
    
   
    std::string line;
    bool move_mode = false;
    crate_stacks crates(N_crates);

    //The lines that describe the moves look as follows:
    std::regex move_expr("^move (\\d+) from (\\d+) to (\\d+)$");
    while(std::getline(infs,line)){        
        if(!move_mode){
            //"arrangement mode": load the initial configuration of the crates
            for(int i = 0; i<crates.size(); i++){
                char crate_label = line[i*4+1];
                if(crate_label >= 'A' && crate_label <= 'Z'){
                    crates[i].push_back(crate_label);
                }                
            }
        }else{
            //"move mode": start shuffling the crates with the crane

            //Extract the move using regex
            std::smatch matches;
            std::regex_match (line,matches,move_expr);
            int quantity     = stoi(matches[1]);    //Number of crates to be moved
            int source_stack = stoi(matches[2]);    //Source stack
            int target_stack = stoi(matches[3]);    //Target stack

            //part 1 (enabled with compiler definition)
            #ifdef DO_PART1
                for(int i=0; i<quantity; i++){
                    //get label of top crate on the source stack
                    char crate_label = crates[source_stack-1][0];
                    //take this crate from the source stack, add it to the target stack
                    crates[source_stack-1].pop_front();
                    crates[target_stack-1].push_front(crate_label);
                }
            #endif
           
            //part 2 (enabled with compiler definition)  
            #ifdef DO_PART2         
                //First add the crates "in reverse order" to the target stack
                //Not the nicest way to do it, but okay..
                for(int i=0; i<quantity; i++){
                    char crate_label = crates[source_stack-1][quantity-i-1];                
                    crates[target_stack-1].push_front(crate_label);
                }
                //Delete the crates from the source stack
                for(int i=0; i<quantity; i++){
                    crates[source_stack-1].pop_front();
                }
            #endif
        }

        
        //and of arrangement mode, enable "move mode"
        if(line == ""){
            move_mode = true;
            //Print the configuration before shuffling crates
            std::cout << "configuration before shuffling crates:" << std::endl;
            print_configuration(crates);       
        }
    }

    //Plot the final solution
    std::cout << "configuration after shuffling crates:" << std::endl;
    print_configuration(crates);    

    return 0;
}