#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <string>
#include <deque>
#include <regex>

//TODO: determine from input
const int N_crates = 9;

void print_configuration(const std::vector<std::deque<char>> &crates){
    for(int i = 0; i<crates.size(); i++){
        std::cout << i << ": ";
        for(int j = 0; j<crates[i].size(); j++){
            std::cout << "["<<crates[i][j] << "] , ";
        }
        std::cout << std::endl;
    }
    std::cout << "message: ";
    for(int i = 0; i<crates.size(); i++){
        std::cout << crates[i][0];
    }
    std::cout << std::endl;
}

int main(){

    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");    
    
   
    std::string line;
    bool move_mode = false;
    std::vector<std::deque<char>> crates(N_crates);

    std::regex move_expr("^move (\\d+) from (\\d+) to (\\d+)$");
    while(std::getline(infs,line)){
        //"arrangement mode": load the initial configuration of the crates
        if(!move_mode){
            for(int i = 0; i<crates.size(); i++){
                char crate_label = line[i*4+1];
                if(crate_label >= 'A' && crate_label <= 'Z'){
                    crates[i].push_back(crate_label);
                }                
            }
        }else{
            

            std::smatch matches;
            std::regex_match (line,matches,move_expr);

            int quantity     = stoi(matches[1]);
            int source_stack = stoi(matches[2]);
            int target_stack = stoi(matches[3]);

            //part 1
            /*
            for(int i=0; i<quantity; i++){
                char crate_label = crates[source_stack-1][0];
                crates[source_stack-1].pop_front();
                crates[target_stack-1].push_front(crate_label);
            }
            */
           
            //part 2            
            for(int i=0; i<quantity; i++){
                char crate_label = crates[source_stack-1][quantity-i-1];                
                crates[target_stack-1].push_front(crate_label);
            }
            for(int i=0; i<quantity; i++){
                crates[source_stack-1].pop_front();
            }
            

            //std::cout << std::endl;

            //print_configuration(crates);
        }

        
        //and of arrangement mode, now we get to the moves
        if(line == ""){
            move_mode = true;     
            print_configuration(crates);       
        }
    }

    print_configuration(crates);
    
    

    return 0;
}