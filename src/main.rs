

#[derive(Debug)]
enum Piece {
    Black,
    White,
    None
}
pub struct Board {
    black: u64,
    white: u64,
    black_pieces_count: usize,
    white_pieces_count: usize,
    turns: Piece
}


const BOARD_SIZE: i32 = 8;

pub struct Coord {
    x: i32,
    y: i32
} 

/*
todo : 演算子オーバーロード
impl Coord {
    fn add(self, ) {

    }
}
*/

const DIRECTION:[Coord; 8] = [
    Coord {x: 1, y: -1},
    Coord {x: 1, y: 0},
    Coord {x: 1, y: 1},
    Coord {x: 0, y: 1},
    Coord {x: 0, y: -1},
    Coord {x: -1, y: 0},
    Coord {x: -1, y: 1},
    Coord {x: -1, y: -1},
    ];



impl Board {
    fn new() -> Self {
        Board {
            black: 0x0000000810000000u64,
            white: 0x0000001008000000u64,
            black_pieces_count: 2usize,
            white_pieces_count: 2usize,
            turns: Piece::Black
        }
    }

    #[inline]
    fn get_piece(&self, y: i32, x: i32) -> Piece {
        let mask: u64 = 1 << y * BOARD_SIZE + x;

        let blank = !(self.black| self.white);

        if self.black & mask != 0 {Piece::Black}
        else if self.white & mask != 0 {Piece::White}
        else {Piece::None}
    }
    fn put_piece(&mut self, y: i32, x: i32) {
        let mask = 1 << y * BOARD_SIZE + x;
        match self.turns {
            Piece::Black => {
                self.black += mask;
                self.turns = Piece::White;
            },
            Piece::White => {
                self.white += mask;
                self.turns = Piece::Black;
            },
            Piece::None => ()
        }
    }

    fn put_able(&self, coord: Coord) -> bool{
        let mut mask = 0xFEFEFEFEFEFEFE; // 左端が0
        /* 0xFEFEFEFEFEFEFE 
        1 1 1 1 1 1 1 0
        1 1 1 1 1 1 1 0
        1 1 1 1 1 1 1 0
        1 1 1 1 1 1 1 0
        1 1 1 1 1 1 1 0
        1 1 1 1 1 1 1 0
        1 1 1 1 1 1 1 0
        1 1 1 1 1 1 1 0
        */
        
        let blank = !(self.black | self.white);


        // black_left
        // 合法手の調べ方（bit board）
        // https://speakerdeck.com/antenna_three/bitutobodojie-shuo?slide=44
        let left = (self.black << 1) & mask; 
        let mut left_white = left & self.white;
        for _ in 0..6 {
            left_white |= (left_white << 1) & mask & self.white;
        }
        let mut legal = (left_white << 1) & mask & blank;

        true
    }legal

    #[inline]
    fn _get_cow(&self, h: i32, w: i32, count : i32, d: (i32,i32)) -> Piece{
        self.get_piece(h + d.0 * count, w + d.1 * count)
    }
}

mod cui_test {

    use crate::{Board, Piece};
    //use termion::*;
    use std::io::stdin;
    //use termion::input::TermRead;
    //use termion::raw::IntoRawMode;

    pub fn print_board(board: &Board, y_now: i32, x_now: i32){
        println!("black: {}\nwhite: {}", board.black_pieces_count, board.white_pieces_count);
        for y in 0..8 {
            for x in 0..8 {
                let value: char = {
                    if y == y_now && x == x_now {'*'}
                    else {
                        match board.get_piece(y, x) {
                            Piece::Black => 'x',
                            Piece::White => 'o',
                            Piece::None  => '.',
                        }
                    }
                };
                print!("{} ", value);
            }
            println!();
        }

        /*
        高速化 （思考aiを作るときに参照するために残している。）
        let mut index: u32 = 0;
        let now_mask: u64 = 1 << y_now * 8 + x_now;
        for _ in 0..8 {
            for _ in 0..8 {
                let mask: u64 = 1 << index;
                let value: char = 
                    if mask == now_mask {'*'}
                    else if board.black & mask != 0 {'x'} 
                    else if board.white & mask != 0 {'o'}
                    else {'.'};
                print!("{} ", value);
                index += 1;
            }
            println!();
        }
        */
    }

    fn input_operation() -> char {
        let mut input = String::new();
        loop {
            stdin().read_line(&mut input).expect("Faild!");
            match input.trim().chars().next() {
                Some(inputed_char) => return inputed_char,
                None => input.clear()
            }
        }
    }
    
    pub fn start(){
        let mut board = Board::new();
        let (mut y_now, mut x_now): (i32, i32) = (0, 0);
        loop {
            print_board(&board, y_now, x_now);
            match input_operation() {
                'w' => if y_now == 0 {y_now = 7} else {y_now-=1},
                'a' => if x_now == 0 {x_now = 7} else {x_now-=1},
                's' => if y_now == 7 {y_now = 0} else {y_now+=1},
                'd' => if x_now == 7 {x_now = 0} else {x_now+=1},
                'x' => board.put_piece(y_now, x_now),
                'q' => return,
                'e' => return,
                _ => (),
            }

        }
    }
}

fn main() {
    cui_test::start();


    let a:[ Coord; 2] = [Coord { x: 0, y : 4}, Coord{x: 1, y: 0}];
}
