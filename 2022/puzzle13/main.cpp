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
    Node* parent = nullptr;
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
        if(part2.size() > 0){
            Node new_node;
            new_node.parent = node;
            node->nodes.push_back(new_node);
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
            new_node.parent = node;
            new_node.number = std::stoi(item);
            node->nodes.push_back(new_node);
        }
    }    
}

//Print the node, primarily to see if it was properly parsed
std::string node_to_string(const Node* node){
    std::string string;
    if(node->nodes.size()>0){
        string += "[";
        for(int i = 0; i< node->nodes.size(); i++){
            string += node_to_string(&node->nodes[i]);
        }
        string += "]";
    }else{
        if(node->number >= 0){
            string += " " + std::to_string(node->number) + " ";
        }else{
            string += "[]";
        }        
    }
    return string;
}

//Return -1 if wrong order, 0 if unknown (so proceed), 1 if right order, 2 if compare needs to start over
int sorted(Node* node_a, Node* node_b){    
    std::cout << "compare " << node_to_string(node_a) << " to " <<  node_to_string(node_b) << std::endl;
    if(node_a->number != -1 && node_b->number != -1){
        //Both nodes are a number --> see if the left number is smaller
        if(node_a->number < node_b->number){
            std::cout << "Left side is smaller, so inputs are in the right order" << std::endl;
            return 1;
        }
        if(node_b->number < node_a->number){
            std::cout << "Right side is smaller, so inputs are not in the right order" << std::endl;
            return -1;
        }
        return 0;
    }else if(node_a->number == -1 && node_b->number == -1){
        //Both values are a list
        if(node_a->nodes.size() == 0 && node_b->nodes.size() == 0){
            return 0;
        }

        for(int i = 0; i<node_a->nodes.size(); i++){
            //right list has run out -> not the right order
            if(i+1 > node_b->nodes.size()){
                std::cout << "Right side ran out of items, so inputs are not in the right order" << std::endl;
                return -1;
            }
            int is_sorted = sorted(&node_a->nodes[i],&node_b->nodes[i]);
            if(is_sorted != 0){
                return is_sorted;
            }
        }
        //Same sized lists -> continue
        if(node_a->nodes.size() == node_b->nodes.size()){
            return 0;
        }else{
            std::cout << "Left side ran out of items, so inputs are in the right order" << std::endl;
            return 1;
        }
    }else if(node_a->number == -1){
        Node new_node;
        new_node.parent = node_b;
        new_node.number = node_b->number;
        node_b->number = -1;
        node_b->nodes.push_back(new_node);
        std::cout << "Changing int to list and retry" << std::endl;
        return 2;
    }else{
        Node new_node;
        new_node.parent = node_a;
        new_node.number = node_a->number;
        node_a->nodes.push_back(new_node);
        node_a->number = -1;
        std::cout << "Changing int to list and retry" << std::endl;
        return 2;
    }
    std::cout << "wrong comparison????" << std::endl;

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
    int pairnum = 0;

    std::array<Node,2> nodes;

    int setnum = 1;
    int unsorted_indices_sum = 0;
    //Read in the data
    while(std::getline(infs,line))    {
        //split input at new line
        if(line.size() == 0){
            std::cout << "Checking set " << setnum << std::endl;
            while(sorted(&nodes[0],&nodes[1]) == 2){
                continue;
            }
            if(sorted(&nodes[0],&nodes[1]) == 1){                
                unsorted_indices_sum+=setnum;
                std::cout << "list is sorted!!" << std::endl;
            }else{
                std::cout << "list is not sorted" << std::endl;
            }
            pairnum = 0;
            setnum++;
            continue;
        }

        nodes[pairnum].nodes.clear();
        nodes[pairnum].number = -1;

        parse(&nodes[pairnum],line);
        //print(&nodes[pairnum]);

        pairnum++;
        
    }
    std::cout << "Checking set " << setnum << std::endl;
    while(sorted(&nodes[0],&nodes[1]) == 2){
        continue;
    }
    if(sorted(&nodes[0],&nodes[1]) == 1){
        unsorted_indices_sum+=setnum;
        std::cout << "list is sorted!!" << std::endl;
    }else{
        std::cout << "list is not sorted" << std::endl;
    }

    std::cout << "part 1 solution: " << unsorted_indices_sum << std::endl;

    return 0;
}