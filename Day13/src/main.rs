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
}

fn parse_boards(lines: Vec<String>) -> Vec<Board> {
    let mut boards : Vec<Board> = Vec::new();

    let mut board : Vec<Vec<char>> = Vec::new();

    for line in lines
    {
        if line.is_empty()
        {
            let new_board : Board = Board
            {
                board : board.clone()
            };

            boards.push(new_board);
            board.clear();
            continue;
        }

        let row : Vec<char> = line.chars().collect::<_>();
        board.push( row );
    }

    let new_board : Board = Board
    {
        board : board.clone()
    };

    boards.push(new_board);


    return boards;
}

fn columns_match(board: &Board, a: i32, b: i32) -> bool {
    for i in 0..board.get_height()
    {
        if board.get_at(a, i) != board.get_at(b,i)
        {
            return false;
        }
    }

    return true;
}

fn is_reflection_col(board: &Board, col: i32) -> bool {
    let columns_to_consider = std::cmp::min( board.get_width() - col, col );
    for offset in 1..=columns_to_consider
    {
        let column_a = col - offset;
        let column_b = col -1 + offset;

        if !columns_match( &board, column_a, column_b)
        {
            return false;
        }
    }

    return true;
}

fn rows_match(board: &Board, a: i32, b: i32) -> bool {
    for i in 0..board.get_width()
    {
        if board.get_at(i, a) != board.get_at(i, b)
        {
            return false;
        }
    }

    return true;
}

fn is_reflection_row(board: &Board, row: i32) -> bool {
    let rows_to_consider = std::cmp::min( board.get_height() - row, row );
    for offset in 1..=rows_to_consider
    {
        let row_a = row - offset;
        let row_b = row -1 + offset;

        if !rows_match( &board, row_a, row_b)
        {
            return false;
        }
    }

    return true;
}

fn find_reflection(board: Board) -> Option<i32> {
    for col in 1..board.get_width()
    {
        if is_reflection_col( &board, col )
        {
            return Some( col );
        }
    }

    for row in 1..board.get_height()
    {
        if is_reflection_row( &board, row )
        {
            return Some( row * 100 );
        }
    }

    return None;
}

fn main() {
    println!("Day13");

    let lines = lines_from_file("input.txt");

    let boards : Vec<Board> = parse_boards( lines );

    let mut sum = 0;
    for board in boards
    {
        let result = find_reflection( board ).unwrap();
        sum += result;
    }

    println!("Sum is {}", sum);
}

