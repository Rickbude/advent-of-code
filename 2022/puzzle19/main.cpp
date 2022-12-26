#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <regex>
#include <cassert>
#include <Eigen/Dense>
#include "aoc_utility.hpp"

enum ResourceType{
    ore = 0,
    clay,
    obsidian,
    geode
};

struct Bot{
    Eigen::Vector4i cost = {0,0,0,0};
    ResourceType type;
};

struct Blueprint{
    int id;
    Eigen::Vector4i max_cost = {0,0,0,0};
    std::array<Bot,4> bots;
};

//Simple function that returns whether we can buy a bot of a certain type, or not.
bool can_buy(const Blueprint& blueprint, const Eigen::Vector4i& items, ResourceType type){
    return (items - blueprint.bots[type].cost).minCoeff() >= 0;
}

//Recursive function to calculate the maximum number of geodes mined, within N minutes
int geodes_collected(const Blueprint& blueprint, const int minutes_left, Eigen::Vector4i items, Eigen::Vector4i bots){

    //We are done. Return the quality level
    if(minutes_left == 0){
        return items[geode];
    }

    //Determine which bots to build, by determining which bots cn actively
    //Contribute to the final geode count.
    // - In the last minute, don't build bots at all
    // - In the one but last minute, only build geode bots
    // - In the second but last minute, only build geode bots and obsidian bots
    std::array<bool,4> build_bots;
    build_bots[ore     ] = (minutes_left > 4) ? true : false;
    build_bots[clay    ] = (minutes_left > 3) ? true : false;
    build_bots[obsidian] = (minutes_left > 2) ? true : false;
    build_bots[geode   ] = (minutes_left > 1) ? true : false;
    
    //It is (probably?) always smart to buy a geode bot whenever possible.
    if(minutes_left>1 && can_buy(blueprint,items,geode)){
        items = items - blueprint.bots[geode].cost + bots;
        bots[geode]++;
        return geodes_collected(blueprint,minutes_left-1,items,bots);
    }
    
    //Only one robot can be built per minute. The worst score we can get (probably)
    //is achieved by simply not doing anything at all
    int max_n_geodes = items[geode] + minutes_left*bots[geode];  
    for(ResourceType extra_bot = ore; extra_bot <= geode; extra_bot = ResourceType(extra_bot+1)){

        //Only consider building this bot if sufficient minutes are left
        if(!build_bots[extra_bot]){
            continue;
        }

        //We can only produce bot N after at least one of bot N-1 has been made
        //E.g. without a clay bot, we can't make an obsidian bot
        if(extra_bot > ore && bots[extra_bot-1] == 0){
            continue;
        }

        //Don't spam bots. Only 1 bot can be built per minute. Therefore, it
        //never makes sense to have more bots of certain resource, than the
        //maximum amount of resources that can be spent on a single bot
        if(extra_bot < geode && bots[extra_bot] >= blueprint.max_cost[extra_bot]){
            continue;
        }
        
        //Calculate the number of time steps that we need to wait before we can afford this bot.
        //Only proceed if it makes sense to build (similar to earlier in this function)
        Eigen::Vector4i new_items = items;
        int new_minutes_left = minutes_left;
        int max_minutes = 4 - extra_bot;
        while(!can_buy(blueprint,new_items,extra_bot) && new_minutes_left>max_minutes){
            new_items+=bots;
            new_minutes_left--;
        }

        int n_geodes;
        if(new_minutes_left == max_minutes){
            //Turns out we can not afford this bot in time with our current collection of bots       
            new_items += max_minutes*bots;     
            n_geodes = new_items[geode];
        }else{
            //We can afford this bot in time, so buy it
            new_items = new_items - blueprint.bots[extra_bot].cost + bots;
            Eigen::Vector4i new_bots = bots;
            new_bots[extra_bot]++;
            n_geodes = geodes_collected(blueprint,new_minutes_left-1,new_items,new_bots);            
        }        
        max_n_geodes = std::max(max_n_geodes,n_geodes);                     
    }
    return max_n_geodes;
}

//Not Enough Minerals
//This script needs as input whether it needs to run part1, or part 2.
// Run as: ./puzzle19 1       or      ./puzzle19 2
//For part 1 and 2 respectively
int main(int argc, char *argv[]){

    //Parse the program arguments, extract the part number
    int part = aoc::get_part_number(argc,argv);

    //Load input file (this file is copied to the build directory) and obtain
    //the number of rows and columns of the map
    std::fstream infs("input.txt");    
    
    std::string line;

    //Input follows pattern: xx,yy,zz
    std::regex line_expr("^Blueprint (\\d+): Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.$");

    std::vector<Blueprint> blueprints;
    
    //Read in the data
    while(std::getline(infs,line)){
        std::smatch matches;
        std::regex_match (line,matches,line_expr);
        if(matches.size() != 8){
            throw std::runtime_error("Regex match failed while parsing input!");
        }

        //Store blueprint data
        Blueprint blueprint;
        blueprint.id = std::stoi(matches[1]);
        blueprint.bots[ore     ].cost[ore     ] = std::stoi(matches[2]);
        blueprint.bots[clay    ].cost[ore     ] = std::stoi(matches[3]);
        blueprint.bots[obsidian].cost[ore     ] = std::stoi(matches[4]);
        blueprint.bots[obsidian].cost[clay    ] = std::stoi(matches[5]);
        blueprint.bots[geode   ].cost[ore     ] = std::stoi(matches[6]);
        blueprint.bots[geode   ].cost[obsidian] = std::stoi(matches[7]);
        
        //Determine the maximum amount of resources any bot in this template costs
        for(int i = 0; i<4; i++){
            blueprint.max_cost = (blueprint.max_cost).cwiseMax(blueprint.bots[i].cost);
        }

        blueprints.push_back(blueprint);
    }

    //Maximum minutes and the number of blueprints to consider differs between part 1 and 2
    int max_minutes  = (part == 1) ? 24 : 32;
    int n_blueprints = (part == 2) ? 3  : blueprints.size();
    std::vector<int> geodes(n_blueprints);

    //openMP gives a decent speedup (especially in debug builds)
    #pragma omp parallel for
    for(int i = 0; i<n_blueprints; i++){
        const Blueprint& blueprint = blueprints[i];
        //Start out with exactly one ore bot
        Eigen::Vector4i items = {0,0,0,0};
        Eigen::Vector4i bots  = {1,0,0,0};
        geodes[i] = geodes_collected(blueprint,max_minutes,items,bots);        
    }

    if(part == 1){
        //For part 1, calculate the sum of "quality factors" of all blueprints
        int quality_sum = 0;
        for(int i = 0; i<n_blueprints; i++){
            int quality_score = blueprints[i].id * geodes[i];
            std::cout << "Blueprint " << blueprints[i].id << " has quality score : " << quality_score << std::endl;
            quality_sum += quality_score;
        }
        std::cout << "Sum of blueprint qualities: " << quality_sum << std::endl;
    }else{
        //For part 2, calculate the product of geodes collected by only the first 3 blueprints
        int geode_product = 1;
        for(int i = 0; i<n_blueprints; i++){
            geode_product *= geodes[i];
            std::cout << "Maximum geodes collected with blueprint " << blueprints[i].id << " : " << geodes[i] << std::endl;
        }
        std::cout << "Product of collected geodes: " << geode_product << std::endl;
    }
    
    return 0;
}