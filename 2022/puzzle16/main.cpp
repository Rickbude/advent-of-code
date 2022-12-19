#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>
#include <regex>
#include <unordered_map>
#include "aoc_utility.hpp"
#include <cmath>

//A valve/chamber as read in from input file.
struct Valve{
    std::string id;
    int flowrate;
    std::vector<std::string> connections;
};
using valve_list  = std::unordered_map<std::string,Valve>;

//A more efficient representation of the valve.
//Furthermore, this representation only stores the 
//connections to "functional" valves, and id's are used instead of strings
//This combined cuts the runtime by about a factor 5
struct CompactValve{
    std::string name;
    int id;
    int flowrate;
    std::vector<int> distances;
    bool opened  = false;
    bool visited = false;
};
using compact_valve_list = std::vector<CompactValve>;

//Pressure released by all currently opened valves
int pressure_released_per_minute(const compact_valve_list& valves){
    int pressure_diff = 0;
    for(const auto& valve: valves){
        if(valve.opened){
            pressure_diff += valve.flowrate;
        }
    }
    return pressure_diff;
}

//Find the shortest distance from one valve to another
int dist_to_valve(const valve_list& valves, const std::string& start, const std::string& target){
    std::unordered_map<std::string,int> openSet;
    openSet[start]=0;

    while(!openSet.empty()){
        // This operation can occur in O(Log(N)) time if openSet is a
        // min-heap or a priority queue
        auto it = std::min_element(openSet.begin(), openSet.end(),
            [](const auto& l, const auto& r) { return l.second < r.second; }    
        );
        
        std::string current = (*it).first;
        int current_cost    = (*it).second;
        openSet.erase(it);        

        //We reached the goal, we are done!
        if(current == target){
            return current_cost;
        }       
        //visit all the neighbors of this node
        for(const std::string neighbor : valves.at(current).connections){
            double tentative_score = current_cost + 1;
            // This path to neighbor is better than any previous one. Record it!
            if(openSet.find(neighbor) == openSet.end() || tentative_score < openSet[neighbor]){
                openSet[neighbor] = tentative_score;
            }                  
        }        
    }

    throw std::runtime_error("Could not find path between nodes " + start + " and " + target);
}

//Find the optimal amount of pressure relieved when only working yourself
// valves:  valve list
// minutes: minutes remaining
// current: node we find ourselves at currently
//This function has (I think) a complexity of N!, with N the number of 
//valves that have non-zero flowrate. So this could take a while...
int pressure_released(compact_valve_list& valves, const int minutes, int current){
    
    //Pressure released this minute due to all currently opened valves
    int pressure_decrease_per_min = pressure_released_per_minute(valves);

    //The pressure released were we to do nothing
    int max_pressure_released = minutes*pressure_decrease_per_min;
    //Go to one of the unopened valves
    for(const auto& new_target : valves){
        //Go to a NEW valve
        if(new_target.id == current){
            continue;
        }
        //Only valves with a nonzero flowrate are interesting to consider
        if(new_target.opened || new_target.flowrate == 0){
           continue;
        }
        //Only consider valves that we can actually reach in time
        int distance = valves[current].distances[new_target.id];
        int minutes_remaining  = minutes-distance-1; 
        if(minutes_remaining < 0){
            continue;
        }

        //Select a new target and start walking
        valves[new_target.id].opened = true;
        int target_decrease   = pressure_released(valves,minutes_remaining,new_target.id) + (distance+1)*pressure_decrease_per_min;
        valves[new_target.id].opened = false;

        max_pressure_released = std::max(target_decrease,max_pressure_released);        
    }
    return max_pressure_released;    
}

//Proboscidea Volcanium
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle16 1       or      ./puzzle16 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;

    //INput is provided in this form
    std::regex line_expr("^Valve (\\w+) has flow rate=(\\d+); tunnels? leads? to valves? (.*)$");

    valve_list  valves;
    Valve valve;

    //Read in the data
    while(std::getline(infs,line)){
        //Parse input line
        std::smatch matches;
        std::regex_match (line,matches,line_expr);
        if(matches.size() != 4){
            throw std::runtime_error("Regex match failed while parsing input!");
        }
        valve.id = matches[1];                //Valve identifier
        valve.flowrate  = stoi(matches[2]);   //This valve's flowrate      

        //Get connected chambers
        valve.connections.clear();
        std::stringstream connections(matches[3]);
        std::string connection;
        while(std::getline(connections, connection, ',')){
            if(connection[0] == ' '){
                valve.connections.push_back(connection.substr(1));
            }else{
                valve.connections.push_back(connection);
            }
        }
        valves[valve.id] = valve;
    }

    //We start both parts at this valve
    std::string start = "AA";   
    int start_id;

    //Create a subgraph, containing only the functional valves
    compact_valve_list compact_valves;
    CompactValve compact_valve;  
    int counter = 0;  
    for(auto& valve : valves){
        if(valve.first == start || valve.second.flowrate > 0){
            if(valve.first == start){
                start_id = counter;
            }
            compact_valve.id        = counter;
            compact_valve.name      = valve.first;
            compact_valve.flowrate  = valve.second.flowrate;
            compact_valves.push_back(compact_valve);
            counter++;
        }        
    }

    //Pre-calculate the shortest distance from each valve to each other valve,
    //and the neighbor you need to move to to get there the fastest
    for(auto& start_valve : compact_valves){
        for(auto& target_valve : compact_valves){
            int distance = dist_to_valve(valves,start_valve.name,target_valve.name);
            start_valve.distances.push_back(distance);
        }
    }   

    //Okay, now let's do this..    
    if(valves[start].flowrate > 0){
        throw std::runtime_error("Current algorithm assumes that the flowrate of the start node is 0");
    }

    if(part == 1){
        //Part 1 is a simple breadth-first search of the graph      
        std::cout << "Total pressure released: " << pressure_released(compact_valves,30,start_id) << std::endl;
    }else{
        //Brute force solution to part 2 (takes 2-3 minutes)
        //Split the valves-to-be-opened in two subsets: one for you, one for the elephant.
        //See which distribution (and there are ~2^15 of them..) works best.
        //I am sure this can be done in a smarter way.
        int max_score = 0;
        compact_valve_list you = compact_valves;
        compact_valve_list ele = compact_valves;
        for(int i = 0; i<std::pow(2,compact_valves.size()); i++){
            int j = 0;
            for(const auto& valve : compact_valves){
                if(i & (1<<j)){
                    you[valve.id].flowrate = 0;
                    ele[valve.id].flowrate = valve.flowrate;
                }else{
                    ele[valve.id].flowrate = 0;
                    you[valve.id].flowrate = valve.flowrate;
                }
                j++;                
            }
            int score_you = pressure_released(you,26,start_id);
            int score_ele = pressure_released(ele,26,start_id);
            max_score     = std::max(max_score, score_you + score_ele);
        }
        std::cout << "Total pressure released: " << max_score << std::endl;
    }
    return 0;
}