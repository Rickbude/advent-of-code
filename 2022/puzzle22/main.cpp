#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <algorithm>
#include <unordered_map>
#include <set>
#include "aoc_utility.hpp"
#include <Eigen/Core>
#include <cassert>

enum Facing{
    right=0,
    down=1,
    left=2,
    up=3
};

struct Node{
    int row;
    int col;
    int x=0;
    int y=0;
    int z=0;
    std::array<int,4> neighbours = {-1,-1,-1,-1};
    char tile;
    int face;
    std::array<int,4> new_facing = {0,0,0,0};
};

struct Face{    
    int row;
    int col;
    int min_row;
    int max_row;
    int min_col;
    int max_col;
    double center_x;
    double center_y;
    int id;
    bool initialized = false;
    std::array<int,4> neighbours = {-1,-1,-1,-1};   //neighbors of this face
    std::array<int,4> new_facing = {0,0,0,0};       //new facing after transitioning to neighbor
    Eigen::Vector3i east   = {1,0,0}; // +x
    Eigen::Vector3i north  = {0,1,0}; // +y
    Eigen::Vector3i normal = {0,0,1}; // +z        
};

//Simple "hashing function" for (row,col) to id. 
int row_col_to_id(int row, int col, int base = 10000){
    return base*row + col;
}

//Rotation matrix for rotation around z-axis
Eigen::Matrix3d Rz(double theta_deg){
    double theta = theta_deg/180*M_PI;
    Eigen::Matrix3d R;
    R << cos(theta) , -sin(theta)   ,  0  ,
         sin(theta) ,  cos(theta)   ,  0  ,
            0       ,       0       ,  1  ; 
    return R;
}

//Try to take a step in a given direction (facing). Return false if the step
//would hit a wall ('#'), true otherwise.
bool try_move(std::unordered_map<int,Node>& map, int& row, int& col, Facing& facing){
    int id = row_col_to_id(row,col);
    int neighbour_id = map.at(id).neighbours[facing];
    const Node& neighbour = map.at(map.at(id).neighbours[facing]);
    if(neighbour.tile != '#'){
        row     = neighbour.row;
        col     = neighbour.col;
        facing  = Facing(map.at(id).new_facing[facing]);
        return true;
    }else{
        return false;
    }
}

//Print the map (for debugging purposes)
void print_map(std::unordered_map<int,Node>& map, int Nrows, int Ncols){
    for(int row = 0; row<Nrows; row++){
        std::string line;
        for(int col = 0; col<Ncols; col++){
            int id = row_col_to_id(row,col);
            if(map.find(id) != map.end()){
                line += map.at(id).tile;
            }else{
                line += ' ';
            }
        }
        std::cout << line << std::endl;
    }
}

//Monkey Map
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle22 1       or      ./puzzle22 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);

    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    std::string line;

    //First part of the input file contains the map
    std::unordered_map<int,Node> map;
    int row = 0;
    Node node;
    const int start_row =  0; //Starting row
    int start_col = -1; //Starting column
    while(std::getline(infs,line)){
        //Empty line marks the end of the map and start of instructions
        if(line == ""){
            break;
        }
        for(int col = 0; col<line.size(); col++){
            char c = line[col];
            //Skip padding at the start of lines
            if(c == ' '){
                continue;
            }
            //Establish starting column
            if(start_col == -1){
                start_col = col;
            }
            node.row  = row;
            node.col  = col;
            node.tile = c;
            int id    = row_col_to_id(row,col);   
            map[id]   = node; 
        }
        row++;
    }

    //The nature of the problem dictates that a total of exactly
    // [6 x face_size x face_size] map entries are present.
    const int face_size = std::sqrt(map.size()/6);
    assert(map.size() == 6*face_size*face_size);
    std::cout << "face size is " << face_size << " x " << face_size << std::endl;

    //Part 2 of the input file contains the instructions   
    std::vector<std::pair<int,int>> instructions; 
    std::getline(infs,line);
    std::string digit = "";
    std::pair<int,int> direction;
    for(char c : line){
        if(c >= '0' && c <= '9'){
            digit += c;
        }else{
            switch(c){
                case 'R':
                    direction = { 1,std::stoi(digit)};
                    break;
                case 'L':
                    direction = {-1,std::stoi(digit)};
                    break;
                default:
                    throw std::runtime_error("Illegal direction encountered while parsing input");
            }   
            digit = "";
            instructions.push_back(direction);
        }                
    }
    //Make sure the final instruction is also captured.
    direction = {0,std::stoi(digit)};
    instructions.push_back(direction);

    //Make a "reduced map" that contains only the position of faces   
    //Also establish the starting face, and the matrix dimensions
    std::unordered_map<int,Face> cube_faces; 
    Face start_face; 
    Face face;
    int Nrows = 0;
    int Ncols = 0;
    for(auto& node : map){
        int row = node.second.row;
        int col = node.second.col;
        Nrows   = std::max(Nrows,row+1);  
        Ncols   = std::max(Ncols,col+1); 
        int face_row = row/face_size;
        int face_col = col/face_size;
        if(row%face_size == 0 && col%face_size == 0){
            int faceid    = row_col_to_id(face_row,face_col,10);
            face.row      = face_row;
            face.col      = face_col;  
            face.min_row  = face_size*face_row;
            face.max_row  = face_size*(face_row+1)-1; 
            face.min_col  = face_size*face_col;
            face.max_col  = face_size*(face_col+1)-1; 
            face.center_x = (face.min_col + face.max_col)/2.0;
            face.center_y = (face.min_row + face.max_row)/2.0;
            face.id       = faceid;
            cube_faces[faceid] = face;
            //Check if this face contains the starting point
            if( face_row == start_row / face_size && 
                face_col == start_col / face_size){
                start_face   = face;
            }
        }        
        node.second.face = row_col_to_id(face_row,face_col,10);
    }   

    Eigen::Vector3i x{1,0,0};
    Eigen::Vector3i y{0,1,0};
    Eigen::Vector3i z{0,0,1};

    //Assume the start face points up
    Eigen::Vector3i normal { 0,0,1};
    int start_face_id = row_col_to_id(start_face.row,start_face.col,10);
    cube_faces.at(start_face.id).initialized = true;

    //Set the face normals for the other faces in an iterative manner
    for(int i = 0; i<6; i++){
        if(part == 1){
            continue;
        }
        for(const auto& face : cube_faces ){
            //Only consider uninitialized faces
            if(!face.second.initialized){
                continue;
            }
            int row = face.second.row;
            int col = face.second.col;
            //Set up/down neighbours
            for(int d_row : {-1,1}){
                const auto& neighbour  = cube_faces.find(row_col_to_id(row+d_row,col,10));
                if(neighbour == cube_faces.end() || neighbour->second.initialized){
                    continue;
                }
                neighbour->second.normal = -d_row*face.second.north; 
                neighbour->second.east   =        face.second.east;  
                neighbour->second.north  =  d_row*face.second.normal;
                neighbour->second.initialized = true;
            }
            //Set left/right neighbours
            for(int d_col : {-1,1}){
                const auto& neighbour  = cube_faces.find(row_col_to_id(row,col+d_col,10));
                if(neighbour == cube_faces.end() || neighbour->second.initialized){
                    continue;
                }
                neighbour->second.normal =  d_col*face.second.east;
                neighbour->second.east   = -d_col*face.second.normal;
                neighbour->second.north  =        face.second.north;
                neighbour->second.initialized = true;
            }  
        }
    }

    //Print the reduced map as a check
    std::cout << "Reduced cube map:" << std::endl;
    for(int i = 0; i<4; i++){
        for(int j = 0; j<4; j++){
            int neighbour_id = row_col_to_id(i,j,10);
            const auto& face = cube_faces.find(neighbour_id);
            if(face == cube_faces.end()){
                std::cout << ".";
                continue;
            }
            if(face->second.normal ==  z){
                std::cout << "T"; continue;
            }
            if(face->second.normal == -z){
                std::cout << "B"; continue;
            }
            if(face->second.normal == x){
                std::cout << "E"; continue;
            }
            if(face->second.normal == -x){
                std::cout << "W"; continue;
            }
            if(face->second.normal == y){
                std::cout << "N"; continue;
            }
            if(face->second.normal == -y){
                std::cout << "S"; continue;
            }
        }
        std::cout << std::endl;
    }

    //Complete neighbour connectivity graph
    for(auto& face : cube_faces ){
        if(part == 1){
            //For part 1, connect the faces based on "loopback" 
            for(int facing = right; facing<=up; facing = Facing(facing+1)){               
                int d_row   = (facing%2 == 1) ? 2 - facing : 0;
                int d_col   = (facing%2 == 0) ? 1 - facing : 0;    
                int new_row = face.second.row;
                int new_col = face.second.col;
                int neighbour_id;             
                do{                
                    new_row = aoc::mod(new_row+d_row,4);
                    new_col = aoc::mod(new_col+d_col,4);
                    neighbour_id = row_col_to_id(new_row,new_col,10);
                }while(cube_faces.find(neighbour_id) == cube_faces.end());
                face.second.neighbours[facing] = neighbour_id;
            }
            face.second.initialized = true;            
        }else{
            //For part 2, connect the faces based on the cube
            for(const auto& neighbour : cube_faces){
                const Eigen::Vector3i& normal = neighbour.second.normal;
                int neighbour_id = neighbour.first;
                if(normal ==  face.second.east){
                    face.second.neighbours[right] = neighbour_id; continue;
                }
                if(normal == -face.second.north){
                    face.second.neighbours[down]  = neighbour_id; continue;
                }
                if(normal == -face.second.east){
                    face.second.neighbours[left]  = neighbour_id; continue;
                }
                if(normal ==  face.second.north){
                    face.second.neighbours[up]    = neighbour_id; continue;
                }
            }  
        }
    }

    //After we figured out the connectivity, figure out the orientation change
    //on face transitions
    for(auto& face : cube_faces ){
        for(int facing = right; facing<=up; facing = Facing(facing+1)){
            if(part == 1){                      
                //In part 1 the facing does not change on a face transition   
                face.second.new_facing[facing] = facing;        
            }else{
                //In part 2 the new facing can be determined via the neighbor.
                //Check in the "arriving face" where we came from. This is 180
                //degrees rotated compared to the direction we "need".
                const auto& neighbour_id = face.second.neighbours[facing];
                const Face& neighbour    = cube_faces.at(neighbour_id);
                for(int neighbor_facing = right; neighbor_facing<=up; neighbor_facing = Facing(neighbor_facing+1)){
                    if(neighbour.neighbours[neighbor_facing] == face.second.id){    
                        face.second.new_facing[facing] = aoc::mod(neighbor_facing+2,4);
                        break;
                    }            
                }   
            }
        }
    }

    std::cout << "Neighbour faces:" << std::endl;
    const std::string signs = ">v<^";
    for(const auto& face : cube_faces ){
        std::cout << "face " <<face.second.id << "  at (" << face.second.row << "," << face.second.col << "): ";
        for(int facing = right; facing<=up; facing = Facing(facing+1)){
             std::cout << signs[facing] << ":" << face.second.neighbours[facing];
             std::cout << "("<< signs[face.second.new_facing[facing]] << ")"<< " ";
        }
        std::cout << std::endl;
    } 

    //We should have detected 6 unique cube faces
    assert(cube_faces.size() == 6);

    //Determine for each node its neighbours. This process is the main difference between part 1 and 2
    for(auto& node : map){
        int row = node.second.row;
        int col = node.second.col;        
        int curr_face_id = node.second.face;
        const Face& curr_face = cube_faces.at(curr_face_id);
        int facing_change   = 0;
        for(int facing = right; facing<=up; facing = Facing(facing+1)){
            node.second.new_facing[facing] = facing;
            
            int d_row = (facing%2 == 1) ? 2 - facing : 0;
            int d_col = (facing%2 == 0) ? 1 - facing : 0;     

            int new_row = row + d_row;
            int new_col = col + d_col;  

            //We only need to do something special on a face change
            if(new_row < curr_face.min_row || new_row > curr_face.max_row 
            || new_col < curr_face.min_col || new_col > curr_face.max_col 
            ){             
                //We got into a new face, determine the new face and facing.   
                int new_face_id       = curr_face.neighbours[facing];
                const Face& new_face  = cube_faces.at(new_face_id);
                node.second.new_facing[facing] = curr_face.new_facing[facing];

                //Check in the "arriving face" where we came from. This is 180
                //degrees rotated compared to the direction we "need". For part
                //1, no rotation is needed.
                int new_facing = curr_face.new_facing[facing];
                
                //Calculate the change in facing
                facing_change = aoc::mod(new_facing-facing,4); 

                //Shift the tentative new position, the "would-be" new face, to
                //the origin
                double px = new_col - curr_face.center_x;                
                double py = new_row - curr_face.center_y;  
                Eigen::Vector3d pos = {px,py,0};              

                //Rotate the vector around the z-axis to match the new orientation                
                pos = Rz(facing_change*90)*(pos);

                //Shift the face back, such that translated face touches the new face.
                //The new position should fall inside the new face.           
                int d_face_row = (new_facing%2 == 1) ? 2 - new_facing : 0;
                int d_face_col = (new_facing%2 == 0) ? 1 - new_facing : 0;
                new_row = std::round(new_face.center_y + pos[1] - d_face_row*face_size);
                new_col = std::round(new_face.center_x + pos[0] - d_face_col*face_size);                  
            }
            node.second.neighbours[facing]    = row_col_to_id(new_row,new_col);
        }          
    }
    
    std::cout << "done setting up connectivity" << std::endl;
    

    //Step through the instructions one at a time
    Facing facing = right;
    row = start_row;
    int col = start_col;
    for(const auto& instruction : instructions){

        int steps     = instruction.second;
        int new_row   = row;
        int new_col   = col;       
        
        //First make N steps
        for(int step = 0; step<steps; step++){
            //Mark our steps in the map  
            const std::string signs = ">v<^";
            map.at(row_col_to_id(row,col)).tile = signs[facing];
            bool did_move = try_move(map,new_row,new_col,facing);
            if(did_move){                
                row = new_row;
                col = new_col;                
            }else{
                break;
            }
        }

        //Mark our steps in the map  
        const std::string signs = ">v<^";
        map.at(row_col_to_id(new_row,new_col)).tile = signs[facing];

        //Change facing in place
        facing = Facing(aoc::mod(facing+instruction.first,4));    
    }

    //Print the final map
    //print_map(map,Nrows,Ncols);    

    std::cout << "Password: " << std::to_string((row+1)*1000 + (col+1)*4 + facing) << std::endl;

    return 0;
}
