use rand::Rng;

#[derive(Clone)]
pub struct Board {
    pub bit_board: [u64; 2],
    pub black_pieces_count: usize,
    pub white_pieces_count: usize,
    pub turns: usize
}

pub enum PutPieceErr {
    NoValidPlacement,
    Unknown(String)
}

impl Board {

    const BOARD_SIZE: i32 = 8;
    const BLACK: usize = 0;
    const WHITE: usize = 1;

    pub fn new() -> Self {
        Board {
            bit_board: [0x0000000810000000u64,0x0000001008000000u64],
            black_pieces_count: 2usize,
            white_pieces_count: 2usize,
            turns: Board::BLACK
        }
    }

    pub fn clear(&mut self) {
        self.bit_board = [0x0000000810000000u64,0x0000001008000000u64];
        self.black_pieces_count = 2usize;
        self.white_pieces_count = 2usize;
        self.turns = Board::BLACK;
    }

    pub fn put_random_piece(&mut self) -> Result<(), PutPieceErr> {
        let legal_moves = self.put_able();
        if legal_moves == 0 {
            return Err(PutPieceErr::NoValidPlacement);
        }

        let mut bit_indices = [0; 64];
        let mut count = 0;
        let mut moves = legal_moves;
        while moves != 0 {
            let bit_index = moves.trailing_zeros();
            bit_indices[count as usize] = bit_index;
            count += 1;
            moves &= moves - 1; 
        }

        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..count);
        let selected_bit_index = bit_indices[random_index as usize];

        self.put_piece(1 << selected_bit_index)
    }

    pub fn put_eval_zero_simple (&mut self) -> Result<(), PutPieceErr> {
        let legal_moves = self.put_able();
        if legal_moves == 0 {
            return Err(PutPieceErr::NoValidPlacement);
        }
        const SCORES: [i32; 64] = [
            120, -20, 20,  5,  5, 20, -20, 120,
            -20, -40, -5, -5, -5, -5, -40, -20,
             20,  -5, 15,  3,  3, 15,  -5,  20,
              5,  -5,  3,  3,  3,  3,  -5,   5,
              5,  -5,  3,  3,  3,  3,  -5,   5,
             20,  -5, 15,  3,  3, 15,  -5,  20,
            -20, -40, -5, -5,  5, -5, -40, -20,
            120, -20, 20,  5,  5, 20, -20, 120,
        ];

        let mut max_score = i32::MIN;
        let mut moves = legal_moves;
        let mut max_score_index = 0;

        while  moves != 0 {
            let bit_index = moves.trailing_zeros() as usize;
            moves &= moves - 1; // 1番小さい桁の1を0にする。
            let current_score: i32 = SCORES[bit_index];
            if current_score > max_score {
                max_score = current_score;
                max_score_index = bit_index;
            }
        }

        eprintln!("{}", max_score);
        self.put_piece(1 << max_score_index)
    }

    pub fn simplest_eval (&self, turn: usize) -> i32 {
        const SCORES: [i32; 64] = [
            120, -20, 20,  5,  5, 20, -20, 120,
            -20, -40, -5, -5, -5, -5, -40, -20,
             20,  -5, 15,  3,  3, 15,  -5,  20,
              5,  -5,  3,  3,  3,  3,  -5,   5,
              5,  -5,  3,  3,  3,  3,  -5,   5,
             20,  -5, 15,  3,  3, 15,  -5,  20,
            -20, -40, -5, -5, -5, -5, -40, -20,
            120, -20, 20,  5,  5, 20, -20, 120,
        ];

        let mut score = 0;
        let mut p_bit = self.bit_board[turn];
        let mut n_bit = self.bit_board[turn ^ 1];
        while  p_bit != 0 {
            let bit_index = p_bit.trailing_zeros() as usize;
            p_bit &= p_bit - 1; // 1番小さい桁の1を0にする。
            score += SCORES[bit_index];
        }
        while  n_bit != 0 {
            let bit_index = n_bit.trailing_zeros() as usize;
            n_bit &= n_bit - 1; // 1番小さい桁の1を0にする。
            score -= SCORES[bit_index];
        }

        score
    }

    pub fn put_eval_one_simple (&mut self) -> Result<(), PutPieceErr> {
        let legal_moves = self.put_able();
        if legal_moves == 0 {
            return Err(PutPieceErr::NoValidPlacement);
        }

        let mut max_score = i32::MIN;
        let mut max_score_put_place = 0;
        let mut moves = legal_moves;
        while  moves != 0 {
            let mut virt_board = self.clone();
            let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
            moves &= moves - 1; // 最も小さい位のbitを消す
            virt_board.put_piece(put_place)?;   
            let current_score: i32 = virt_board.simplest_eval(self.turns);
            if current_score > max_score {
                max_score = current_score;
                max_score_put_place = put_place;
            }
        }

        eprintln!("{}", max_score);

        self.put_piece(max_score_put_place)

    }

    pub fn put_piece_from_coord(&mut self, y: i32, x: i32) -> Result<(), PutPieceErr> {
        let mask = 1 << y * Board::BOARD_SIZE + x;
        self.put_piece(mask)
    }

    #[inline]
    fn put_piece(&mut self, put_mask: u64) -> Result<(), PutPieceErr> {
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

        let player_board: u64 = self.bit_board[self.turns];
        let opponent_board: u64 = self.bit_board[self.turns ^ 1];
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

        self.bit_board[self.turns] |= put_mask;
        self.bit_board[Board::BLACK] ^= reverse_bit;
        self.bit_board[Board::WHITE] ^= reverse_bit;
        self.turns = self.turns ^ 1;
        Ok(())
    }

    #[inline]
    pub fn put_able(&self) -> u64{
        let blank = !(self.bit_board[Board::BLACK] | self.bit_board[Board::WHITE]);

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