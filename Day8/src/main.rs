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

struct Node
{
    name : String,
    left : String,
    right : String
}

fn parse_mapping(map_line : &String) ->Node{
    let re = Regex::new(r"(...)\s+=\s+\((...),\s+(...)\)").unwrap();

    let captures = re.captures(&*map_line).unwrap();
    let name = &captures[1];
    let left = &captures[2];
    let right = &captures[3];

    let result : Node = Node
    {
        name : name.to_string(),
        left : left.to_string(),
        right : right.to_string()
    };

    return result;
}

fn get_node(mappings: &Vec<Node>, node_name: String) ->Option<&Node> {
    for node  in mappings
    {
        if node.name == node_name
        {
            return Option::from( node );
        }
    }

    return Option::None;
}

fn main() {
    println!("Day8");

    let lines = lines_from_file( "input.txt");

    let pattern = lines[0].clone();

    let lines_of_mappings = lines.iter().skip(2).collect::<Vec<_>>();

    let mut mappings : Vec<Node> = Vec::new();
    for map_line in lines_of_mappings
    {
        let mapping = parse_mapping( &map_line );
        mappings.push( mapping );
    }


    let mut current_node : &Node = get_node(&mappings, "AAA".to_string()).unwrap();
    let mut step_count = 0;
    'outer: loop {
        for direction in pattern.chars()
        {
            let new_node_name : String =
                    if direction == 'L'
                    {
                        current_node.left.clone()
                    } else {
                        current_node.right.clone()
                    };

            step_count += 1;

            if new_node_name == "ZZZ"
            {
                break 'outer;
            }

            current_node = get_node(&mappings, new_node_name).unwrap();
        }
    }

    println!("Step count: {}", step_count);

}


