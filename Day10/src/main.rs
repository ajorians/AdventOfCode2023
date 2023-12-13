use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use crate::PieceType::{Ground, HorizontalPipe, NE_Bend, NW_Bend, SE_Bend, StartingPos, SW_Bend, Unknown, VerticalPipe};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum PieceType {
    VerticalPipe,
    HorizontalPipe,
    NE_Bend,
    NW_Bend,
    SW_Bend,
    SE_Bend,
    Ground,
    StartingPos,
    Unknown,
}

struct Board
{
    board: Vec<Vec<PieceType>>,
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
    fn get_at(&self, x: i32, y: i32) -> PieceType
    {
        let piece_type: PieceType = self.board[y as usize][x as usize];
        return piece_type;
    }
    fn print(&self)
    {
        let width = self.get_width();
        let height = self.get_height();

        for y in 0..height
        {
            for x in 0..width
            {
                let spot = self.get_at(x, y);
                let ch =
                    {
                        if spot == VerticalPipe
                        {
                            '|'
                        } else if spot == HorizontalPipe
                        {
                            '-'
                        } else if spot == NE_Bend
                        {
                            'L'
                        } else if spot == NW_Bend
                        {
                            'J'
                        } else if spot == SW_Bend
                        {
                            '7'
                        } else if spot == SE_Bend
                        {
                            'F'
                        } else if spot == Ground
                        {
                            '.'
                        } else if spot == StartingPos
                        {
                            'S'
                        } else { ' ' }
                    };
                print!("{}", ch);
            }
            println!("");
        }
    }

    fn get_starting_pos(&self) -> (i32, i32)
    {
        let width = self.get_width();
        let height = self.get_height();

        for x in 0..width
        {
            for y in 0..height
            {
                if self.get_at(x, y) == StartingPos
                {
                    return (x, y);
                }
            }
        }

        return (0, 0);
    }

    fn remove_unconnected(&mut self)
    {
        let width = self.get_width();
        let height = self.get_height();

        loop {
            let mut made_changes: bool = false;
            for x in 0..width
            {
                for y in 0..height
                {
                    let spot = self.get_at(x, y);
                    if spot == Ground || spot == Unknown
                    {
                        continue;
                    }

                    if spot == VerticalPipe &&
                        ((y == 0 || y == self.get_height() - 1) ||
                            (
                                (self.get_at(x, y - 1) == NW_Bend || self.get_at(x, y - 1) == NE_Bend || self.get_at(x, y - 1) == HorizontalPipe || self.get_at(x, y - 1) == Ground) ||
                                    (self.get_at(x, y + 1) == SW_Bend || self.get_at(x, y + 1) == SE_Bend || self.get_at(x, y + 1) == HorizontalPipe || self.get_at(x, y + 1) == Ground)
                            ))
                    {
                        made_changes = true;
                        self.board[y as usize][x as usize] = Ground;
                        continue;
                    }

                    if spot == HorizontalPipe &&
                        ((x == 0 || x == self.get_width() - 1) ||
                            (
                                (self.get_at(x - 1, y) == NW_Bend || self.get_at(x - 1, y) == SW_Bend || self.get_at(x - 1, y) == VerticalPipe || self.get_at(x - 1, y) == Ground) ||
                                    (self.get_at(x + 1, y) == NE_Bend || self.get_at(x + 1, y) == SE_Bend || self.get_at(x + 1, y) == VerticalPipe || self.get_at(x + 1, y) == Ground)
                            ))
                    {
                        made_changes = true;
                        self.board[y as usize][x as usize] = Ground;
                    }

                    if spot == NE_Bend &&
                        ((y == 0 || x == self.get_width() - 1) ||
                            (
                                (self.get_at(x, y - 1) == NW_Bend || self.get_at(x, y - 1) == NE_Bend || self.get_at(x, y - 1) == HorizontalPipe || self.get_at(x, y - 1) == Ground) ||
                                    (self.get_at(x + 1, y) == NE_Bend || self.get_at(x + 1, y) == SE_Bend || self.get_at(x + 1, y) == VerticalPipe || self.get_at(x + 1, y) == Ground)
                            ))
                    {
                        made_changes = true;
                        self.board[y as usize][x as usize] = Ground;
                    }

                    if spot == NW_Bend &&
                        ((y == 0 || x == 0) ||
                            (
                                (self.get_at(x, y - 1) == NW_Bend || self.get_at(x, y - 1) == NE_Bend || self.get_at(x, y - 1) == HorizontalPipe || self.get_at(x, y - 1) == Ground) ||
                                    (self.get_at(x - 1, y) == NW_Bend || self.get_at(x - 1, y) == SW_Bend || self.get_at(x - 1, y) == VerticalPipe || self.get_at(x - 1, y) == Ground)
                            ))
                    {
                        made_changes = true;
                        self.board[y as usize][x as usize] = Ground;
                    }

                    if spot == SW_Bend &&
                        ((y == self.get_height() - 1 || x == 0) ||
                            (
                                (self.get_at(x, y + 1) == SW_Bend || self.get_at(x, y + 1) == SE_Bend || self.get_at(x, y + 1) == HorizontalPipe || self.get_at(x, y + 1) == Ground) ||
                                    (self.get_at(x - 1, y) == NW_Bend || self.get_at(x - 1, y) == SW_Bend || self.get_at(x - 1, y) == VerticalPipe || self.get_at(x - 1, y) == Ground)
                            ))
                    {
                        made_changes = true;
                        self.board[y as usize][x as usize] = Ground;
                    }

                    if spot == SE_Bend &&
                        ((y == self.get_height() - 1 || x == self.get_width() - 1) ||
                            (
                                (self.get_at(x, y + 1) == SW_Bend || self.get_at(x, y + 1) == SE_Bend || self.get_at(x, y + 1) == HorizontalPipe || self.get_at(x, y + 1) == Ground) ||
                                    (self.get_at(x + 1, y) == NE_Bend || self.get_at(x + 1, y) == SE_Bend || self.get_at(x + 1, y) == VerticalPipe || self.get_at(x + 1, y) == Ground)
                            ))
                    {
                        made_changes = true;
                        self.board[y as usize][x as usize] = Ground;
                    }
                }
            }

            if made_changes == false
            {
                break;
            }
        }
    }

    fn get_path(&self) -> Vec<(i32, i32)>
    {
        let mut path: Vec<(i32, i32)> = Vec::new();

        let (mut x, mut y) = self.get_starting_pos();
        path.push((x, y));
        loop {
            if x > 0 && !path.contains(&(x - 1, y)) && (self.get_at(x - 1, y) == HorizontalPipe || self.get_at(x - 1, y) == NE_Bend || self.get_at(x - 1, y) == SE_Bend)
            {
                x -= 1;
                path.push((x, y));
                continue;
            }
            if y > 0 && !path.contains(&(x, y - 1)) && (self.get_at(x, y - 1) == VerticalPipe || self.get_at(x, y - 1) == SW_Bend || self.get_at(x, y - 1) == SE_Bend)
            {
                y -= 1;
                path.push((x, y));
                continue;
            }
            if x < self.get_width() - 1 && !path.contains(&(x + 1, y)) && (self.get_at(x + 1, y) == HorizontalPipe || self.get_at(x + 1, y) == NW_Bend || self.get_at(x + 1, y) == SW_Bend)
            {
                x += 1;
                path.push((x, y));
                continue;
            }
            if y < self.get_height() - 1 && !path.contains(&(x, y + 1)) && (self.get_at(x, y + 1) == VerticalPipe || self.get_at(x, y + 1) == NW_Bend || self.get_at(x, y + 1) == NE_Bend)
            {
                y += 1;
                path.push((x, y));
                continue;
            }

            break;
        }

        return path;
    }
}

fn build_board(lines: Vec<String>) -> Board {
    let mut board_vec: Vec<Vec<PieceType>> = Vec::new();
    for line in lines
    {
        let mut row: Vec<PieceType> = Vec::new();
        for ch in line.chars()
        {
            let spot: PieceType =
                if ch == '|'
                {
                    VerticalPipe
                } else if ch == '-'
                {
                    HorizontalPipe
                } else if ch == 'L'
                {
                    NE_Bend
                } else if ch == 'J'
                {
                    NW_Bend
                } else if ch == '7'
                {
                    SW_Bend
                } else if ch == 'F'
                {
                    SE_Bend
                } else if ch == '.'
                {
                    Ground
                } else if ch == 'S'
                {
                    StartingPos
                } else {
                    Unknown
                };

            row.push(spot);
        }
        board_vec.push(row);
    }

    let board: Board = Board
    {
        board: board_vec
    };

    return board;
}

fn main() {
    println!("Day10");

    let lines = lines_from_file("input.txt");

    let mut board = build_board(lines);

    board.print();

    println!("-----------");

    board.remove_unconnected();
    board.print();

    /*for (x,y) in board.get_path()
    {
        println!("{},{}", x, y);
    }*/

    let path_length = board.get_path().len();
    println!("Board path length: {}", path_length);
    println!("Half path length: {}", path_length / 2);
}


