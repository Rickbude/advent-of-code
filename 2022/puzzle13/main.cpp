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
#include <cmath>
#include <memory>

struct Node{
    //Would be nice to try out std::vector<std::variant<Node,int>>
    std::vector<Node> nodes;
    int number = -1;
};

void parse(Node* node, std::string in){
    int depth = 0;
    int position = 1;

    if(in.size() == 0){
        return;
    }

    //Find the position of the first bracket
    int startpos = in.find('[');
    int endpos;

    //Find the matching closing bracket
    //std::cout << "parsing: " << in << std::endl;
    for(int pos = startpos; pos < in.size(); pos++){
        char c = in[pos];
        if(c == '['){
            depth++;
        }else if(c == ']'){
            depth--;
            if(depth == 0){
                endpos = pos;
                break; 
            }
        }
    }

    //If [] are encountered, recursively parse the string in 3 parts:
    if(startpos != std::string::npos){
        std::string part1 = in.substr(0,startpos);                    // 1: the part before the closed set
        
        std::string part3 = in.substr(endpos+1);                      
        if(part1.size() > 0){
            parse(node,part1.substr(0,part1.size()-1));
        }

        //part 2: the closed set
        std::string part2 = in.substr(startpos+1,endpos-startpos-1);  
        Node new_node;
        node->nodes.push_back(new_node);
        if(part2.size() > 0){            
            parse(&node->nodes.back(),part2);           
        }

        //part 3: the part after the closed set
        if(part3.size() > 0){
            parse(node,part3.substr(1));
        }
    }else{
        //No brackets found, but we still have work to do!!
        std::stringstream starting_items(in);
        std::string item;
        while(std::getline(starting_items, item, ','))
        {
            Node new_node;
            new_node.number = std::stoi(item);
            node->nodes.push_back(new_node);
        }
    }    
}

//Print the node, primarily to see if it was properly parsed
std::string node_to_string(const Node& node){
    std::string string;
    if(node.nodes.size()>0){
        string += "[";
        for(int i = 0; i< node.nodes.size(); i++){
            string += node_to_string(node.nodes[i]);
        }
        string += "]";
    }else{
        if(node.number >= 0){
            string += " " + std::to_string(node.number) + " ";
        }else{
            string += "[]";
        }        
    }
    return string;
}

//When comparing a list to an item, the number must be wrapped in a node,
//and the comparison must be tried again
Node wrap_number(int number){
    Node parent  ;
    Node daughter;
    daughter.number = number;
    parent.nodes.push_back(daughter);
    return parent;
}

//Return -1 if wrong order, 0 if unknown (so proceed), 1 if right order
int sorted(const Node& node_a, const Node& node_b){    
    //std::cout << "compare " << node_to_string(node_a) << " to " <<  node_to_string(node_b) << std::endl;
    if(node_a.number != -1 && node_b.number != -1){
        //Both nodes are a number --> see if the left number is smaller
        if(node_a.number < node_b.number){
            //std::cout << "Left side is smaller, so inputs are in the right order" << std::endl;
            return 1;
        }
        if(node_b.number < node_a.number){
            //std::cout << "Right side is smaller, so inputs are not in the right order" << std::endl;
            return -1;
        }
        return 0;
    }else if(node_a.number == -1 && node_b.number == -1){
        //Both values are a list
        if(node_a.nodes.size() == 0 && node_b.nodes.size() == 0){
            return 0;
        }

        for(int i = 0; i<node_a.nodes.size(); i++){
            //right list has run out -> not the right order
            if(i+1 > node_b.nodes.size()){
                //std::cout << "Right side ran out of items, so inputs are not in the right order" << std::endl;
                return -1;
            }
            int is_sorted = sorted(node_a.nodes[i],node_b.nodes[i]);
            if(is_sorted != 0){
                return is_sorted;
            }
        }
        //Same sized lists -> continue
        if(node_a.nodes.size() == node_b.nodes.size()){
            return 0;
        }else{
            //std::cout << "Left side ran out of items, so inputs are in the right order" << std::endl;
            return 1;
        }
    }else if(node_a.number == -1){
        //Comparing integer to list -> wrap number in a node and retry
        //std::cout << "Changing int to list and retry" << std::endl;
        Node new_node = wrap_number(node_b.number);
        return sorted(node_a,new_node);
    }else{
        //Comparing integer to list -> wrap number in a node and retry
        //std::cout << "Changing int to list and retry" << std::endl;
        Node new_node = wrap_number(node_a.number);
        return sorted(new_node,node_b);
    }
    std::cout << "wrong comparison????" << std::endl;
}

//Small function that makes sorting possible
bool is_sorted(const Node& node_a, const Node& node_b){
    return sorted(node_a,node_b) == 1;
}

//Distress Signal
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle13 1       or      ./puzzle13 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;
    std::vector<Node> nodes;

    //Read in the data
    while(std::getline(infs,line)){
        //split input at new line
        if(line.size() == 0){
            continue;
        }

        Node new_node;
        parse(&new_node,line.substr(1,line.size()-2));        
        nodes.push_back(new_node);       
    }

    int unsorted_indices_sum = 0;
    for(int i = 0; i<(nodes.size()/2); i++){
        if(is_sorted(nodes[i*2],nodes[i*2+1])){
            unsorted_indices_sum += (i+1);
        }
    }

    std::cout << "part 1 solution: " << unsorted_indices_sum << std::endl;

    //For part 2, insert 2 extra nodes: the "markers"
    Node startnode = wrap_number(2);
    Node endnode   = wrap_number(6);
    nodes.push_back(startnode);
    nodes.push_back(endnode);
    std::sort(nodes.begin(), nodes.end(), is_sorted);

    int startnode_index = -1;
    int endnode_index   = -1;
    for(int i = 0; i<nodes.size(); i++){
        if(sorted(nodes[i],startnode) == 0){
            //start marker located
            startnode_index = i+1;
        }else if(sorted(nodes[i],endnode) == 0){
            //end marker located
            endnode_index   = i+1;
        }
    }
    std::cout << "part 2 solution: " << startnode_index*endnode_index << std::endl;

    return 0;
}