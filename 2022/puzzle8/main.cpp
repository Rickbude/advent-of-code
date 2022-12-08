#include <iostream>
#include <string>
#include <fstream>
#include <vector>


//Examine an elf filesystem
int main(){
    
    //Load input file (this file is copied to the build directory) and read its only line
    std::fstream infs("input.txt");    
    std::string line;
    
    //Load the trees into a matrix (vector of vectors)
    std::vector<std::vector<int>> trees;
    while(std::getline(infs,line)){
        std::vector<int> tree_row(line.size());
        for(int i =0; i<line.size(); i++){
            tree_row[i] = line[i];
        }
        trees.push_back(tree_row);
    }   

    //Now determine for every tree whether it is obstructed, and calculate the scenic score.
    int n_not_obstructed = 0;
    int max_scenic_score = 0;
    for(int row = 0; row<trees.size(); row++){
        for(int column = 0; column<trees[row].size(); column++){
            int N_rows = trees.size();
            int N_cols = trees[row].size();

            int tree_height = trees[row][column];
            
            bool obstructed_north = false;
            int scenic_score_north = 0;
            //check not obstructed in the north direction
            for(int i = row-1; i>=0; i--){                
                scenic_score_north++;
                if(tree_height <= trees[i][column]){
                    obstructed_north = true;
                    break;
                }
            }

            bool obstructed_south = false;
            int scenic_score_south = 0;
            //check not obstructed in the south direction
            for(int i = row+1; i<N_rows; i++){                
                scenic_score_south++;
                if(tree_height <= trees[i][column]){
                    obstructed_south = true;
                    break;
                }
            }

            bool obstructed_east = false;
            int scenic_score_east = 0;
            //check not obstructed in the east direction
            for(int i = column-1; i>=0; i--){                
                scenic_score_east++;
                if(tree_height <= trees[row][i]){
                    obstructed_east = true;
                    break;
                }
            }

            bool obstructed_west = false;
            int scenic_score_west = 0;
            //check not obstructed in the west direction
            for(int i = column+1; i<N_cols; i++){
                scenic_score_west++;
                if(tree_height <= trees[row][i]){
                    obstructed_west = true;
                    break;
                }                
            }

            //Calculate the "scenic score" for this tree
            int scenic_score = scenic_score_north*scenic_score_south*scenic_score_east*scenic_score_west;
            if(scenic_score > max_scenic_score){
                max_scenic_score = scenic_score;
            }

            //Determine if this tree is obstructed
            n_not_obstructed += (!obstructed_north || !obstructed_south || !obstructed_east || !obstructed_west);

        }
    }

    std::cout << "The number of unobstructed trees: " << n_not_obstructed << std::endl;
    std::cout << "The maximum scenic score is: " << max_scenic_score << std::endl;

    return 0;
}