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
}


fn main() {
    let b = Board::new();
    cui_test::print_board(&b);
}
