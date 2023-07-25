pub struct Board {
    black: u64,
    white: u64,
    black_pieces_count: usize,
    white_pieces_count: usize,
    turns: i32
}

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
    fn get_valid(self) {
        for row in 0..8 {
            let mut index: i32 = row * 8;
            for col in 0..8 {
                let mask: u64 = 1 << index;
                if self.black & mask == 0 && self.white & mask == 0 {
                    
                    for shift in [-9, -8, -7, -1, 1, 7, 8, 9].iter() {
                        
                    }
                }
                index += 1;
            }
        }
    }
}

mod cui_test {

    use crate::Board;
    use termion::*;
    use std::io::{Write, stdout, stdin};
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    pub fn print_board(board: &Board){
        println!("black: {}\nwhite: {}", board.black_pieces_count, board.white_pieces_count);
        for row in 0..8 {
            let mut index: i32 = row * 8;
            for col in 0..8 {
                let mask: u64 = 1 << index;
                let value: char = 
                if board.black & mask != 0 {
                    'x' 
                } else if board.white & mask != 0 {
                    'o'
                } else {
                    '.'
                };
                print!("{} ", value);
                index += 1;
            }
            println!();
        }
    }
    pub fn start(){
        let board = Board::new();
        let mut count = 0; 
        let mut input = String::new();
        loop {
            input.clear();
            print_board(&board);
            stdin().read_line(&mut input).expect("Faild!");
            let inputed_char = input.trim().chars().next().unwrap();

            match stdin.keys() {
                'w' => (),
                'a' => (),
                's' => (),
                'd' => (),
                'q' => return,
                 x => println!("inputed: {}", x),
            }
        }
    }
}


fn main() {
    cui_test::start();
}
