#include <iostream>
#include <string>
#include <fstream>
#include <sstream>
#include <vector>
#include <algorithm>
#include <regex>
#include "aoc_utility.hpp"

struct point{
    int x = 0;
    int y = 0;
};

int manhattan_distance(const point& a, const point& b){
    return std::abs(a.x-b.x) + std::abs(a.y-b.y);
}

//Beacon Exclusion Zone
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle14 1       or      ./puzzle14 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);
    
    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;

    //INput is provided in this form
    std::regex line_expr("^Sensor at x=(-?\\d+), y=(-?\\d+): closest beacon is at x=(-?\\d+), y=(-?\\d+)$");

    std::vector<point> sensors; // Sensor positions
    std::vector<point> beacons; // Beacon closest to sensor
    std::vector<int>   radius;  // Radius around sensor containing no other beacon
    
    //Read in the data
    while(std::getline(infs,line)){
        //Parse input line
        std::smatch matches;
        std::regex_match (line,matches,line_expr);
        if(matches.size() != 5){
            throw std::runtime_error("Regex match failed while parsing input!");
        }
        point beacon;
        point sensor;
        sensor.x = stoi(matches[1]);    //Number of crates to be moved
        sensor.y = stoi(matches[2]);    //Source stack
        beacon.x = stoi(matches[3]);    //Target stack
        beacon.y = stoi(matches[4]);

        //Store the sensor and beacon positions
        beacons.push_back(beacon);
        sensors.push_back(sensor);
        radius.push_back(manhattan_distance(sensor,beacon));
    }

    //Establish the boundaries of the "search box"
    int min_row = (part == 1) ? 2000000 : 0;
    int max_row = (part == 1) ? 2000001 : 4000000;
    int min_col = (part == 2) ? min_row : std::numeric_limits<int>::min();
    int max_col = (part == 2) ? max_row : std::numeric_limits<int>::max();

    for(int row = min_row; row<max_row; row++){
        point test_point;
        test_point.y = row;
        int n_blocked = 0;

        std::vector<std::pair<int,int>> ranges;
        
        for(int i = 0; i<sensors.size(); i++){

            //Vertical projection of a sensor on the desired row
            point projection;
            projection.x = sensors[i].x;
            projection.y = row;

            //Check if this projection is in range of the sensor
            int dx = radius[i] - manhattan_distance(projection,sensors[i]);
            if(dx < 0){
                continue;
            }
            
            std::pair<int,int> range {
                std::max(min_col,projection.x-dx),
                std::min(max_col,projection.x+dx)
            };
            ranges.push_back(range);
        }

        //Sort the ranges in increasing start position
        std::sort(ranges.begin(), ranges.end());

        //Merge the ranges that overlap / are adjacent
        //(possible optimization: work within the same vector)
        std::vector<std::pair<int,int>> merged_ranges;
        std::pair<int,int> prev_range = *ranges.begin();
        for(const auto& range : ranges){
            if(range.first <= (prev_range.second+1)){
                if(range.second > prev_range.second){
                    prev_range.second = range.second;
                }            
            }else{
                //New disjoint range found
                merged_ranges.push_back(prev_range);
                prev_range = range;
            }
        }
        merged_ranges.push_back(prev_range);

        //Determine the total amount of blocked-off positions in this row
        for(const auto& range : merged_ranges){
            n_blocked += (range.second - range.first);
        }    

        if(part == 1){
            std::cout << "blocked positions on line "<< row << " (= part 1 answer) : " << n_blocked << std::endl;
        }else{
            if(n_blocked != max_col-min_col){
                //We know there is exactly one position, in between the first and second range..
                //This is of course not rigorous, but okay..
                //longs are needed, because te values overflow otherwise
                long col = merged_ranges[0].second + 1;
                std::cout << "Beacon is on (" << row << ","<< col << "). Part 2 answer: " 
                          << (long(col)*long(4000000)+long(row)) << std::endl;
            }
        }
        
    }
    
    return 0;
}