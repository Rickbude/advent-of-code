use std::fs::File;
use std::io::prelude::*;

//Hand types, in decreasing order of power
enum HandType{
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

//Calculate the score for a collection of hands
fn calculate_score(part : usize, symbols: &str, contents: &str) -> usize {
    let lines = contents.lines();
    let mut hands: Vec<Vec<usize>> = Vec::new();
    for line in lines{
        //Split hand and bid
        let mut parts = line.split_whitespace();
        let hand_str = parts.next().unwrap();
        let bid  = parts.next().unwrap().parse::<usize>().unwrap();

        //Count the number of cards of type held in the hand
        let mut hand: Vec<usize> = Vec::new();
        let mut jokers = 0;
        for symbol in symbols.chars(){
            let num_cards = hand_str.chars().filter(|x| x == &symbol).count();
            if part == 2 && symbol == 'J' {
                jokers = num_cards;
            } else {
                hand.push(num_cards);
            }
        }
        
        //Add Jokers to the current maximum held number of cards. This always
        //seems to be the optimal strategy.
        let index_of_max = hand
            .iter()
            .enumerate()
            .max_by_key(|(_idx, &val)| val)
            .map(|(idx, _val)| idx).unwrap();
        hand[index_of_max] += jokers;

        //Determine what category the hand falls into
        let hand_type = match hand {
            _ if hand.contains(&5) => HandType::FiveOfAKind,
            _ if hand.contains(&4) => HandType::FourOfAKind,
            _ if hand.contains(&3) && hand.contains(&2) => HandType::FullHouse,
            _ if hand.contains(&3) => HandType::ThreeOfAKind,
            _ if hand.iter().filter(|x| x == &&2).count() == 2 => HandType::TwoPair,
            _ if hand.contains(&2) => HandType::OnePair,
            _ => HandType::HighCard
        };
    
        //Convert card characters to sortable integers
        //Append the hand type, prepend the bid, for use of use
        let mut hand_int : Vec<usize> = Vec::new();
        hand_int.push(hand_type as usize);
        hand_str.chars().for_each(|x| hand_int.push(symbols.find(x).unwrap()));
        hand_int.push(bid);
        hands.push(hand_int);
    }

    //Sort the hand and calculate the final score
    hands.sort();
    let mut score = 0;
    for (rank,hand) in hands.iter().enumerate(){
        score += (rank+1)*hand.last().unwrap();
    }
    return score;
}

fn main() -> std::io::Result<()> {
    //Open test file
    //let mut file = File::open("src/test.txt")?;
    //Open actual input file
    let mut file = File::open("src/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    println!("Key part 1: {}", calculate_score(1,"123456789TJQKA",&contents));
    println!("Key part 2: {}", calculate_score(2,"J123456789TQKA",&contents));

    Ok(())
}
