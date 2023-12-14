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
        let piece_type: char = self.board[y as usize][x as usize];
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

                print!("{}", spot);
            }
            println!("");
        }
    }

    fn expand_empty(&mut self) {
        let mut empty_rows: Vec<i32> = Vec::new();
        let mut empty_columns: Vec<i32> = Vec::new();

        let width = self.get_width();
        let height = self.get_height();

        for y in 0..height
        {
            let mut all_empty = true;
            for x in 0..width
            {
                let spot = self.get_at(x, y);

                if spot == '#'
                {
                    all_empty = false;
                    break;
                }
            }
            if all_empty == true
            {
                empty_rows.push(y);
            }
        }

        for x in 0..width
        {
            let mut all_empty = true;
            for y in 0..height
            {
                let spot = self.get_at(x, y);

                if spot == '#'
                {
                    all_empty = false;
                    break;
                }
            }
            if all_empty == true
            {
                empty_columns.push(x);
            }
        }

        empty_rows.reverse();
        empty_columns.reverse();

        for i in empty_rows.clone()
        {
            let mut row: Vec<char> = Vec::new();
            for _ in 0..width
            {
                row.push('.');
            }

            self.board.insert(i as usize, row);
        }


        let height = self.get_height();

        for i in empty_columns.clone()
        {
            for j in 0..height
            {
                self.board[j as usize].insert(i as usize, '.');
            }
        }
    }

    fn get_galaxies(&self) -> Vec<(i32, i32)>
    {
        let mut result: Vec<(i32, i32)> = Vec::new();

        let width = self.get_width();
        let height = self.get_height();

        for y in 0..height
        {
            for x in 0..width
            {
                let spot = self.get_at(x, y);

                if spot == '#'
                {
                    result.push((x, y));
                }
            }
        }

        return result;
    }
}

fn build_board(lines: Vec<String>) -> Board {
    let mut board_vec: Vec<Vec<char>> = Vec::new();
    for line in lines
    {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars()
        {
            let spot = ch;

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
    println!("Day11");

    let lines = lines_from_file("input.txt");

    let mut board = build_board(lines);

    board.expand_empty();

    //board.print();

    let galaxies = board.get_galaxies();

    let mut pairs: Vec<((i32, i32), (i32, i32))> = Vec::new();
    for i in 0..galaxies.len()
    {
        for j in i + 1..galaxies.len()
        {
            let pair = (galaxies[i], galaxies[j]);
            pairs.push(pair);
        }
    }

    println!("This many pairs {}", pairs.len());

    let mut total_distance = 0;
    for ((a_x, a_y), (b_x, b_y)) in pairs
    {
        let v_distance = (a_y - b_y).abs();
        let h_distance = (a_x - b_x).abs();

        let distance = v_distance + h_distance;

        total_distance += distance;
    }

    println!("Total distance: {}", total_distance);
}
