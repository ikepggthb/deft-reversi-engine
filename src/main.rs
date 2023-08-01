
#[derive(Debug)]

pub struct Board {
    bit_board: [u64; 2],
    black_pieces_count: usize,
    white_pieces_count: usize,
    turns: usize
}


impl Board {

    const BOARD_SIZE: i32 = 8;
    const BLACK_TURN: usize = 0;
    const WHITE_TURN: usize = 1;

    fn new() -> Self {
        Board {
            bit_board: [0x0000000810000000u64,0x0000001008000000u64],
            black_pieces_count: 2usize,
            white_pieces_count: 2usize,
            turns: Board::BLACK_TURN
        }
    }

    #[inline]
    fn put_piece(&mut self, y: i32, x: i32) {
        let mask = 1 << y * Board::BOARD_SIZE + x;

        if self.put_able() & mask == 0 {
            return;
        }
        self.bit_board[self.turns] |= mask;
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
        let player_board: u64 = self.bit_board[self.turns];
        let opponent_board: u64 = self.bit_board[self.turns ^ 1];

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
        
        self.bit_board[0] ^= reverse_board;
        self.bit_board[1] ^= reverse_board;
    }

    #[inline]
    fn put_able(&self) -> u64{
        let blank = !(self.bit_board[0] | self.bit_board[1]);

        let directions: [i32; 4] = [1, 8, 7, 9];
        let masks: [u64; 4] = [
            0x7e7e7e7e7e7e7e7e, // 左右
            0xffffffffffffff00, // 上下
            0x007e7e7e7e7e7e00, // 斜め
            0x007e7e7e7e7e7e00, // 斜め
        ];
        let player_board: u64 = self.bit_board[self.turns];
        let opponent_board: u64 = self.bit_board[self.turns ^ 1];

        let mut legal_moves = 0u64;
        for (direction, mask) in directions.iter().zip(&masks) {
            let mut flipped_positions =  (player_board << *direction) & *mask & opponent_board;
            for _ in 0..5 { 
                flipped_positions |=  (flipped_positions << *direction) & *mask & opponent_board;
            }
            legal_moves |=  (flipped_positions << *direction) & blank;
            
            // 逆方向
            let mut flipped_positions =  (player_board >> *direction) & *mask & opponent_board;
            for _ in 0..5 {
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


    pub fn print_board(board: &Board, y_now: i32, x_now: i32, stdout: &mut AlternateScreen<raw::RawTerminal<std::io::Stdout>>) -> std::io::Result<()>{
        write!(stdout, "{}", clear::All)?;
        write!(stdout, "{}", cursor::Goto(1, 1))?;
        write!(stdout, "black: {}\n", board.black_pieces_count)?;
        write!(stdout, "{}", cursor::Goto(1, 2))?;
        write!(stdout, "white: {}\n", board.white_pieces_count)?;
        write!(stdout, "{}", cursor::Goto(1, 3))?;
        for y in 0..8 {
            for x in 0..8 {
                let value: char = {
                    if y == y_now && x == x_now {'*'}
                    else {
                        let mask: u64 = 1 << y * 8 + x;
                        let put_able_bit = board.put_able();
                        
                        if put_able_bit & mask != 0 {'#'}
                        else if board.bit_board[0] & mask != 0 {'x'}
                        else if board.bit_board[1] & mask != 0 {'o'}
                        else {'.'}
                    }
                };
                write!(stdout, "{} ", value)?;
            }

            write!(stdout, "\n")?;
            write!(stdout, "{}", cursor::Goto(1, y as u16 +4))?;
            
        }
        stdout.flush()?;
        Ok(())
    }

    extern crate termion;

    use std::io::{stdout, Write};
    use termion::*;
    use termion::event::{Event, Key};
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use termion::screen::AlternateScreen;
    pub fn start() -> std::io::Result<()>{
        // if let Event::Key(KeyEvent { code, .. }) = event::read()? {
        //     println!("{:?}", code);
        // }
        let mut stdin = stdin();
        let mut stdout = 
            AlternateScreen::from( stdout().into_raw_mode().unwrap());
        write!(stdout, "{}", clear::All)?;
        write!(stdout, "{}", cursor::Goto(1, 1))?;



        write!(stdout, "Hello World!")?;
        stdout.flush()?;

        let mut board = Board::new();
        let (mut y_now, mut x_now): (i32, i32) = (0, 0);
        print_board(&board, y_now, x_now, &mut stdout);

        for evt in stdin.events() {
            match evt.unwrap() {
                // Ctrl-cでプログラム終了
                Event::Key(Key::Ctrl('c')) =>  {
                    return Ok(());
                }
                Event::Key(Key::Char(key_char)) => {
                    match key_char {
                        'w' => if y_now == 0 {y_now = 7} else {y_now-=1},
                        'a' => if x_now == 0 {x_now = 7} else {x_now-=1},
                        's' => if y_now == 7 {y_now = 0} else {y_now+=1},
                        'd' => if x_now == 7 {x_now = 0} else {x_now+=1},
                        'x' => board.put_piece(y_now, x_now),
                        'q' => return Ok(()),
                        _ => ()
                    }
                }
                _ => ()
            }
            print_board(&board, y_now, x_now, &mut stdout);
        }
        // loop {
        //     print_board(&board, y_now, x_now);
        //     match input_operation() {
        //         'q' => return,
        //         'e' => return,
        //         _ => (),
        //     }

        // }


        Ok(())
    }

}

fn main() -> std::io::Result<()> {
    cui_test::start()?;
    Ok(())
}
