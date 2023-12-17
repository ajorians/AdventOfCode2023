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

struct Board
{
    board: Vec<Vec<char>>,
}

impl Board
{
    fn get_width(&self) -> i32
    {
        return self.board[0].len() as i32;
    }
    fn get_height(&self) -> i32
    {
        return self.board.len() as i32;
    }
    fn get_at(&self, x: i32, y: i32) -> char
    {
        let ch = self.board[y as usize][x as usize];
        return ch;
    }
    fn set_at(&mut self, x : i32, y : i32, ch : char )
    {
        self.board[y as usize][x as usize] = ch;
    }

    fn move_up(&mut self) {
        loop {
            let mut made_changes = false;
            for x in 0..self.get_width()
            {
                for y in 1..self.get_height()
                {
                    if self.get_at(x, y) == 'O'
                    {
                        if self.get_at(x,y-1) == '.'
                        {
                            self.set_at(x,y, '.');
                            self.set_at(x,y-1, 'O');
                            made_changes = true;
                        }
                    }
                }
            }

            if made_changes == false
            {
                break;
            }
        }
    }
}

fn parse_board(lines: Vec<String>) -> Board {
    let mut board : Vec<Vec<char>> = Vec::new();

    for line in lines
    {
        let row : Vec<char> = line.chars().collect::<_>();
        board.push( row );
    }

    let new_board : Board = Board
    {
        board : board.clone()
    };

    return new_board;
}

fn get_weight(board: Board) -> i32 {
    let mut weight = 0;
    for y in 0..board.get_height()
    {
        let row_val = board.get_height()-y;
        for x in 0..board.get_width()
        {
            if board.get_at(x,y ) == 'O'
            {
                weight += row_val;
            }
        }
    }

    return weight;
}

fn main() {
    println!("Day14");

    let lines = lines_from_file("input.txt");

    let mut board : Board = parse_board( lines );

    board.move_up();

    println!("Wight: {}", get_weight( board ));
}

