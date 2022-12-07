#include <iostream>
#include <string>
#include <fstream>
#include <algorithm>
#include <vector>
#include <unordered_map>

//Take the current working directory, stored as a vector of strings,
//and create a single string from it, separated with "/"
std::string cwd_to_string(const std::vector<std::string>& cwd){
    std::string cwd_str = "/";
    for(std::string dir : cwd){
        cwd_str = cwd_str + dir + "/";
    }
    return cwd_str;
}

//Examine an elf filesystem
int main(){

    //Current working directory, stored as vector of strings, so we can 
    //push and pop to it while cd'ing
    std::vector<std::string> cwd;

    //A map that stores per (nested) directory the total folder size
    std::unordered_map<std::string,int> dir_sizes;
    
    //Load input file (this file is copied to the build directory) and read its only line
    std::fstream infs("input.txt");    
    std::string line;
    int total_file_size = 0;
    while(std::getline(infs,line)){
        if(line[0] == '$'){
            std::string command = line.substr(2,2);
            //Command encountered
            if(command == "cd"){
                std::string dirname = line.substr(5);               
                if(dirname == ".."){
                    //Go back a directory --> pop
                    cwd.pop_back();
                }else if(dirname != "/"){
                    //Dive into a directory --> push
                    //Also set the size for the new subdirectory to 0
                    cwd.push_back(dirname);
                    if(cwd.size() > 0){
                        dir_sizes[cwd_to_string(cwd)] = 0;
                    }
                }        
            }
        }else{
            //Output
            
            //Split the input line at the ' '
            size_t delim_pos = line.find(' ');
            std::string part1 = line.substr(0,delim_pos);
            std::string name  = line.substr(delim_pos+1);

            if(part1 != "dir"){
                //File size found
                int filesize = std::stoi(part1);

                //Add to the total filesystem size
                total_file_size += filesize;

                //Add the filesize to the current working directory,
                //and propagate the filesize to all parent directories
                auto cwd_copy = cwd;
                while(cwd_copy.size() > 0){
                    dir_sizes[cwd_to_string(cwd_copy)] += filesize;
                    cwd_copy.pop_back();
                }            
            }
        }    
    }   


    //Copy of the values from the hashmap into a vector for part 2
    std::vector<int> dir_sizes_vector;

    //Find the sum of sizes of all folders smaller than 100kB
    int answer_part_1 = 0;
    for(const auto& dir_size_pair : dir_sizes){
        int dir_size = dir_size_pair.second;
        dir_sizes_vector.push_back(dir_size);
        if(dir_size <= 100000){            
            answer_part_1 += dir_size;
        }
    }
    std::cout << "Sum of directories smaller than 100kB: " << answer_part_1 << std::endl;

    //Calculate the space we need to save
    int fs_size         = 70000000;
    int required_space  = 30000000;
    int required_save   = total_file_size - (fs_size-required_space);
    std::cout << "Total used space by all files: " << total_file_size << std::endl;
    std::cout << "Need to save: " << required_save << " bytes" << std::endl;

    //Find the smallest directory that can achieve this save
    std::sort(dir_sizes_vector.begin(),dir_sizes_vector.end());
    auto it =  std::upper_bound(dir_sizes_vector.begin(),dir_sizes_vector.end(),required_save);
    std::cout << "Smallest folder to remove: " << *it << std::endl;
   
    return 0;
}