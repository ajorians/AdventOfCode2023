use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::cmp::Ordering;
use std::collections::HashMap;
use crate::HandKind::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, ThreeOfAKind};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Eq)]
struct Hand
{
    text : String
}

#[derive(PartialEq, PartialOrd)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

fn get_hand_kind(hand : &Hand ) ->HandKind
{
    let mut letter_counts: HashMap<char,i32> = HashMap::new();

    let char_vec: Vec<char> = hand.text.chars().collect();

    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }

    let counts : Vec<i32> = letter_counts.values().cloned().collect();

    if counts.contains(&5)
    {
        return FiveOfAKind;
    }

    if counts.contains( &4 )
    {
        return FourOfAKind;
    }
    if counts.contains( &3 ) && counts.contains( &2)
    {
        return FullHouse;
    }
    if counts.contains( &3 )
    {
        return ThreeOfAKind;
    }
    if counts.len() == 3 && counts.contains( &2 )
    {
        return HandKind::TwoPair;
    }
    if counts.contains( &2 )
    {
        return HandKind::OnePair;
    }

    return HighCard;
}

fn compare_first_card( a : char, b : char ) -> Ordering
{
    if a == b
    {
        return Ordering::Equal;
    }

    if a == 'A'
    {
        return Ordering::Greater;
    }
    else if b == 'A'
    {
        return Ordering::Less;
    }

    if a == 'K'
    {
        return Ordering::Greater;
    }
    else if b == 'K'
    {
        return Ordering::Less;
    }

    if a == 'Q'
    {
        return Ordering::Greater;
    }
    else if b == 'Q'
    {
        return Ordering::Less;
    }

    if a == 'J'
    {
        return Ordering::Greater;
    }
    else if b == 'J'
    {
        return Ordering::Less;
    }

    if a == 'T'
    {
        return Ordering::Greater;
    }
    else if b == 'T'
    {
        return Ordering::Less;
    }

    return a.cmp(&b);
}

fn compare_by_first_card( a : &String, b : &String ) -> Ordering
{
    for i in 0..5
    {
        let cmp = compare_first_card(a.chars().nth(i).unwrap(), b.chars().nth(i).unwrap());
        if cmp != Ordering::Equal
        {
            return cmp;
        }
    }

    return Ordering::Equal;
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if get_hand_kind( self ) < get_hand_kind( other )
        {
            return Ordering::Less;
        }
        else if get_hand_kind( self ) > get_hand_kind( other )
        {
            return Ordering::Greater;
        }

        return compare_by_first_card( &self.text, &other.text);
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

#[derive(Eq)]
struct HandAndBid
{
    hand : Hand,
    bid : i32
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandAndBid {
    fn eq(&self, other: &Self) -> bool {
        self.bid == other.bid
    }
}

fn parse_hand_and_bid( line : &String)->HandAndBid
{
    let hand : Hand = Hand
    {
        text : line[0..5].trim().to_string()
    };
    let bid = str::parse::<i32>( line[5..].trim() ).unwrap();

    let result : HandAndBid = HandAndBid
    {
        hand : hand,
        bid : bid
    };

    return result;
}

fn convert_to_hand_and_bid(lines: &Vec<String>) -> Vec<HandAndBid> {
    let mut result : Vec<HandAndBid> = Vec::new();

    for line in lines
    {
        let hand_and_bid = parse_hand_and_bid( line );
        result.push( hand_and_bid );
    }

    return result;
}

fn main() {
    println!("Day7");

    let lines = lines_from_file( "input.txt");

    let mut hands_and_bids : Vec<HandAndBid> = convert_to_hand_and_bid( &lines );

    hands_and_bids.sort();

    //hands_and_bids.reverse();

    let mut total = 0;
    let mut rank = 1;
    for item in hands_and_bids
    {
        let winnings = rank * item.bid;
        total += winnings;

        println!("{} {}, rank: {}", item.hand.text, item.bid, rank);
        rank+=1;

    }
    println!("Winnings: {}", total);
}


