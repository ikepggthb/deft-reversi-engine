
use termion::color::Black;

use crate::board;
use crate::board::*;
use crate::ai::*;


use crate::bit::*;

// use rand::Rng;

struct LookUpTableTernary {
    lookup_table: Vec<u64>
}

impl LookUpTableTernary {
    const SIZE: usize = 10;
    fn new() -> Self{
        let mut table = vec![0u64; 1<<(LookUpTableTernary::SIZE+LookUpTableTernary::SIZE)];
        for black in 0..(1 << LookUpTableTernary::SIZE) {
            for white in 0..(1 << LookUpTableTernary::SIZE) {
                if black & white == 0 { // 重複がない場合のみ
                    let index = black | (white << LookUpTableTernary::SIZE);
                    table[index as usize] = Self::compute_ternary(black as u64, white as u64);
                }
            }
        };
        Self { lookup_table: table }
    }

    fn compute_ternary(black: u64, white: u64) -> u64 {
        let mut ternary_board: u64 = 0;
        let mut mul = 1;
        for i in 0..LookUpTableTernary::SIZE {
            let mask = 1u64 << i;
            if black & mask != 0 {
                ternary_board += 1 * mul;
            } else if white & mask != 0 {
                ternary_board += 2 * mul;
            }
            mul *= 3;
        }
        ternary_board
    }

    fn fast_bitboard_to_ternary(&self, black: u64, white: u64) -> u64 {
        let index = (black | (white << 10)) & 0xFFFFF;
        self.lookup_table[index as usize]
    }

    
}




pub struct Evaluator{
    eval_from_board_patterns: Vec<Vec<Vec<i64>>>,
    table_ternary: LookUpTableTernary
}



impl Evaluator {
    const EVAL_CHANGE_INTERVAL: usize = 2;
    const PATTERNS_WEIGHTS: [u64; 3] = [1, 1, 1];
    const PATTERN_NUM: usize = 3;

    pub fn new() -> Self {
        if 60 % Evaluator::EVAL_CHANGE_INTERVAL != 0 {
            panic!("EVAL_CHANGE_INTERVAL is inappropriate value.");
        }
         /*
        pattern [30][3][3^10]
        60手を、30に分ける
        確かめるパターンは、3つ
        パターン数は、3通りの箇所が10箇所あるから、3^10
    
    
        ----------
        |........|
        |X.XXXX.X|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        ----------
    
    
        and
    
        ----------
        |X......X|
        |XX....XX|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        |XXXXXXXX|
        ----------
    
        and
    
        ----------
        |.XXXXXXX|
        |X.XXXXXX|
        |XX.XXXXX|
        |XXX.XXXX|
        |XXXX.XXX|
        |XXXXX.XX|
        |XXXXXX.X|
        |XXXXXXX.|
        ----------
        */
        Self { 
            eval_from_board_patterns: vec![vec![vec![0;59049]; Evaluator::PATTERN_NUM]; 60/Evaluator::EVAL_CHANGE_INTERVAL],
            table_ternary: LookUpTableTernary::new()
        }
    }

    fn board_pattern0(bit: u64) -> u64 {
        let bit1 = bit & 0b00000000_11111111;
        let bit2 = (bit >> 1) & 0b00000001_00000000;
        let bit3 = (bit >> 5) & 0b00000010_00000000;    
        bit1 | bit2 | bit3
    }
    fn board_pattern1(bit: u64) -> u64 {
        let bit1 = (bit >> 1) & 0b00000000_00111111;
        let bit2 = (bit >> 4) & 0b00000011_11000000 ;
        bit1 | bit2
    }

    fn board_pattern2(bit: u64) -> u64 {
        let mut board_pattern_bit = 0u64;
        board_pattern_bit |= bit         & 0b00000001;
        board_pattern_bit |= (bit >>  8) & 0b00000010;
        board_pattern_bit |= (bit >> 16) & 0b00000100;
        board_pattern_bit |= (bit >> 24) & 0b00001000;
        board_pattern_bit |= (bit >> 32) & 0b00010000;
        board_pattern_bit |= (bit >> 40) & 0b00100000;
        board_pattern_bit |= (bit >> 48) & 0b01000000;
        board_pattern_bit |= (bit >> 56) & 0b10000000;
        board_pattern_bit
    }
    pub fn eval_from_board_pattern(&self, board: &Board) -> i64 {
        let move_count = board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones() - 4;
        let step = move_count as usize / Evaluator::EVAL_CHANGE_INTERVAL;
        let sym_boards = board.get_all_symmetries();
        let mut eval = 0;
        let mut board_pattern_bit = [[0u64; 2]; 3];

        let make_board_pattern_bit: [fn(u64)->u64; Evaluator::PATTERN_NUM] = [Self::board_pattern0, Self::board_pattern1, Self::board_pattern2];
        for sym_board in sym_boards.iter() {
            for i in 0..Evaluator::PATTERN_NUM {
                board_pattern_bit[i][Board::BLACK] = make_board_pattern_bit[i](sym_board.bit_board[Board::BLACK]);
                board_pattern_bit[i][Board::WHITE] = make_board_pattern_bit[i](sym_board.bit_board[Board::WHITE]);
                let board_pattern = self.table_ternary.fast_bitboard_to_ternary(board_pattern_bit[i][Board::BLACK], board_pattern_bit[i][Board::WHITE]);
                eval += self.eval_from_board_patterns[step][i][board_pattern as usize];
            }
        }

        if board.next_turn ==  Board::WHITE {
            eval = -eval;
        }
        eval
    }
    
    pub fn put_piece_eval_from_board_pattern(&mut self,board: &mut Board) -> Result<(), PutPieceErr>  {
        //let board_pattern = vec![vec![vec![0i64; 59049]; 3]; 30];
    
        let legal_moves = board.put_able();
        if legal_moves == 0 {
            return Err(PutPieceErr::NoValidPlacement);
        }
    
        let mut max_score = i64::MIN;
        let mut max_score_put_place = 0;
        let mut moves = legal_moves;
        while  moves != 0 {
            let mut virt_board = board.clone();
            let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
            moves &= moves - 1; // 最も小さい位のbitを消す
            virt_board.put_piece(put_place)?;   
            let current_score = -self.eval_from_board_pattern(board);
            eprintln!("current_score: {}", current_score);
            if current_score > max_score {
                max_score = current_score;
                max_score_put_place = put_place;
            }
        }
    
        eprintln!("{}", max_score);
    
        board.put_piece(max_score_put_place)
    
    }
    
    
    pub fn learn_eval_from_board_pattern(&mut self) {
        let mut board = Board::new();

        let make_board_pattern_bit: [fn(u64)->u64; Evaluator::PATTERN_NUM] = [Self::board_pattern0, Self::board_pattern1, Self::board_pattern2];
        let mut board_pattern = [[0_u64; Evaluator::PATTERN_NUM];  60];
        for i in 0..60 {
            let put_result = board.put_piece(mid_game_solver_nega_alpha_variation(&board, 4, 3));
            if put_result.is_err() {
                board.next_turn ^= 1;
                let put_result = board.put_piece(mid_game_solver_nega_alpha_variation(&board, 4, 3));
                if put_result.is_err() {
                    break;
                }
            }
            let sym_boards = board.get_all_symmetries();
            for sym_board in sym_boards.iter() {
                for patten in 0..Evaluator::PATTERN_NUM {
                    let board_pattern_black = make_board_pattern_bit[patten](sym_board.bit_board[Board::BLACK]);
                    let board_pattern_white = make_board_pattern_bit[patten](sym_board.bit_board[Board::WHITE]);
                    board_pattern[i][patten] = self.table_ternary.fast_bitboard_to_ternary(board_pattern_black, board_pattern_white);
                }
            }
        }
        let eval = board.bit_board[Board::BLACK].count_ones() as i64 - board.bit_board[Board::WHITE].count_ones() as i64;
        for i in 0..60 {
            let step = i as usize / Evaluator::EVAL_CHANGE_INTERVAL;
            for patten in 0..Evaluator::PATTERN_NUM {
                self.eval_from_board_patterns[step][patten][board_pattern[i][patten] as usize] += eval;
            }
        }
    }

    pub fn learn_eval_from_board_pattern2(&mut self) {
        let mut board = Board::new();

        let make_board_pattern_bit: [fn(u64)->u64; Evaluator::PATTERN_NUM] = [Self::board_pattern0, Self::board_pattern1, Self::board_pattern2];
        let mut board_pattern = [[0_u64; Evaluator::PATTERN_NUM];  60];
        for i in 0..60 {
            let put_result = self.put_piece_eval_from_board_pattern(&mut board);
            if put_result.is_err() {
                board.next_turn ^= 1;
                let put_result = self.put_piece_eval_from_board_pattern(&mut board);
                if put_result.is_err() {
                    break;
                }
            }
            let sym_boards = board.get_all_symmetries();
            for sym_board in sym_boards.iter() {
                for patten in 0..Evaluator::PATTERN_NUM {
                    let board_pattern_black = make_board_pattern_bit[patten](sym_board.bit_board[Board::BLACK]);
                    let board_pattern_white = make_board_pattern_bit[patten](sym_board.bit_board[Board::WHITE]);
                    board_pattern[i][patten] = self.table_ternary.fast_bitboard_to_ternary(board_pattern_black, board_pattern_white);
                }
            }
        }
        let eval = board.bit_board[Board::BLACK].count_ones() as i64 - board.bit_board[Board::WHITE].count_ones() as i64;
        for i in 0..60 {
            let step = i as usize / Evaluator::EVAL_CHANGE_INTERVAL;
            for patten in 0..Evaluator::PATTERN_NUM {
                self.eval_from_board_patterns[step][patten][board_pattern[i][patten] as usize] += eval;
            }
        }
    }
    
}
