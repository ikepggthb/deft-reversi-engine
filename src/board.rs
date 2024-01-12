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

    pub fn put_piece_from_coord(&mut self, y: i32, x: i32) -> Result<(), PutPieceErr>
    {
        let mask = 1 << y * Board::BOARD_SIZE + x;
        self.put_piece(mask)
    }

    pub fn put_piece(&mut self, put_mask: u64) -> Result<(), PutPieceErr>
    {
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
    pub fn flip_bit(&self, x: u64) -> u64
    {
        let p: u64 = self.bit_board[self.next_turn];
        let o: u64 = self.bit_board[self.next_turn ^ 1];
        let mut flip = 0u64;

        let maskd = o & 0x7e7e7e7e7e7e7e7e;
        let mut flip1 =  (x << 1) & maskd;
        flip1 |=  (flip1 << 1) & maskd;
        flip1 |=  (flip1 << 1) & maskd;
        flip1 |=  (flip1 << 1) & maskd;
        flip1 |=  (flip1 << 1) & maskd;
        flip1 |=  (flip1 << 1) & maskd;
        flip1 |=  (flip1 << 1) & maskd;
        let outflank = p & (flip1 << 1);
        if outflank == 0 {flip1 = 0};
        flip |= flip1;
        
        // 逆方向
        let mut flip2 =  (x >> 1) & maskd;
        flip2 |=  (flip2 >> 1) & maskd;
        flip2 |=  (flip2 >> 1) & maskd;
        flip2 |=  (flip2 >> 1) & maskd;
        flip2 |=  (flip2 >> 1) & maskd;
        flip2 |=  (flip2 >> 1) & maskd;
        flip2 |=  (flip2 >> 1) & maskd;
        let outflank = p & (flip2 >> 1);
        if outflank == 0 {flip2 = 0};
        flip |= flip2;

        // 上下
        let maskd = o & 0xffffffffffffff00;
        let mut flip1 =  (x << 8) & maskd;
        flip1 |=  (flip1 << 8) & maskd;
        flip1 |=  (flip1 << 8) & maskd;
        flip1 |=  (flip1 << 8) & maskd;
        flip1 |=  (flip1 << 8) & maskd;
        flip1 |=  (flip1 << 8) & maskd;
        flip1 |=  (flip1 << 8) & maskd;
        let outflank = p & (flip1 << 8);
        if outflank == 0 {flip1 = 0};
        flip |= flip1;
        
        // 逆方向
        let mut flip2 =  (x >> 8) & maskd;
        flip2 |=  (flip2 >> 8) & maskd;
        flip2 |=  (flip2 >> 8) & maskd;
        flip2 |=  (flip2 >> 8) & maskd;
        flip2 |=  (flip2 >> 8) & maskd;
        flip2 |=  (flip2 >> 8) & maskd;
        flip2 |=  (flip2 >> 8) & maskd;
        let outflank = p & (flip2 >> 8);
        if outflank == 0 {flip2 = 0};
        flip |= flip2;

        // 斜め
        let maskd = o & 0x007e7e7e7e7e7e00;
        let mut flip1 =  (x << 7) & maskd;
        flip1 |=  (flip1 << 7) & maskd;
        flip1 |=  (flip1 << 7) & maskd;
        flip1 |=  (flip1 << 7) & maskd;
        flip1 |=  (flip1 << 7) & maskd;
        flip1 |=  (flip1 << 7) & maskd;
        flip1 |=  (flip1 << 7) & maskd;
        let outflank = p & (flip1 << 7);
        if outflank == 0 {flip1 = 0};
        flip |= flip1;
        
        // 逆方向
        let mut flip2 =  (x >> 7) & maskd;
        flip2 |=  (flip2 >> 7) & maskd;
        flip2 |=  (flip2 >> 7) & maskd;
        flip2 |=  (flip2 >> 7) & maskd;
        flip2 |=  (flip2 >> 7) & maskd;
        flip2 |=  (flip2 >> 7) & maskd;
        flip2 |=  (flip2 >> 7) & maskd;
        let outflank = p & (flip2 >> 7);
        if outflank == 0 {flip2 = 0};
        flip |= flip2;

        // 斜め 2
        let mut flip1 =  (x << 9) & maskd;
        flip1 |=  (flip1 << 9) & maskd;
        flip1 |=  (flip1 << 9) & maskd;
        flip1 |=  (flip1 << 9) & maskd;
        flip1 |=  (flip1 << 9) & maskd;
        flip1 |=  (flip1 << 9) & maskd;
        flip1 |=  (flip1 << 9) & maskd;
        let outflank = p & (flip1 << 9);
        if outflank == 0 {flip1 = 0};
        flip |= flip1;
        
        // 逆方向
        let mut flip2 =  (x >> 9) & maskd;
        flip2 |=  (flip2 >> 9) & maskd;
        flip2 |=  (flip2 >> 9) & maskd;
        flip2 |=  (flip2 >> 9) & maskd;
        flip2 |=  (flip2 >> 9) & maskd;
        flip2 |=  (flip2 >> 9) & maskd;
        flip2 |=  (flip2 >> 9) & maskd;
        let outflank = p & (flip2 >> 9);
        if outflank == 0 {flip2 = 0};
        flip |= flip2;

        flip
    }

    #[inline(always)]
    pub fn put_piece_fast(&mut self, put_mask: u64)
    {
        // ひっくり返す箇所を計算
        let reverse_bit = self.flip_bit(put_mask);
        
        // 石を置く
        self.bit_board[self.next_turn] |= put_mask;

        // ひっくり返す
        self.bit_board[0] ^= reverse_bit; // BLACK
        self.bit_board[1] ^= reverse_bit; // WHITE

        // 次のターンにする
        self.next_turn = self.next_turn ^ 1;
    }

    #[inline(always)]
    pub fn put_able(&self) -> u64
    {
        let blank = !(self.bit_board[Board::BLACK] | self.bit_board[Board::WHITE]);

        let p: u64 = self.bit_board[self.next_turn];
        let o: u64 = self.bit_board[self.next_turn ^ 1];

        let mut legal_moves = 0u64;

        // 左右
        let maskd = 0x7e7e7e7e7e7e7e7e & o;
        let mut flip =  (p << 1) & maskd;
        flip |=  (flip << 1) & maskd;
        flip |=  (flip << 1) & maskd;
        flip |=  (flip << 1) & maskd;
        flip |=  (flip << 1) & maskd;
        flip |=  (flip << 1) & maskd;
        legal_moves |=  (flip << 1) & blank;
        
        // 逆方向
        let mut flip =  (p >> 1) & maskd;
        flip |=  (flip >> 1) & maskd;
        flip |=  (flip >> 1) & maskd;
        flip |=  (flip >> 1) & maskd;
        flip |=  (flip >> 1) & maskd;
        flip |=  (flip >> 1) & maskd;
        legal_moves |=  (flip >> 1) & blank;


        // 上下
        let maskd = 0xffffffffffffff00 & o;
        let mut flip =  (p << 8) & maskd;
        flip |=  (flip << 8) & maskd;
        flip |=  (flip << 8) & maskd;
        flip |=  (flip << 8) & maskd;
        flip |=  (flip << 8) & maskd;
        flip |=  (flip << 8) & maskd;
        legal_moves |=  (flip << 8) & blank;
        
        // 逆方向
        let mut flip =  (p >> 8) & maskd;
        flip |=  (flip >> 8) & maskd;
        flip |=  (flip >> 8) & maskd;
        flip |=  (flip >> 8) & maskd;
        flip |=  (flip >> 8) & maskd;
        flip |=  (flip >> 8) & maskd;
        legal_moves |=  (flip >> 8) & blank;


        // 斜め
        let maskd = 0x007e7e7e7e7e7e00 & o;
        let mut flip =  (p << 7) & maskd;
        flip |=  (flip << 7) & maskd;
        flip |=  (flip << 7) & maskd;
        flip |=  (flip << 7) & maskd;
        flip |=  (flip << 7) & maskd;
        flip |=  (flip << 7) & maskd;
        legal_moves |=  (flip << 7) & blank;
        
        // 逆方向
        let mut flip =  (p >> 7) & maskd;
        flip |=  (flip >> 7) & maskd;
        flip |=  (flip >> 7) & maskd;
        flip |=  (flip >> 7) & maskd;
        flip |=  (flip >> 7) & maskd;
        flip |=  (flip >> 7) & maskd;
        legal_moves |=  (flip >> 7) & blank;


        // 斜め 2
        let mut flip =  (p << 9) & maskd;
        flip |=  (flip << 9) & maskd;
        flip |=  (flip << 9) & maskd;
        flip |=  (flip << 9) & maskd;
        flip |=  (flip << 9) & maskd;
        flip |=  (flip << 9) & maskd;
        legal_moves |=  (flip << 9) & blank;
        
        // 逆方向
        let mut flip =  (p >> 9) & maskd;
        flip |=  (flip >> 9) & maskd;
        flip |=  (flip >> 9) & maskd;
        flip |=  (flip >> 9) & maskd;
        flip |=  (flip >> 9) & maskd;
        flip |=  (flip >> 9) & maskd;
        legal_moves |=  (flip >> 9) & blank;

        legal_moves

    }


    pub fn get_all_symmetries(&self) -> Vec<Board>
    {
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
    pub fn get_all_rotations(&self) -> Vec<Board>
    {
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
    pub fn move_count(&self) -> i32
    { // 現在何手目まで打たれたか(0~60)
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

    pub fn move_bit_to_str(bit: u64) -> Result<String, String>
    {
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

    #[inline(always)]
    pub fn empties_count(&self) -> i32
    {
        (self.bit_board[0] | self.bit_board[1]).count_zeros() as i32
    }

}