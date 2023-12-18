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

fn compute_hash(input: String) -> i32 {
    let mut result = 0;
    for ch in input.chars()
    {
        let ascii = ch as i32;
        result += ascii;
        result *= 17;
        result %= 256;
    }

    return result;
}

fn main() {
    println!("Day15");

    let lines = lines_from_file("input.txt");

    let mut sum = 0;
    for line in lines
    {
        let sections = line.split(',').collect::<Vec<_>>();
        for section in sections
        {
            let hash = compute_hash(String::from( section ));
            sum += hash;
        }
    }

    println!("Sum: {}", sum);
}
