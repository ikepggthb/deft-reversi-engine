pub struct Board {
    black: u64,
    white: u64,
    black_pieces_count: usize,
    white_pieces_count: usize,
    turns: i32
}

#[derive(Debug)]
enum Piece {
    Black,
    White,
    None
}

const BOARD_SIZE: u32 = 8;
impl Board {
    fn new() -> Self {
        Board {
            black: 0x0000000810000000u64,
            white: 0x0000001008000000u64,
            black_pieces_count: 2usize,
            white_pieces_count: 2usize,
            turns: 0
        }
    }

    #[inline]
    fn get_piece(&self, h: u32, w: u32) -> Piece {
        let mask: u64 = 1 << h * BOARD_SIZE + w;

        if self.black & mask != 0 {Piece::Black}
        else if self.white & mask != 0 {Piece::White}
        else {Piece::None}
    }
    

    #[inline]
    fn _get_cow(&self, h: u32, w: u32, count : u32, d: (u32,u32)) -> Piece{
        self.get_piece(h + d.0 * count, w + d.1 * count)
    }
}

mod cui_test {

    use crate::{Board, Piece};
    //use termion::*;
    use std::io::stdin;
    //use termion::input::TermRead;
    //use termion::raw::IntoRawMode;
    pub fn print_board(board: &Board, y_now: u32, x_now: u32){
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
        let board = Board::new();
        let (mut y_now, mut x_now): (i32, i32) = (0, 0);
        loop {
            print_board(&board, y_now as u32, x_now as u32);
            match input_operation() {
                'w' => if y_now == 0 {y_now = 7} else {y_now-=1},
                'a' => if x_now == 0 {x_now = 7} else {x_now-=1},
                's' => if y_now == 7 {y_now = 0} else {y_now+=1},
                'd' => if x_now == 7 {x_now = 0} else {x_now+=1},
                'q' => return,
                _ => (),
            }

        }
    }
}

fn main() {
    cui_test::start();
}
