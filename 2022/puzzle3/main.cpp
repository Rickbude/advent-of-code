#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>
#include <string>

using rucksack = std::vector<int>;

//Translate from character to item priority:
//  a-z ->  1 - 26
//  A-Z -> 27 - 52
int priority(char item){
    if(islower(item)){
        return item - 'a' + 1;
    }else if(isupper(item)){
        return item - 'A' + 27;
    }else{
        throw std::runtime_error("illegal item in rucksack (not a-z A-Z)");
    }
}

//Fill the rucksack with items from string. Clear first and sort contents after
//(needed for the set operations we will be using)
rucksack fill_rucksack(const std::string& items){
    rucksack new_rucksack(items.size());
    for(int i = 0; i<items.length(); i++){
        new_rucksack[i] = priority(items[i]);
    }
    std::sort(new_rucksack.begin(),new_rucksack.end());
    return new_rucksack;
}

//Check which items are duplicate between two rucksacks (or compartments).
//This requires that the rucksacks are already sorted. A new rucksack 
//will be returned, only containing the duplicate item(s)
rucksack duplicate_items(const rucksack& rucksack1, const rucksack& rucksack2){
        
    //Check if rucksacks are sorted (needed for set_intersection)
    if(!std::is_sorted(rucksack1.begin(),rucksack1.end())){
        throw std::runtime_error("rucksack1 is not sorted!");
    }
    if(!std::is_sorted(rucksack2.begin(),rucksack2.end())){
        throw std::runtime_error("rucksack2 is not sorted!");
    }

    //Calculate which elements are present in both lists (sets)
    //C++ can do this with the set_intersection algorithm
    rucksack intersection;
    std::set_intersection(  rucksack1.begin(),rucksack1.end(),
                            rucksack2.begin(),rucksack2.end(),
                            std::back_inserter(intersection));

    //Remove potential duplicates from intersection
    //(example:  AAAbcd  frAAg -> intersection would be AA)
    std::sort(intersection.begin(), intersection.end());
    auto last = std::unique(intersection.begin(), intersection.end());
    intersection.erase(last, intersection.end());
    
    return intersection;
}


int main(){

    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");    
    
    // --------------------------- part 1 --------------------------------

    //Pre-declare rucksacks
    rucksack items_comp1;
    rucksack items_comp2;

    std::string line;
    int duplicate_item_sum = 0;
    while(std::getline(infs,line)){
        //Total number of items in each compartment. Should always be even
        int N_items = line.size();
        if(N_items % 2 != 0){
            throw std::runtime_error("Number of items in rucksack not even!");
        }

        //Divide the items over the two compartments:
        // first half of items in compartment 1, rest in compartment 2
        items_comp1 = fill_rucksack(line.substr(0, N_items/2));
        items_comp2 = fill_rucksack(line.substr(   N_items/2));

        //Calculate which elements are present in both compartments
        rucksack intersection = duplicate_items(items_comp1,items_comp2);
        
        //Check the elf's bad work
        if(intersection.size() != 1){
            throw std::runtime_error("Elf packed more than 1 duplicate item!: " + line);
        }

        //Add to the sum
        duplicate_item_sum += intersection[0];
    }
    
    std::cout << "Sum of duplicate items (part 1):" << duplicate_item_sum << std::endl;

    // --------------------------- part 2 --------------------------------
    
    //Reset input filestream
    infs.clear();
    infs.seekg(0);

    //Pre-declare rucksacks
    rucksack rucksack1;     //items in rucksack 1
    rucksack rucksack2;     //items in rucksack 2
    rucksack rucksack3;     //items in rucksack 3
    rucksack duplicates12;  //duplicates between rucksack 1 and 2
    rucksack duplicates123; //duplicates between all three rucksacks

    int elf_counter = 0;
    int badge_item_sum = 0;
    while(std::getline(infs,line)){
        switch(elf_counter % 3){
            case 0:
                //Elf 1 of 3
                rucksack1 = fill_rucksack(line);
                break;
            case 1:
                //Elf 2 of 3
                rucksack2 = fill_rucksack(line);
                break;
            case 2:
                //Elf 3 of 3 -> gather results
                rucksack3 = fill_rucksack(line);
                duplicates12  = duplicate_items(rucksack1,rucksack2);
                duplicates123 = duplicate_items(duplicates12,rucksack3);
                //Check if indeed only one item is present in threefold
                if(duplicates123.size() != 1){
                    throw std::runtime_error("More than 1 item is present in threefold!");
                }
                badge_item_sum += duplicates123[0];
                break;
            default:
                break;
        }
        elf_counter++;
    }    

    std::cout << "Sum of badge items (part 2):" << badge_item_sum << std::endl;

    return 0;
}