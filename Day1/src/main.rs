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

fn first_digit(line: &String) -> i32 {
    for c in line.chars() {
        if c.is_digit(10)
        {
            return c.to_digit( 10 ).unwrap() as i32;
        }
    }

    0
}

fn last_digit(line: &String) -> i32 {
    for c in line.chars().rev() {
        if c.is_digit(10)
        {
            return c.to_digit( 10 ).unwrap() as i32;
        }
    }

    0
}

fn numbers_from_line(line: &String) ->i32 {
    let first_digit = first_digit( &line );
    let last_digit = last_digit( &line );

    first_digit*10 + last_digit
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut total = 0;
    for line in contents {
        let line_result = numbers_from_line( &line );

        println!("{:?}: {:?}", line.clone(), line_result);
        total += line_result;
    }

    println!("Total: {:?}", total);
}

