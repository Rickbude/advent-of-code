#include <iostream>
#include <string>
#include <fstream>

//Determine if the current clock cycle is "interesting" (according to part 1)
bool is_interesting_cycle(int clock){
    return (clock - 20) % 40 == 0;
}

//Draw a pixel to the screen. 
//Also loop around every 40th pixel
bool draw_CRT(const int clock, const int regX){
    //Determine what column the current pixel goes into
    //If the mask and current column overlap, draw a 
    //lit pixel (#), else a dark pixel (.)
    int column = (clock-1)%40;    
    if(std::abs(column - regX) <= 1){
        std::cout << "#";
    }else{
        std::cout << ".";
    }
    //Loop around every 40th pixel
    if(column == 39){
        std::cout << std::endl;
    }
}

//Debug the communication device
int main(int argc, char *argv[]){    

    int clock = 1;  //Clock counter
    int regX  = 1;  //X register of the computer

    //Answer for part 1
    int signal_strength_sum = 0;

    //Load input file (this file is copied to the build directory) and read its only line
    std::fstream infs("input.txt");    
    std::string line;
    while(std::getline(infs,line)){   
        //Instruction is (in this case) always 4 letters long: either "addx" or "noop"
        std::string instruction = line.substr(0,4); 

        //Draw a pixel on the screen
        draw_CRT(clock, regX);       
        if(instruction == "noop"){
            //For a no-op, only increment the clock
            clock++;
        }else if(instruction == "addx"){
            //For an addx, increment the clock twice. Draw a pixel in between
            //the increments. At the end of the second cycle, add to register X.
            //Check at every clock increment whether it is an "interesting value". 
            int value = std::stoi(line.substr(5));
            clock++;
            if(is_interesting_cycle(clock)){
                signal_strength_sum += clock*regX;
            }            
            draw_CRT(clock, regX);
            clock++;
            regX += value;
        }           
        
        if(is_interesting_cycle(clock)){
            signal_strength_sum += clock*regX;
        }       
    }   

    std::cout << "signal strength sum: " << signal_strength_sum << std::endl;
    return 0;
}