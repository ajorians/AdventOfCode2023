use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;
use std::collections::HashSet;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

struct CardInfo
{
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>
}

fn read_card( card: &String )->CardInfo
{
    let (card_detail, numbers) = card.split_at(card.find(':').unwrap());

    let numbers = &numbers[2..];

    let (winning_numbers, my_numbers) = numbers.split_at(numbers.find('|').unwrap());

    let my_numbers = &my_numbers[2..].trim();

    let winning_numbers: Vec<i32> = winning_numbers.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();

    let my_numbers: Vec<i32> = my_numbers.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();

    let card_info : CardInfo = CardInfo
    {
        winning_numbers : winning_numbers,
        my_numbers : my_numbers
    };

    return card_info;
}

fn get_card_points( card : &String) -> i32
{
    let card_info = read_card( card );

    let my_numbers = card_info.my_numbers.iter().collect::<HashSet<_>>();
    let winning_numbers = card_info.winning_numbers.iter().collect::<HashSet<_>>();

    let my_winning_numbers = winning_numbers.intersection(&my_numbers).collect::<Vec<_>>();

    let mut points = 0;
    if !my_winning_numbers.is_empty()
    {
        points = 1;
        for i in 1..my_winning_numbers.len()
        {
            points *= 2;
        }
    }

    return points;
}

fn main() {
    println!("Day4");

    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut total_points = 0;
    for line in contents
    {
        let points = get_card_points( &line );
        total_points+= points;
    }

    println!("Total points: {}", total_points);
}
