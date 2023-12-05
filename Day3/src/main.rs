use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

struct Board
{
    rows: Vec<Vec<char>>
}

struct Item
{
    value: i32,
    x: i32,
    y: i32
}

impl Board
{
    fn get_width(&self)-> i32
    {
        return self.rows[0].len() as i32;
    }
    fn get_height(&self)->i32
    {
        return self.rows.len() as i32;
    }
    fn is_number(&self, x:i32, y:i32)->bool
    {
        if x < 0 || x >= self.get_width()
        {
            return false;
        }
        if y < 0 || y >= self.get_height()
        {
            return false;
        }

        let ch : char = self.rows[y as usize][x as usize];
        return ch.is_digit(10);
    }
    fn is_symbol(&self, x:i32, y:i32)->bool
    {
        if x < 0 || x >= self.get_width()
        {
            return false;
        }
        if y < 0 || y >= self.get_height()
        {
            return false;
        }

        let ch : char = self.rows[y as usize][x as usize];
        if ch.is_whitespace()
        {
            return false;
        }
        else if ch.is_digit( 10 )
        {
            return false;
        }
        else if ch == '.'
        {
            return false;
        }

        return true;
    }
    fn is_part(&self, item: &Item) -> bool
    {
        for y in item.y-1..item.y+2
        {
            for x in item.x-1..item.x+item.value.to_string().len() as i32+1
            {
                if self.is_symbol(x,y)
                {
                    return true;
                }
            }
        }
        return false;
    }
    fn get_value(&self, x: i32, y: i32, length:i32)->i32
    {
        let mut value: i32 = 0;
        for i in x..x+length
        {
            let ch : char = self.rows[y as usize][i as usize];
            let v: i32 = ch.to_digit(10 ).unwrap() as i32;
            value = value*10 + v;
        }
        return value;
    }
    fn get_items(&self) -> Vec<Item>
    {
        let mut items : Vec<Item> = Vec::new();

        for y in 0..self.get_height()
        {
            let mut start_index = -1;
            for x in 0..self.get_width()+1
            {
                if self.is_number( x, y )
                {
                    if start_index == -1
                    {
                        start_index = x;
                    }
                }
                else if x == self.get_width() && start_index != -1
                {
                    let item : Item = Item{
                        value: self.get_value(start_index, y, x-start_index),
                        x: start_index,
                        y:y
                    };

                    items.push( item );
                }
                else {
                    if start_index != -1
                    {
                        let item : Item = Item{
                          value: self.get_value(start_index, y, x-start_index),
                            x: start_index,
                            y:y
                        };

                        items.push( item );

                        start_index = -1;
                    }
                }
            }
        }

        return items;
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn build_board( contents : &Vec<String> )->Board
{
    let mut rows : Vec<Vec<char>> = Vec::new();
    for line in contents
    {
        let mut row_line : Vec<char> = Vec::new();
        for ch in line.chars()
        {
            row_line.push( ch );
        }
        rows.push( row_line);
    }

    let mut board : Board = Board
    {
        rows : rows
    };

    board
}

fn main() {
    println!("Day 3");

    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut board = build_board( &contents );
    println!("Width: {}, Height: {}", board.get_width(), board.get_height());

    let items =  board.get_items();
    let mut total_parts = 0;
    for item in items
    {
        println!("Item value: {}", item.value);
        if board.is_part( &item )
        {
            println!("Is part");
            total_parts += item.value;
        }
    }

    println!( "Total part values: {}", total_parts);
}
