use crate::bit::*;

#[derive(Clone)]
pub struct Board {
    pub bit_board: [u64; 2],
    pub next_turn: usize
}

pub enum PutPieceErr {
    NoValidPlacement,
    Unknown(String)
}

impl Board {

    pub const BOARD_SIZE: i32 = 8;
    pub const BLACK: usize = 0;
    pub const WHITE: usize = 1;

    pub fn new() -> Self {
        Board {
            bit_board: [0x0000000810000000u64,0x0000001008000000u64],
            next_turn: Board::BLACK
        }
    }

    pub fn clear(&mut self) {
        self.bit_board = [0x0000000810000000u64,0x0000001008000000u64];
        self.next_turn = Board::BLACK;
    }


    pub fn put_piece_from_coord(&mut self, y: i32, x: i32) -> Result<(), PutPieceErr> {
        let mask = 1 << y * Board::BOARD_SIZE + x;
        self.put_piece(mask)
    }

    pub fn put_piece(&mut self, put_mask: u64) -> Result<(), PutPieceErr> {
        if self.put_able() & put_mask == 0 {
            return Err(PutPieceErr::NoValidPlacement);
        }

        // search reverse bit
        let directions: [i32; 4] = [8, 7, 1, 9];
        
        let masks: [u64; 4] = [
            0xffffffffffffff00,
            0x7f7f7f7f7f7f7f00,
            0xfefefefefefefefe,
            0xfefefefefefefe00
        ];

        let rmasks: [u64; 4] = [
            0x00ffffffffffffff,
            0x00fefefefefefefe,
            0x7f7f7f7f7f7f7f7f,
            0x007f7f7f7f7f7f7f,
            ];

        let player_board: u64 = self.bit_board[self.next_turn];
        let opponent_board: u64 = self.bit_board[self.next_turn ^ 1];
        let mut reverse_bit = 0u64;

        for ((direction, &mask), &rmask) in directions.iter().zip(&masks).zip(&rmasks) {
            let mut shifted_bit = (put_mask << direction) & mask;
            let mut prev_shifted_bit= 0u64;
            while shifted_bit & opponent_board != 0u64 {
                prev_shifted_bit |= shifted_bit;
                shifted_bit = (shifted_bit << direction) & mask;
            }
            if shifted_bit & player_board != 0 {
                reverse_bit |= prev_shifted_bit;
            }

            // 逆方向
            let mut shifted_bit = (put_mask >> direction) & rmask;
            let mut prev_shifted_bit = 0u64;
            while shifted_bit & opponent_board != 0u64 {
                prev_shifted_bit |= shifted_bit;
                shifted_bit = (shifted_bit >> direction) & rmask;
            }
            if shifted_bit & player_board != 0 {
                reverse_bit |= prev_shifted_bit;
            }
        }

        self.bit_board[self.next_turn] |= put_mask;
        self.bit_board[Board::BLACK] ^= reverse_bit;
        self.bit_board[Board::WHITE] ^= reverse_bit;
        self.next_turn = self.next_turn ^ 1;
        Ok(())
    }

    #[inline(always)]
    pub fn reverse_bit(&self, put_mask: u64) -> u64{

        let player_board: u64 = self.bit_board[self.next_turn];
        let opponent_board: u64 = self.bit_board[self.next_turn ^ 1];
        let mut reverse_bit = 0u64;

        // 上下
        let mut shifted_bit = (put_mask << 8) & 0xffffffffffffff00;
        let mut prev_shifted_bit= 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit << 8) & 0xffffffffffffff00;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }
        
        // 逆方向
        let mut shifted_bit = (put_mask >> 8) & 0x00ffffffffffffff;
        let mut prev_shifted_bit = 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit >> 8) & 0x00ffffffffffffff;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }

        // 斜め
        let mut shifted_bit = (put_mask << 7) & 0x7f7f7f7f7f7f7f00;
        let mut prev_shifted_bit= 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit << 7) & 0x7f7f7f7f7f7f7f00;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }
        
        // 逆方向
        let mut shifted_bit = (put_mask >> 7) & 0x00fefefefefefefe;
        let mut prev_shifted_bit = 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit >> 7) & 0x00fefefefefefefe;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }

        // 左右
        let mut shifted_bit = (put_mask << 1) & 0xfefefefefefefefe;
        let mut prev_shifted_bit= 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit << 1) & 0xfefefefefefefefe;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }
        
        // 逆方向
        let mut shifted_bit = (put_mask >> 1) & 0x7f7f7f7f7f7f7f7f;
        let mut prev_shifted_bit = 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit >> 1) & 0x7f7f7f7f7f7f7f7f;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }

        // 斜め 2
        let mut shifted_bit = (put_mask << 9) & 0xfefefefefefefe00;
        let mut prev_shifted_bit= 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit << 9) & 0xfefefefefefefe00;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }
        
        // 逆方向
        let mut shifted_bit = (put_mask >> 9) & 0x007f7f7f7f7f7f7f;
        let mut prev_shifted_bit = 0u64;
        while shifted_bit & opponent_board != 0u64 {
            prev_shifted_bit |= shifted_bit;
            shifted_bit = (shifted_bit >> 9) & 0x007f7f7f7f7f7f7f;
        }
        if shifted_bit & player_board != 0 {
            reverse_bit |= prev_shifted_bit;
        }
        reverse_bit
    }

    #[inline(always)]
    pub fn put_piece_fast(&mut self, put_mask: u64){

        // ひっくり返す箇所を計算
        let reverse_bit = self.reverse_bit(put_mask);
        
        // 石を置く
        self.bit_board[self.next_turn] |= put_mask;

        // ひっくり返す
        self.bit_board[0] ^= reverse_bit; // BLACK
        self.bit_board[1] ^= reverse_bit; // WHITE

        // 次のターンにする
        self.next_turn = self.next_turn ^ 1;
    }

    

    pub fn put_able_old(&self) -> u64{
        let blank = !(self.bit_board[Board::BLACK] | self.bit_board[Board::WHITE]);

        let player_turn = self.next_turn;
        let opponent_turn = self.next_turn ^ 1;

        let player_board: u64 = self.bit_board[player_turn];
        let opponent_board: u64 = self.bit_board[opponent_turn];

        let mut legal_moves = 0u64;

        // 左右
        let direction = 1; let mask = 0x7e7e7e7e7e7e7e7e;
        let mut flipped_positions =  (player_board << direction) & mask & opponent_board;
        for _ in 0..5 { 
            flipped_positions |=  (flipped_positions << direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions << direction) & blank;
        
        // 逆方向
        let mut flipped_positions =  (player_board >> direction) & mask & opponent_board;
        for _ in 0..5 {
            flipped_positions |=  (flipped_positions >> direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions >> direction) & blank;


        // 上下
        let direction = 8; let mask = 0xffffffffffffff00;
        let mut flipped_positions =  (player_board << direction) & mask & opponent_board;
        for _ in 0..5 { 
            flipped_positions |=  (flipped_positions << direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions << direction) & blank;
        
        // 逆方向
        let mut flipped_positions =  (player_board >> direction) & mask & opponent_board;
        for _ in 0..5 {
            flipped_positions |=  (flipped_positions >> direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions >> direction) & blank;


        // 斜め
        let direction = 7; let mask = 0x007e7e7e7e7e7e00;
        let mut flipped_positions =  (player_board << direction) & mask & opponent_board;
        for _ in 0..5 { 
            flipped_positions |=  (flipped_positions << direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions << direction) & blank;
        
        // 逆方向
        let mut flipped_positions =  (player_board >> direction) & mask & opponent_board;
        for _ in 0..5 {
            flipped_positions |=  (flipped_positions >> direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions >> direction) & blank;


        // 斜め
        let direction = 9; let mask = 0x007e7e7e7e7e7e00;
        let mut flipped_positions =  (player_board << direction) & mask & opponent_board;
        for _ in 0..5 { 
            flipped_positions |=  (flipped_positions << direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions << direction) & blank;
        
        // 逆方向
        let mut flipped_positions =  (player_board >> direction) & mask & opponent_board;
        for _ in 0..5 {
            flipped_positions |=  (flipped_positions >> direction) & mask & opponent_board;
        }
        legal_moves |=  (flipped_positions >> direction) & blank;

        legal_moves

    }

    
    #[inline(always)]
    pub fn put_able(&self) -> u64{
        let blank = !(self.bit_board[Board::BLACK] | self.bit_board[Board::WHITE]);

        let player_turn = self.next_turn;
        let opponent_turn = self.next_turn ^ 1;

        let mut legal_moves = 0u64;

        // 左右
        let maskd = 0x7e7e7e7e7e7e7e7e & self.bit_board[opponent_turn];
        let mut flipped_positions =  (self.bit_board[player_turn] << 1) & maskd;
        flipped_positions |=  (flipped_positions << 1) & maskd;
        flipped_positions |=  (flipped_positions << 1) & maskd;
        flipped_positions |=  (flipped_positions << 1) & maskd;
        flipped_positions |=  (flipped_positions << 1) & maskd;
        flipped_positions |=  (flipped_positions << 1) & maskd;
        legal_moves |=  (flipped_positions << 1) & blank;
        
        // 逆方向
        let mut flipped_positions =  (self.bit_board[player_turn] >> 1) & maskd;
        flipped_positions |=  (flipped_positions >> 1) & maskd;
        flipped_positions |=  (flipped_positions >> 1) & maskd;
        flipped_positions |=  (flipped_positions >> 1) & maskd;
        flipped_positions |=  (flipped_positions >> 1) & maskd;
        flipped_positions |=  (flipped_positions >> 1) & maskd;
        legal_moves |=  (flipped_positions >> 1) & blank;


        // 上下
        let maskd = 0xffffffffffffff00 & self.bit_board[opponent_turn];
        let mut flipped_positions =  (self.bit_board[player_turn] << 8) & maskd;
        flipped_positions |=  (flipped_positions << 8) & maskd;
        flipped_positions |=  (flipped_positions << 8) & maskd;
        flipped_positions |=  (flipped_positions << 8) & maskd;
        flipped_positions |=  (flipped_positions << 8) & maskd;
        flipped_positions |=  (flipped_positions << 8) & maskd;
        legal_moves |=  (flipped_positions << 8) & blank;
        
        // 逆方向
        let mut flipped_positions =  (self.bit_board[player_turn] >> 8) & maskd;
        flipped_positions |=  (flipped_positions >> 8) & maskd;
        flipped_positions |=  (flipped_positions >> 8) & maskd;
        flipped_positions |=  (flipped_positions >> 8) & maskd;
        flipped_positions |=  (flipped_positions >> 8) & maskd;
        flipped_positions |=  (flipped_positions >> 8) & maskd;
        legal_moves |=  (flipped_positions >> 8) & blank;


        // 斜め
        let maskd = 0x007e7e7e7e7e7e00 & self.bit_board[opponent_turn];
        let mut flipped_positions =  (self.bit_board[player_turn] << 7) & maskd;
        flipped_positions |=  (flipped_positions << 7) & maskd;
        flipped_positions |=  (flipped_positions << 7) & maskd;
        flipped_positions |=  (flipped_positions << 7) & maskd;
        flipped_positions |=  (flipped_positions << 7) & maskd;
        flipped_positions |=  (flipped_positions << 7) & maskd;
        legal_moves |=  (flipped_positions << 7) & blank;
        
        // 逆方向
        let mut flipped_positions =  (self.bit_board[player_turn] >> 7) & maskd;
        flipped_positions |=  (flipped_positions >> 7) & maskd;
        flipped_positions |=  (flipped_positions >> 7) & maskd;
        flipped_positions |=  (flipped_positions >> 7) & maskd;
        flipped_positions |=  (flipped_positions >> 7) & maskd;
        flipped_positions |=  (flipped_positions >> 7) & maskd;
        legal_moves |=  (flipped_positions >> 7) & blank;


        // 斜め 2
        let mut flipped_positions =  (self.bit_board[player_turn] << 9) & maskd;
        flipped_positions |=  (flipped_positions << 9) & maskd;
        flipped_positions |=  (flipped_positions << 9) & maskd;
        flipped_positions |=  (flipped_positions << 9) & maskd;
        flipped_positions |=  (flipped_positions << 9) & maskd;
        flipped_positions |=  (flipped_positions << 9) & maskd;
        legal_moves |=  (flipped_positions << 9) & blank;
        
        // 逆方向
        let mut flipped_positions =  (self.bit_board[player_turn] >> 9) & maskd;
        flipped_positions |=  (flipped_positions >> 9) & maskd;
        flipped_positions |=  (flipped_positions >> 9) & maskd;
        flipped_positions |=  (flipped_positions >> 9) & maskd;
        flipped_positions |=  (flipped_positions >> 9) & maskd;
        flipped_positions |=  (flipped_positions >> 9) & maskd;
        legal_moves |=  (flipped_positions >> 9) & blank;

        legal_moves

    }


    pub fn get_all_symmetries(&self) -> Vec<Board> {
        let mut symmetries = Vec::new();

        for i in 0b0000..0b1000 { // 2^3 = 8 different combinations
            let mut sym_board = self.clone();
            if (i & 0b0001) != 0 {
                sym_board.bit_board[Board::BLACK] = horizontal_mirror(sym_board.bit_board[Board::BLACK]);
                sym_board.bit_board[Board::WHITE] = horizontal_mirror(sym_board.bit_board[Board::WHITE]);
            }
            if (i & 0b0010) != 0 {
                sym_board.bit_board[Board::BLACK] = vertical_mirror(sym_board.bit_board[Board::BLACK]);
                sym_board.bit_board[Board::WHITE] = vertical_mirror(sym_board.bit_board[Board::WHITE]);
            }
            if (i & 0b0100) != 0 {
                sym_board.bit_board[Board::BLACK] = transpose(sym_board.bit_board[Board::BLACK]);
                sym_board.bit_board[Board::WHITE] = transpose(sym_board.bit_board[Board::WHITE]);
            }
            symmetries.push(sym_board);
        }
        symmetries
    }
    pub fn get_all_rotations(&self) -> Vec<Board> {
        let mut rotations = Vec::new();

        let no_rotation = self.clone();
        rotations.push(no_rotation);

        let mut rotate_90_degrees = self.clone();
        rotate_90_degrees.bit_board[Board::BLACK] = vertical_mirror(rotate_90_degrees.bit_board[Board::BLACK]);
        rotate_90_degrees.bit_board[Board::WHITE] = vertical_mirror(rotate_90_degrees.bit_board[Board::WHITE]);
        rotate_90_degrees.bit_board[Board::BLACK] = transpose(rotate_90_degrees.bit_board[Board::BLACK]);
        rotate_90_degrees.bit_board[Board::WHITE] = transpose(rotate_90_degrees.bit_board[Board::WHITE]);
        rotations.push(rotate_90_degrees);

        let mut rotate_180_degrees = self.clone();
        rotate_180_degrees.bit_board[Board::BLACK] = vertical_mirror(rotate_180_degrees.bit_board[Board::BLACK]);
        rotate_180_degrees.bit_board[Board::WHITE] = vertical_mirror(rotate_180_degrees.bit_board[Board::WHITE]);
        rotate_180_degrees.bit_board[Board::BLACK] = horizontal_mirror(rotate_180_degrees.bit_board[Board::BLACK]);
        rotate_180_degrees.bit_board[Board::WHITE] = horizontal_mirror(rotate_180_degrees.bit_board[Board::WHITE]);
        rotations.push(rotate_180_degrees);

        let mut rotate_270_degrees = self.clone();
        rotate_270_degrees.bit_board[Board::BLACK] = horizontal_mirror(rotate_270_degrees.bit_board[Board::BLACK]);
        rotate_270_degrees.bit_board[Board::WHITE] = horizontal_mirror(rotate_270_degrees.bit_board[Board::WHITE]);
        rotate_270_degrees.bit_board[Board::BLACK] = transpose(rotate_270_degrees.bit_board[Board::BLACK]);
        rotate_270_degrees.bit_board[Board::WHITE] = transpose(rotate_270_degrees.bit_board[Board::WHITE]);
        rotations.push(rotate_270_degrees);

        rotations
    }

    #[inline(always)]
    pub fn move_count(&self) -> i32{ // 現在何手目まで打たれたか(0~60)
        (self.bit_board[Board::BLACK] | self.bit_board[Board::WHITE]).count_ones() as i32 - 4
    }

    pub fn print_board(&self) {
        for y in 0..8 {
            for x in 0..8 {
                let mask = 1u64 << y * 8 + x;
                if self.bit_board[Board::BLACK] & mask != 0 {
                    print!("X");
                } else if self.bit_board[Board::WHITE] & mask != 0 {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    pub fn move_bit_to_str(bit: u64) -> Result<String, String> {
        for y in 0..8 {
            for x in 0..8 {
                let mask = 1u64 << y * 8 + x;
                if mask == bit {
                    let mut result = String::new();
                    match x {
                        0 => result.push('a'),
                        1 => result.push('b'),
                        2 => result.push('c'),
                        3 => result.push('d'),
                        4 => result.push('e'),
                        5 => result.push('f'),
                        6 => result.push('g'),
                        7 => result.push('h'),
                        _ => {}
                    }
                    result.push_str((y+1).to_string().as_str());
                    return Ok(result);
                }
            }
        }

        let error_message = format!("put_place is undefind. (bit = {:0x})", bit);
        return Err(error_message);
    }

    #[inline(always)]
    fn piece_count(&self) -> i32
    {
        (self.bit_board[0] | self.bit_board[1]).count_ones() as i32
    }


}