#include <iostream>
#include <fstream>
#include <unordered_map>
#include <string>

//Check if range1 fully contains range2, or vice versa
bool full_range_overlap(int start1, int end1, int start2, int end2){
    return (start1>=start2 && end1<=end2) ||
           (start2>=start1 && end2<=end1) ;
}

//Check if range1 fully contains range2, or vice versa
bool partial_range_overlap(int start1, int end1, int start2, int end2){
    //See https://nedbatchelder.com/blog/201310/range_overlap_in_two_compares.html
    //Instead of thinking when ranges DO overlap, it is easier to consider when they DON'T
    //Ranges do NOT overlap if entire range 1 < range 2 (or vice versa)
    // --> return !(end1 < start2 || end2 < start1)
    //Or, equivalently: (using de Morgan's laws (negation of a disjunction: !(A v B) = (!A & !B))    
    return (end1 >= start2) && (end2 >= start1);
}

int main(){

    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");    
    
    std::string line;
    int N_full_overlap=0;
    int N_partial_overlap=0;
    while(std::getline(infs,line)){
        //Split the input line at the ','
        size_t delim_pos = line.find(',');
        std::string plots_elf1 = line.substr(0,delim_pos);
        std::string plots_elf2 = line.substr(delim_pos+1);

        //Split plots for elf 1 (delimited with '-')
        size_t delim_pos_elf1 = plots_elf1.find('-');
        int elf1_plot1 = std::stoi(plots_elf1.substr(0,delim_pos_elf1));
        int elf1_plot2 = std::stoi(plots_elf1.substr(delim_pos_elf1+1));

        //Split plots for elf 2 (delimited with '-')
        size_t delim_pos_elf2 = plots_elf2.find('-');
        int elf2_plot1 = std::stoi(plots_elf2.substr(0,delim_pos_elf2));
        int elf2_plot2 = std::stoi(plots_elf2.substr(delim_pos_elf2+1));

        //Now check if a range is overlapping
        N_full_overlap    +=    full_range_overlap(elf1_plot1,elf1_plot2,elf2_plot1,elf2_plot2);
        N_partial_overlap += partial_range_overlap(elf1_plot1,elf1_plot2,elf2_plot1,elf2_plot2);
    }

    std::cout << "number of fully overlapping tasks is  "  << N_full_overlap << std::endl;
    std::cout << "number of partial overlapping tasks is " << N_partial_overlap << std::endl;
        
    return 0;
}