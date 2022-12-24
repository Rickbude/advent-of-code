#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <regex>
#include <cassert>

//Boiling Boulders
int main(){

    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;

    //Store map in a three dimensional array
    const int Lx = 25;
    const int Ly = 25;
    const int Lz = 25;
    bool map[Lx][Ly][Lz] = {false};

    //Input follows pattern: xx,yy,zz
    std::regex line_expr("^(\\d+),(\\d+),(\\d+)$");

    //Read in the data
    while(std::getline(infs,line)){
        std::smatch matches;
        std::regex_match (line,matches,line_expr);
        if(matches.size() != 4){
            throw std::runtime_error("Regex match failed while parsing input!");
        }
        //Extract x,y,z coordinates, and store in the matrix
        int x = std::stoi(matches[1])+1;
        int y = std::stoi(matches[2])+1;
        int z = std::stoi(matches[3])+1;   

        //Check if the matrix size is large enough (only in debug builds),
        //and if so, write to the matrix 
        assert(x > 0 && y > 0 && z > 0);    
        assert(x < Lx-1 && y < Ly-1 && z < Lz-1);
        map[x][y][z] = true;
    }

    //Part 1: count ANY face exposed to air
    int uncovered = 0;
    for(int i = 1; i<Lx-1; i++){
        for(int j = 1; j<Ly-1; j++){
            for(int k = 1; k<Lz-1; k++){
                if(map[i][j][k]){
                    uncovered += !map[i-1][j][k];
                    uncovered += !map[i][j-1][k];
                    uncovered += !map[i][j][k-1];
                    uncovered += !map[i+1][j][k];
                    uncovered += !map[i][j+1][k];
                    uncovered += !map[i][j][k+1];
                }
            }
        }
    }
    std::cout << "Part 1 - total surface area: " << uncovered << std::endl;

    //Part 2
    //Only count the surface area on the outside of the shape

    //Set the edges of the map to "steam"
    bool steam[Lx][Ly][Lz] = {false};
    for(int i = 0; i<Lx; i++){
        for(int j = 0; j<Ly; j++){
            for(int k = 0; k<Lz; k++){
                if( i == 0 || i == Lx-1 ||
                    j == 0 || j == Ly-1 ||
                    k == 0 || k == Lz-1 ){
                        steam[i][j][k] = true;
                }                
            }
        }
    }

    //Do a flood fill from the outside of the map to the inside
    //--> Get every position on the map that can be reached by steam
    //This will take a handful of iterations
    int n_steam = 0;
    int n_steam_prev = n_steam;
    while(1){
        for(int i = 1; i<Lx-1; i++){
            for(int j = 1; j<Ly-1; j++){
                for(int k = 1; k<Lz-1; k++){
                    if(!steam[i][j][k] && !map[i][j][k]){
                        if( steam[i-1][j][k] || steam[i+1][j][k] ||
                            steam[i][j-1][k] || steam[i][j+1][k] ||
                            steam[i][j][k-1] || steam[i][j][k+1]
                        ){
                            steam[i][j][k] = true;
                            n_steam++;
                        }
                    }   
                }
            }
        }
        //Break the loop if no more squares were turned into steam
        if( n_steam == n_steam_prev){
            break;
        }
        n_steam_prev = n_steam;
    }

    //Part 2: count only the faces exposed to the outside ("steam")
    uncovered = 0;
    for(int i = 1; i<Lx-1; i++){
        for(int j = 1; j<Ly-1; j++){
            for(int k = 1; k<Lz-1; k++){
                if(map[i][j][k]){
                    uncovered += steam[i-1][j][k];
                    uncovered += steam[i][j-1][k];
                    uncovered += steam[i][j][k-1];
                    uncovered += steam[i+1][j][k];
                    uncovered += steam[i][j+1][k];
                    uncovered += steam[i][j][k+1];
                }
            }
        }
    }
    std::cout << "Part 2 - outside surface area: " << uncovered << std::endl;

    return 0;
}