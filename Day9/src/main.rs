use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn get_last_num(mut nums: Vec<i32>) -> i32 {

    let mut history : Vec<Vec<i32>> = Vec::new();

    loop {
        history.push( nums.clone() );

        let result = nums.iter()
            .zip(nums.iter().skip(1))
            //.inspect(|(a, b)| println!("a: {}, b: {}", a, b))
            .collect::<Vec<_>>();

        let mut diffs : Vec<i32> = Vec::new();
        for (a,b) in result
        {
            diffs.push( b - a);
        }

        nums = diffs.clone();

        let all_zeros = diffs.iter().all( |item|{
            return item.clone() == 0;
        });

        if all_zeros
        {
            break;
        }
    }

    history.reverse();

    let mut last_number = 0;
    let mut last_number_on_row = 0;
    for row in history
    {
        last_number_on_row = *row.last().unwrap();
        last_number += last_number_on_row;
        /*for item in row
        {
            print!("{} ", item )
        }
        println!();*/
    }

    return last_number;
}

fn main() {
    println!("Day 9");

    let lines = lines_from_file( "input.txt");

    let mut sum = 0;
    for line in lines
    {
        let nums: Vec<i32> = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();

        let last_num = get_last_num( nums.clone() );

        println!("New last number: {}", last_num);

        sum += last_num;
    }
    println!("Sum: {}", sum);
}


