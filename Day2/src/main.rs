use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use regex::Regex;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn game_number( line: &String)->i32
{
    let re = Regex::new(r"Game\s+(\d+):").unwrap();
    if re.is_match( line ) == false
    {
        return 0;
    }

    let captures = re.captures( line ).unwrap();
    let result = &captures[1];
    return result.parse::<i32>().unwrap();
}

struct Items {
    red : i32,
    blue : i32,
    green : i32
}

fn count_items(part: &str) -> Items {
    let mut item = Items
    {
        red: 0,
        blue: 0,
        green: 0
    };

    {
        let re = Regex::new(r"(\d+)\s+red").unwrap();
        if re.is_match( part ) == true
        {
            let captures = re.captures( part ).unwrap();
            let result = &captures[1];
            item.red = result.parse::<i32>().unwrap();
        }
    }
    {
        let re = Regex::new(r"(\d+)\s+blue").unwrap();
        if re.is_match( part ) == true
        {
            let captures = re.captures( part ).unwrap();
            let result = &captures[1];
            item.blue = result.parse::<i32>().unwrap();
        }
    }
    {
        let re = Regex::new(r"(\d+)\s+green").unwrap();
        if re.is_match( part ) == true
        {
            let captures = re.captures( part ).unwrap();
            let result = &captures[1];
            item.green = result.parse::<i32>().unwrap();
        }
    }

    item
}

fn get_items( line: &String)->Vec<Items>
{
    let mut result : Vec<Items> = Vec::new();

    let colon_index = line.find(":").unwrap();
    let item_parts = &line[colon_index..];

    for item_part in item_parts.split(";")
    {
        result.push( count_items( item_part ));
    }

    result
}

fn main() {
    println!("Day2");

    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut game_number_count = 0;
    for line in contents
    {
        let game_number : i32 = game_number(&line);
        println!("Game number {}", game_number);

        let items = get_items( &line );
        let mut possible = true;
        for item in items
        {
            if item.red > 12
            {
                println!("Too many reds {} > 12", item.red);
                possible = false;
                break;
            }
            else if item.green > 13 {
                println!("Too many green {} > 13", item.green);
                possible = false;
                break;
            }
            else if item.blue > 14 {
                println!("Too many blue {} > 14", item.blue);
                possible = false;
                break;
            }
        }

        if possible == true
        {
            game_number_count += game_number;
        }
    }

    println!("Sum of Game numbers: {}", game_number_count);
}
