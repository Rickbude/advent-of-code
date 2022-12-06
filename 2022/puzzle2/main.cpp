#include <iostream>
#include <fstream>

//Points for an "actual" rock-paper-scissors game.
// Input are - opponent move (0,1,2 = Rock,Paper,Scissors)
//           - your move     (0,1,2 = Rock,Paper,Scissors)
// Output is score:
//   - 6 for a win   +  points for shape (0,1,2 for Rock,Paper,Scissors respectively) 
//   - 3 for a draw  +  points for shape (0,1,2 for Rock,Paper,Scissors respectively) 
//   - 0 for a loss  +  points for shape (0,1,2 for Rock,Paper,Scissors respectively) 
int part1_points(int opponent, int you){    
        
    //You win if your index is one higher (mod 3) than your opponent:
    // - opponent has 0 (R), you need 1    (P)
    // - opponent has 1 (P), you need 2    (S)
    // - opponent has 2 (S), you need 3->0 (R)
    bool win  = you == (opponent+1)%3;
    bool draw = you ==  opponent;
    bool loss = (!win && !draw);

    //Calculate the points for the shape you played
    int  move_points = you + 1;

    //Return the accumulative points
    return (move_points + win*6 + draw*3 + loss*0);
}

//This is how the input was meant to be interpreted:
// column 1 is the opponent move (A,B,C), and column 2 weather you should win, draw or lose (X,Y,Z)
//We can reuse the scoring function from part 1
int part2_points(int opponent, int win_or_lose){
    // - to lose, we need one lower (mod 3) than the opponent -->  + 2 mod 3
    // - to draw, we need the same value as the opponent      -->  + 3 mod 3
    // - to win , we need 1 higher (mod 3)  than the opponent -->  + 4 mod 3
    // add 2 to avoid taking modulo of negative number
    char you = (opponent + win_or_lose + 2) % 3;
    return part1_points(opponent,you);
}

/*
    The elves play a Rock - Paper - Scissors contest, but you have inside knowledge.
*/
int main(){

    //Load input file (this file is copied to the build directory)
    std::fstream infs("input.txt");
    
    std::string line;
    int part1_score = 0;
    int part2_score = 0;
    
    while(std::getline(infs,line)){
        //Calculate score with the strategy from part 1
        part1_score += part1_points(line[0]-'A',line[2]-'X');
        //Calculate score with the strategy from part 2
        part2_score += part2_points(line[0]-'A',line[2]-'X');
    }
    std::cout << "total score when following strategy of part 1: " << part1_score << std::endl;
    std::cout << "total score when following strategy of part 2: " << part2_score << std::endl;

    return 0;
}