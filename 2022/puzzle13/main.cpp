#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>
#include <variant>

struct List{
    //Each list element is either a number, or a list
    std::vector<std::variant<int,List>> nodes;
};
using list_or_num = std::variant<int,List>;

//Variant is of integer type?
bool is_number(const list_or_num& node){
    return std::holds_alternative<int>(node);
}

//Variant is a list?
bool is_list(const list_or_num& node){
    return std::holds_alternative<List>(node);
}

//Recursively parse the input string
void parse(List* node, std::string in){
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
            //If string is nonzero, recurse 
            parse(node,part1.substr(0,part1.size()-1));
        }

        //part 2: the closed set
        std::string part2 = in.substr(startpos+1,endpos-startpos-1);     
        List new_node;            
        if(part2.size() > 0){
            //If string is nonzero, recurse 
            parse(&new_node,part2);                       
        }
        node->nodes.push_back(new_node);

        //part 3: the part after the closed set
        if(part3.size() > 0){
            //If string is nonzero, recurse 
            parse(node,part3.substr(1));
        }
    }else{
        //No brackets found, but we still have work to do!!
        std::stringstream starting_items(in);
        std::string item;
        while(std::getline(starting_items, item, ','))
        {
            node->nodes.push_back(std::stoi(item));
        }
    }    
}

//Print the node, primarily to see if it was properly parsed
//(Only used if debug prints are uncommented)
std::string node_to_string(const list_or_num& node){
    std::string string;
    if(is_list(node)){
        //list element is a list -> recurse
        string += "[";
        for(const auto& item : std::get<List>(node).nodes){
            string += node_to_string(item);
        }
        string += "]";        
    }else{
        int number = std::get<int>(node);
        string += " " + std::to_string(number);
    }
    return string;
}

//When comparing a list to an item, the number must be wrapped in a node,
//and the comparison must be tried again
List wrap_number(int number){
    List new_node;
    new_node.nodes.push_back(number);
    return new_node;
}

//Return -1 if wrong order, 0 if unknown (so proceed), 1 if right order
int sorted(const list_or_num& node_a, const list_or_num& node_b){    
    //std::cout << "compare " << node_to_string(node_a) << " to " <<  node_to_string(node_b) << std::endl;
    if(is_number(node_a) && is_number(node_b)){
        int number_a = std::get<int>(node_a);
        int number_b = std::get<int>(node_b);
        //Both nodes are a number --> see if the left number is smaller
        if(number_a < number_b){
            //std::cout << "Left side is smaller, so inputs are in the right order" << std::endl;
            return 1;
        }
        if(number_b < number_a){
            //std::cout << "Right side is smaller, so inputs are not in the right order" << std::endl;
            return -1;
        }
        return 0;
    }else if(is_list(node_a) && is_list(node_b)){
        const List& list_a = std::get<List>(node_a);
        const List& list_b = std::get<List>(node_b);
        //Both values are a list
        if(list_a.nodes.size() == 0 && list_b.nodes.size() == 0){
            return 0;
        }

        for(int i = 0; i<list_a.nodes.size(); i++){
            //right list has run out -> not the right order
            if(i+1 > list_b.nodes.size()){
                //std::cout << "Right side ran out of items, so inputs are not in the right order" << std::endl;
                return -1;
            }
            int is_sorted = sorted(list_a.nodes[i],list_b.nodes[i]);
            if(is_sorted != 0){
                return is_sorted;
            }
        }
        //Same sized lists -> continue
        if(list_a.nodes.size() == list_b.nodes.size()){
            return 0;
        }else{
            //std::cout << "Left side ran out of items, so inputs are in the right order" << std::endl;
            return 1;
        }
    }else if(is_number(node_b)){
        //Comparing integer to list -> wrap number in a node and retry
        //std::cout << "Changing int to list and retry" << std::endl;
        List new_node = wrap_number(std::get<int>(node_b));
        return sorted(node_a,new_node);
    }else{
        //Comparing integer to list -> wrap number in a node and retry
        //std::cout << "Changing int to list and retry" << std::endl;
        List new_node = wrap_number(std::get<int>(node_a));
        return sorted(new_node,node_b);
    }
}

//Small function that makes sorting possible
bool is_sorted(const List& node_a, const List& node_b){
    return sorted(node_a,node_b) == 1;
}

bool is_equal(const List& node_a, const List& node_b){
    return sorted(node_a,node_b) == 0;
}

//Distress Signal
int main(){
    
    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;
    std::vector<List> nodes;

    //Read in the data
    while(std::getline(infs,line)){
        //split input at new line
        if(line.size() == 0){
            continue;
        }

        List new_node;
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
    List startnode = wrap_number(2);
    List endnode   = wrap_number(6);
    nodes.push_back(startnode);
    nodes.push_back(endnode);
    std::sort(nodes.begin(), nodes.end(), is_sorted);

    //Search for the start and end markers
    int startnode_index = -1;
    int endnode_index   = -1;
    for(int i = 0; i<nodes.size(); i++){
        if(is_equal(nodes[i],startnode)){
            //start marker located
            startnode_index = i+1;
        }else if(is_equal(nodes[i],endnode)){
            //end marker located
            endnode_index   = i+1;
        }
    }
    std::cout << "part 2 solution: " << startnode_index*endnode_index << std::endl;

    return 0;
}