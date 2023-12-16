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

struct LineInfo
{
    arrangement : String,
    continuous_info : Vec<i32>
}

fn parse_line(line: String) -> LineInfo {
    let re = Regex::new(r"(.+)\s+(.+)").unwrap();

    let captures = re.captures(&*line).unwrap();
    let arrangement = String::from( &captures[1] );
    let continuous_info = String::from(&captures[2] ).trim().split(',').flat_map(str::parse::<i32>).collect::<Vec<_>>();

    let result : LineInfo = LineInfo
    {
        arrangement,
        continuous_info
    };

    return result;
}

fn create_combinations(arrangement: String) -> Vec<String> {
    let mut combinations : Vec<String> = Vec::new();

    if !arrangement.contains('?' )
    {
        combinations.push( arrangement );
        return combinations;
    }

    let operational = arrangement.replacen( "?", ".", 1);

    combinations.append(&mut create_combinations(operational));

    let damaged  = arrangement.replacen( "?", "#", 1);
    combinations.append(&mut create_combinations(damaged));

    return combinations;
}

fn is_valid_arrangement(arrangement: String, line_info: Vec<i32>) -> bool {
    let groups = arrangement.split("." ).filter(|g| !g.is_empty()).collect::<Vec<_>>();

    if groups.len() != line_info.len()
    {
        return false;
    }

    for i in 0..groups.len()
    {
        let items = groups[i].len() as i32;
        let expected = line_info[i];

        if items != expected
        {
            return false;
        }
    }

    return true;
}

fn compute_arrangements( line_info : LineInfo ) -> i32
{
    let combinations : Vec<String> = create_combinations( line_info.arrangement );

    let possible_arrangements =
    combinations.iter()
        .filter(|&arrangement| is_valid_arrangement( arrangement.clone(), line_info.continuous_info.clone()))
        .collect::<Vec<_>>();

    return possible_arrangements.len() as i32;
}

fn main() {
    println!("Day12");

    let lines = lines_from_file("input.txt");

    let mut total_counts = 0;
    for line in lines
    {
        let line_info : LineInfo = parse_line( line );

        let number_different_arrangements = compute_arrangements( line_info );
        total_counts += number_different_arrangements;
    }

    println!("Total counts: {}", total_counts);
}


