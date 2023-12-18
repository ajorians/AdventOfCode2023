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
    fn set_at(&mut self, x: i32, y: i32, ch: char)
    {
        self.board[y as usize][x as usize] = ch;
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

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West
}

fn move_pos(pos: (i32, i32), dir: Direction) -> (i32, i32) {
    if dir == Direction::North
    {
        return (pos.0, pos.1-1);
    }
    if dir == Direction::East
    {
        return (pos.0+1, pos.1);
    }
    if dir == Direction::South
    {
        return (pos.0, pos.1+1);
    }
    if dir == Direction::West
    {
        return (pos.0-1, pos.1);
    }

    return pos;
}

fn get_beam_spots(beam_spots: &mut Vec<(i32, i32, Direction)>, board: &Board, mut dir : Direction, mut pos : (i32, i32)) {
    loop {
        if pos.0 < 0 || pos.0 >= board.get_width()
        {
            break;
        }
        if pos.1 < 0 || pos.1 >= board.get_height()
        {
            break;
        }
        if beam_spots.contains( &(pos.0, pos.1, dir) )
        {
            break;
        }

        let spot = board.get_at(pos.0, pos.1);

        beam_spots.push( (pos.0, pos.1, dir) );
        if spot == '.'
        {
            pos = move_pos( pos, dir );
            continue;
        }
        else if spot == '/'
        {
            if dir == Direction::North
            {
                dir = Direction::East;
            }
            else if dir == Direction::East
            {
                dir = Direction::North;
            }
            else if dir == Direction::South
            {
                dir = Direction::West;
            }
            else if dir == Direction::West
            {
                dir = Direction::South;
            }
            pos = move_pos( pos, dir );
            continue;
        }
        else if spot == '\\'
        {
            if dir == Direction::North
            {
                dir = Direction::West;
            }
            else if dir == Direction::East
            {
                dir = Direction::South;
            }
            else if dir == Direction::South
            {
                dir = Direction::East;
            }
            else if dir == Direction::West
            {
                dir = Direction::North;
            }
            pos = move_pos( pos, dir );
            continue;
        }
        else if spot == '|'
        {
            if dir == Direction::North
            {
                pos = move_pos( pos, dir );
                continue;
            }
            else if dir == Direction::East
            {
                get_beam_spots(beam_spots, board, Direction::North, move_pos(pos, Direction::North) );
                get_beam_spots(beam_spots, board, Direction::South, move_pos(pos, Direction::South) );

                return;
            }
            else if dir == Direction::South
            {
                pos = move_pos( pos, dir );
                continue;
            }
            else if dir == Direction::West
            {
                get_beam_spots(beam_spots, board, Direction::North, move_pos(pos, Direction::North) );
                get_beam_spots(beam_spots, board, Direction::South, move_pos(pos, Direction::South) );

                return;
            }
            continue;
        }
        else if spot == '-'
        {
            if dir == Direction::North
            {
                get_beam_spots(beam_spots, board, Direction::West, move_pos(pos, Direction::West) );
                get_beam_spots(beam_spots, board, Direction::East, move_pos(pos, Direction::East) );

                return;
            }
            else if dir == Direction::East
            {
                pos = move_pos( pos, dir );
                continue;
            }
            else if dir == Direction::South
            {
                get_beam_spots(beam_spots, board, Direction::West, move_pos(pos, Direction::West) );
                get_beam_spots(beam_spots, board, Direction::East, move_pos(pos, Direction::East) );

                return;
            }
            else if dir == Direction::West
            {
                pos = move_pos( pos, dir );
                continue;
            }
            continue;
        }
    }

    return;
}

fn main() {
    println!("Day16");

    let lines = lines_from_file("input.txt");

    let board : Board = parse_board( lines );

    let mut beam_spots : Vec<(i32, i32, Direction)> = Vec::new();
    get_beam_spots( &mut beam_spots, &board, Direction::East, (0, 0) );

    let mut energized_cells : Vec<(i32, i32)> = Vec::new();
    for beam_spot in beam_spots
    {
        if !energized_cells.contains( &(beam_spot.0, beam_spot.1))
        {
            energized_cells.push( (beam_spot.0, beam_spot.1));
        }
    }

    println!("Energized cells {}", energized_cells.len() as i32);
}

