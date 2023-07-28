
#[derive(Debug)]

pub struct Board {
    black: u64,
    white: u64,
    black_pieces_count: usize,
    white_pieces_count: usize,
    turns: u32
}


impl Board {

    const BOARD_SIZE: i32 = 8;
    const BLACK_TURN: u32 = 0;
    const WHITE_TURN: u32 = 1;

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
    fn put_piece(&mut self, y: i32, x: i32) {
        let mask = 1 << y * Board::BOARD_SIZE + x;

        if self.put_able() & mask == 0 {
            println!("そこには置けません！");
            return;
        }
        if self.turns == Board::BLACK_TURN {
            self.black |= mask;
        }else {
            self.white |= mask;
        }
        self.reverse_piece(mask);
        self.turns = self.turns ^ 1;   
    }

    fn reverse_piece(&mut self, put_mask: u64) {
        let directions: [i32; 4] = [1, 8, 7, 9];
        let masks: [u64; 4] = [
            0x7e7e7e7e7e7e7e7e, // 左右
            0xffffffffffffff00, // 上下
            0x007e7e7e7e7e7e00, // 斜め
            0x007e7e7e7e7e7e00, // 斜め
        ];

        let player_board: u64;
        let opponent_board: u64;
        if self.turns == Board::BLACK_TURN {
            player_board = self.black;
            opponent_board = self.white;
        }else {
            player_board = self.white;
            opponent_board = self.black;
        }

        let mut reverse_board = 0u64;
        for (&direction, &mask) in directions.iter().zip(&masks) {
            let mut shifted_bit = put_mask << direction & mask;
            let mut prev_shifted_bit= 0u64;
            while shifted_bit & opponent_board != 0u64 {
                prev_shifted_bit |= shifted_bit;
                shifted_bit = shifted_bit << direction & mask;
            }
            if shifted_bit & player_board != 0 {
                reverse_board |= prev_shifted_bit;
            }

            // 逆方向
            let mut shifted_bit = put_mask >> direction & mask;
            let mut prev_shifted_bit = 0u64;
            while shifted_bit & opponent_board != 0u64 {
                prev_shifted_bit |= shifted_bit;
                shifted_bit = shifted_bit >> direction & mask;
            }
            if shifted_bit & player_board != 0 {
                reverse_board |= prev_shifted_bit;
            }
        }
        
        self.white ^= reverse_board;
        self.black ^= reverse_board;
    }

    #[inline]
    fn put_able(&self) -> u64{
        let blank = !(self.black | self.white);

        let directions: [i32; 4] = [1, 8, 7, 9];
        let masks: [u64; 4] = [
            0x7e7e7e7e7e7e7e7e, // 左右
            0xffffffffffffff00, // 上下
            0x007e7e7e7e7e7e00, // 斜め
            0x007e7e7e7e7e7e00, // 斜め
        ];

        let player_board: u64;
        let opponent_board: u64;
        if self.turns == Board::BLACK_TURN {
            player_board = self.black;
            opponent_board = self.white;
        }else {
            player_board = self.white;
            opponent_board = self.black;
        }

        let mut legal_moves = 0u64;
        for (direction, mask) in directions.iter().zip(&masks) {
            let mut flipped_positions =  (player_board << *direction) & *mask & opponent_board;
            for _ in 0..5 { // 5回の反復で8マス全てをカバーできます。
                flipped_positions |=  (flipped_positions << *direction) & *mask & opponent_board;
            }
            legal_moves |=  (flipped_positions << *direction) & blank;
            let mut flipped_positions =  (player_board >> *direction) & *mask & opponent_board;
            for _ in 0..5 { // 5回の反復で8マス全てをカバーできます。
                flipped_positions |=  (flipped_positions >> *direction) & *mask & opponent_board;
            }
            legal_moves |=  (flipped_positions >> *direction) & blank;
        }

        legal_moves

    }


}

mod cui_test {

    use crate::Board;
    use std::io::stdin;

    pub fn print_board(board: &Board, y_now: i32, x_now: i32){
        println!("black: {}\nwhite: {}", board.black_pieces_count, board.white_pieces_count);
        for y in 0..8 {
            for x in 0..8 {
                let value: char = {
                    if y == y_now && x == x_now {'*'}
                        else {
                        let mask: u64 = 1 << y * 8 + x;
                        let put_able_bit = board.put_able();

                        if put_able_bit & mask != 0 {'#'}
                        else if board.black & mask != 0 {'x'}
                        else if board.white & mask != 0 {'o'}
                        else {'.'}
                    }
                };
                print!("{} ", value);
            }
            println!();
        }
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

}
