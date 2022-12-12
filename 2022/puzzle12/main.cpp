#include <iostream>
#include <string>
#include <fstream>
#include <deque>
#include <sstream>
#include <vector>
#include <algorithm>
#include <set>
#include <queue>
#include "aoc_utility.hpp"
#include <Eigen/Dense>

//Position type
using pos_type = Eigen::Vector2i;


//
struct Node{
    int         id;
    pos_type    pos    ;
    int         height = 0;
    int         gscore = std::numeric_limits<int>::max();  //
    int         fscore = std::numeric_limits<int>::max();  //
    Node*       camefrom = nullptr;                        //The node we came from
    //Make sorting Nodes possible
    bool operator<(const Node& a) const
    {
        return (fscore < a.fscore);
    }
};

//Map type
using map_type = Eigen::Matrix<Node,-1,-1>;

int distance(const pos_type& position, const pos_type& start){
    return (position-start).cwiseAbs().sum();
}

//Current heuristic: Manhattan distance
int heuristic(const pos_type& position, const pos_type& target){
    return (position-target).cwiseAbs().sum();
}

// A* finds a path from start to goal.
// Translated from pseudocode on https://en.wikipedia.org/wiki/A*_search_algorithm
Node A_Star(pos_type start, pos_type goal, map_type& map){
    
    pos_type curr_pos   = start;
    Node* startnode     = &map(curr_pos[0],curr_pos[1]);
    startnode->gscore   = 0;
    startnode->fscore   = heuristic(curr_pos,goal);


    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    std::deque<Node> openSet;
    openSet.push_back(*startnode);
    
    while(!openSet.empty()){
        std::cout << "set size: " << openSet.size() << std::endl;
        Node current = *openSet.begin();
        curr_pos = current.pos;

        std::cout << "set contains nodes with fscores:" <<std::endl;
        for(const Node& node : openSet){
            std::cout << node.fscore << ", ";
        }
        std::cout << std::endl;

        //We reached the goal, we are done!
        if(current.pos == goal){
            std::cout << "WE ARE DONE" << std::endl;
            return map(curr_pos[0],curr_pos[1]);
        }

        openSet.pop_front();  
        
        //Go in each direction
        std::array<int,4> dx{-1, 1,  0, 0};
        std::array<int,4> dy{ 0, 0, -1, 1};

        for(int i = 0; i<4; i++){
            
            int row  = curr_pos[0] + dx[i];
            int col  = curr_pos[1] + dy[i];            

            //Only consider valid neighbors
            if(row>=0 && col >= 0 && row <map.rows() && col < map.cols()){

                pos_type neighbor_pos{row,col};
                Node* neighbor    = &map(row,col);

                //This is only a valid neigbour if we can actually climb it
                if(std::abs(neighbor->height - current.height) > 1){
                    continue;
                }

                int tentative_gScore = current.gscore + 1;

                if(tentative_gScore < neighbor->gscore){
                    neighbor->camefrom = &map(curr_pos[0],curr_pos[1]);
                    neighbor->gscore   = tentative_gScore;
                    neighbor->fscore   = tentative_gScore + heuristic(neighbor_pos,goal);

                    //Check if this neighbor is not in the queue. If not, add it
                    bool found = false;
                    for(const Node& node : openSet){
                        if(neighbor->id == node.id){
                            found = true;                            
                            break;
                        }
                    }
                    if(!found){
                        openSet.push_back(*neighbor);
                        std::sort(openSet.begin(), openSet.end());
                    }
                }
            }            
        }
    }
    std::cout << "Failure" << std::endl;
}


//Hill Climbing Algorithm
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle12 1       or      ./puzzle12 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    int N_rows = 0;
    int N_cols = 0;
    std::string line;
    while(std::getline(infs,line))    {
        N_rows++;
        N_cols = line.size();
    }

    pos_type startpos;
    pos_type endpos;

    //Reset input filestream
    infs.clear();
    infs.seekg(0);    

    //Read in the height map
    int counter = 0;
    map_type height_map(N_rows,N_cols);
    int row = 0;
    while(std::getline(infs,line)){
        int col = 0;
        for(char c : line){       
            int height;     
            if(c >= 'a' && c <= 'z'){
                height = c - 'a';
            }else if(c == 'S'){
                height = 0;
                startpos[0] = row;
                startpos[1] = col;
            }else if(c == 'E'){
                height = 25;
                endpos[0] = row;
                endpos[1] = col;
            }else{
                throw std::runtime_error("invalid char encountered");
            }
            
            Node node;
            node.id     = counter;
            node.height = height;
            node.pos[0] = row;
            node.pos[1] = col;
            height_map(row,col) = node;
            counter++;
            col++;
        }
        row++;
    }

    //Now, do A* search

    
    
    pos_type current_position = startpos;


    Node endnode = A_Star(startpos, endpos, height_map);
    Node* camefrom = endnode.camefrom;

    int n_steps = 0;
    while(camefrom != nullptr){
        std::cout << "came through node " << camefrom->id << std::endl;
        camefrom  = camefrom->camefrom;
        n_steps++;
    }

    std::cout << "route took " << n_steps << " steps" << std::endl;

   

    return 0;
}