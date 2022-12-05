#include <iostream>
#include <fstream>
#include <unordered_map>

/*
    The elves play a Rock - Paper - Scissors contest, but you have inside knowledge.

    This code gets the thing done, but could be made nicer:
        - The Rock Paper Scissors game could probably be nicely be implemented with a 
          circular linked list, to determine which moves win / lose
        - The shape_score map is for convenience, but can also be easily calculated 
          directly from the input (score = char - 'X')
    I can't really be bothered though
*/
int main(){

    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");
    
    // ----------------------- Game definition ---------------------------
    
    //Shape map, just for convenience (R,P,S makes clearer what is what)
    std::unordered_map<char,char> shape_map = {
       { 'A', 'R' }, //Rock
       { 'B', 'P' }, //Paper
       { 'C', 'S' }, //Scissors
       { 'X', 'R' }, //Rock
       { 'Y', 'P' }, //Paper
       { 'Z', 'S' }  //Scissors
    };

    //Points for each shape
    std::unordered_map<char,int> shape_score = {
       { 'R', 1 }, //Rock
       { 'P', 2 }, //Paper
       { 'S', 3 }, //Scissors
    };

    //Winning replies to opponent
    std::unordered_map<char,char> win {
        {'R','P'}, // Enemy plays Rock,     you play Paper
        {'P','S'}, // Enemy plays Paper,    you play Scissors
        {'S','R'}, // Enemy plays Scissors, you play Rock
    };

    //Losing replies to opponent
    //Note that this is the reverse of the previous map
    std::unordered_map<char,char> loss {
        {'R','S'}, // Enemy plays Rock,     you play Scissors
        {'P','R'}, // Enemy plays Paper,    you play Rock
        {'S','P'}, // Enemy plays Scissors, you play Paper
    };


    // -----------------  Part 1  -------------------------------------

    std::string line;
    int score = 0;
    while(std::getline(infs,line)){
        //Load opponent move and (supposedly) your counter move
        char enemy_move   = shape_map[line[0]];
        char counter_move = shape_map[line[2]];

        //The shape you play is part of the total score
        score += shape_score[counter_move];

        //Determine if you win, lose, or draw the game and add points
        if(enemy_move == counter_move){
            //Draw: same shape
            score += 3;
        }else if(counter_move == win[enemy_move]){
            //Win
            score += 6;
        }
    }
    std::cout << "total score when following wrong strategy: " << score << std::endl;

    // -----------------  Part 2  -------------------------------------

    //Reset filestream
    infs.clear();
    infs.seekg(0);
    score = 0;
    while(std::getline(infs,line)){
        //Load opponent move and whether you should win/draw/lose
        char enemy_move   = shape_map[line[0]];
        char win_lose     = line[2];
        
        //Determine the appropriate counter move
        char counter_move;
        switch(win_lose){
            case 'X':
                //You need to lose
                counter_move = loss[enemy_move];
                break;
            case 'Y':
                //You need to draw
                score += 3;
                counter_move = enemy_move;
                break;
            case 'Z':
                //You need to win
                score += 6;
                counter_move = win[enemy_move];
                break;
            default:
                break;
        }

        //Add the score of your shape
        score += shape_score[counter_move];
    }
    std::cout << "total score when following correct strategy: " << score << std::endl;

    return 0;
}