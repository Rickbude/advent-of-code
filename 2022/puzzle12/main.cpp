#include <iostream>
#include <string>
#include <fstream>
#include <deque>
#include <sstream>
#include <vector>
#include <algorithm>
#include <set>
#include <list>
#include "aoc_utility.hpp"
#include <Eigen/Dense>
#include <cmath>

//Position type
using pos_type = Eigen::Vector2i;

//Node structure
struct Node{
    //node id
    int         id;
    //Node position                                                 
    pos_type    pos;                                            
    //Node position
    int         height = 0;                                         
    //Node height
    double      gscore = std::numeric_limits<double>::infinity();   
    //gScore is the cost of the cheapest path from start to n currently known.
    double      fscore = std::numeric_limits<double>::infinity();   
    //For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    Node*       camefrom = nullptr;        
    //The node we came from using the current best route
    std::vector<Node*> neighbors;
};

//Small function used for comparing Node pointers
bool compareNodes(const Node* a, const Node* b)
{
    return a->fscore < b->fscore;
}

//Map type
using map_type = Eigen::Matrix<Node,-1,-1>;

//Current heuristic: Manhattan distance.
//Problem specific
double heuristic(const Node* node, const pos_type& target){
    return (node->pos - target).cwiseAbs().sum();
}

//Cost of moving from one node to its neighbor
//Problem specific
//Reverse this condition for part 2
double cost(const Node* current, const Node* neighbor){
    if(neighbor->height - current->height <= 1){
        return 1.0;
    }else{
        return 1000;
    }
}

// A* finds a path from start to goal. Relatively reusable implementation
// Translated from pseudocode on https://en.wikipedia.org/wiki/A*_search_algorithm
Node* A_Star(Node* startnode, const pos_type& goal){
    startnode->gscore   = 0;
    startnode->fscore   = heuristic(startnode,goal);

    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    std::list<Node*> openSet;
    openSet.push_back(startnode);

    int nsteps = 0;
    int ninspect = 0;
    
    while(!openSet.empty()){
        // This operation can occur in O(Log(N)) time if openSet is a
        // min-heap or a priority queue
        auto it = std::min_element(openSet.begin(), openSet.end(),compareNodes);
        nsteps ++;
        
        Node* current = *it;
        openSet.erase(it);        

        //We reached the goal, we are done!
        if(current->pos == goal){
        //for part 2: if(current->height == 0){
            std::cout << "A* solution found within " << nsteps << " rounds, inspected " << ninspect << " elements " << std::endl;
            return current;
        }       
        
        //visit all the neighbors of this node
        for(Node* neighbor : current->neighbors){
            double tentative_gScore = current->gscore + cost(current,neighbor);
            // This path to neighbor is better than any previous one. Record it!
            if(tentative_gScore < neighbor->gscore){
                neighbor->camefrom = current;
                neighbor->gscore   = tentative_gScore;
                neighbor->fscore   = tentative_gScore + heuristic(neighbor,goal);

                //Check if this neighbor is not in the queue. If not, add it
                auto result1 = std::find(openSet.begin(),openSet.end(),neighbor);                    
                if(result1 == openSet.end()){
                    openSet.push_back(neighbor);
                    ninspect++;
                }
            }                  
        }        
    }
    throw std::runtime_error("A* failed to find a route");
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

    //Start end and position
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
            //Convert height map to ints     
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
            
            //Create the node
            Node node;
            node.id     = counter;
            node.height = height;
            node.pos[0] = row;
            node.pos[1] = col;
            
            //Add the neighbors this node might have
            if(col>0){           node.neighbors.push_back(&height_map(row,col-1));}
            if(col<N_cols-1){    node.neighbors.push_back(&height_map(row,col+1));}
            if(row>0){           node.neighbors.push_back(&height_map(row-1,col));}
            if(row<N_rows-1){    node.neighbors.push_back(&height_map(row+1,col));}

            height_map(row,col) = node;
            counter++;
            col++;
        }
        row++;
    }

    //Now, do A* search    
    Node* startnode = &height_map(startpos[0],startpos[1]);
    Node* endnode = A_Star(startnode, endpos);
   
    //Reverse the search for part2
    //Node* startnode = &height_map(endpos[0],endpos[1]);
    //Node* endnode = A_Star(startnode, startpos);
    
    //Count the number of steps by backtracing the route
    int n_steps = 0;
    Node* camefrom = endnode->camefrom;
    while(camefrom != nullptr){
        camefrom  = camefrom->camefrom;
        n_steps++;
    }

    std::cout << "route took " << n_steps << " steps" << std::endl;

    return 0;
}