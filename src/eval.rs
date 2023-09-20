
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use termion::color::Black;

use crate::board;
use crate::board::*;
use crate::ai::*;


use crate::bit::*;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write, Error, BufRead, BufReader,self};
use rand::Rng;

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
    pub eval_from_board_patterns: Vec<Vec<Vec<i64>>>,
    table_ternary: LookUpTableTernary,
    make_board_pattern_bit: [fn(u64)->u64; Evaluator::PATTERN_NUM],
    rng: XorShiftRng
}



impl Evaluator {
    pub const EVAL_CHANGE_INTERVAL: usize = 2;
    pub const PATTERNS_WEIGHTS: [u64; 3] = [1, 1, 1];
    pub const PATTERN_NUM: usize = 4;

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
            eval_from_board_patterns: vec![vec![vec![0;59049]; Evaluator::PATTERN_NUM]; 60/Evaluator::EVAL_CHANGE_INTERVAL + 1],
            table_ternary: LookUpTableTernary::new(),
            make_board_pattern_bit:  [Self::board_pattern0, Self::board_pattern1, Self::board_pattern2, Self::board_pattern3],
            rng : XorShiftRng::from_entropy()
        }
    }
    pub fn save_to_file(&self, filename: &str) -> Result<(), Error> {
        let mut file = File::create(filename)?;
        let mut writer = BufWriter::new(file);


        for patterns in self.eval_from_board_patterns.iter(){
            for pattern in patterns.iter(){
                for eval in pattern.iter(){
                    writeln!(&mut writer, "{}", *eval)?;
                }
                writeln!(&mut writer, "")?;
            }
            writeln!(&mut writer, "")?;
        }
        writer.flush()?;  
        Ok(())
    }
    pub fn load_from_file(&mut self,filename: &str) -> Result<(), Error> {
        // todo: debug

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut eval_from_board_patterns = vec![
            vec![vec![0; 59049]; Evaluator::PATTERN_NUM];
            60 / Evaluator::EVAL_CHANGE_INTERVAL + 1
        ];

        let mut outer_idx = 0;
        let mut middle_idx = 0;
        let mut inner_idx = 0;

        for line in reader.lines() {
            let line = line?;

            if line.is_empty() {
                if inner_idx != 0 {
                    middle_idx += 1;
                    inner_idx = 0;
                } else {
                    outer_idx += 1;
                    middle_idx = 0;
                }
                continue;
            }

            self.eval_from_board_patterns[outer_idx][middle_idx][inner_idx] = line.parse::<i64>().unwrap();
            inner_idx += 1;
        }

        // ここで他のメンバも初期化してください。必要に応じて変更してください。
        Ok(())
    }


    fn board_pattern0(bit: u64) -> u64 { // edge + 2X
        let bit1 = bit & 0b00000000_11111111;
        let bit2 = (bit >> 1) & 0b00000001_00000000;
        let bit3 = (bit >> 5) & 0b00000010_00000000;    
        bit1 | bit2 | bit3
    }
    fn board_pattern1(bit: u64) -> u64 { // corner+block
        let bit1 = bit & 0b00000001;
        let bit2 = (bit >> 1) & 0b00011110;
        let bit3 = (bit >> 2) & 0b00100000;
        let bit4 = (bit >> 4) & 0b00000011_11000000 ;
        bit1 | bit2 | bit3 | bit4
    }

    fn board_pattern2(bit: u64) -> u64 { // X line
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

    fn board_pattern3(bit: u64) -> u64 { // X line
        let bit1 = bit & 0b00000111;
        let bit2 = (bit >> 5) & 0b00111000;
        let bit3 = (bit >> 10) & 0b1_11000000;
        bit1 | bit2 | bit3
    }
    pub fn eval_from_board_pattern(&self, board: &Board) -> i64 {
        let move_count = board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones() - 4;
        let step = move_count as usize / Evaluator::EVAL_CHANGE_INTERVAL;
        let sym_boards = board.get_all_symmetries();
        let mut eval = 0;
        //let mut board_pattern_bit = [[0u64; 2]; 3];

        for sym_board in sym_boards.iter() {
            for i in 0..Evaluator::PATTERN_NUM {
                let board_pattern_bit_black = self.make_board_pattern_bit[i](sym_board.bit_board[Board::BLACK]);
                let board_pattern_bit_white = self.make_board_pattern_bit[i](sym_board.bit_board[Board::WHITE]);
                let board_pattern = self.table_ternary.fast_bitboard_to_ternary(board_pattern_bit_black, board_pattern_bit_white);
                eval += self.eval_from_board_patterns[step][i][board_pattern as usize];
            }
        }

        if board.next_turn ==  Board::WHITE {
            eval = -eval;
        }

        eval /= 8;

        eval
    }

    pub fn eval_from_board_pattern_for_learn(&self, board: &Board) -> i64 {
        let move_count = board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones() - 4;
        let step = move_count as usize / Evaluator::EVAL_CHANGE_INTERVAL;
        let sym_boards = board.get_all_symmetries();
        let mut eval = 0;

        for sym_board in sym_boards.iter() {
            for i in 0..Evaluator::PATTERN_NUM {
                let board_pattern_bit_black = self.make_board_pattern_bit[i](sym_board.bit_board[Board::BLACK]);
                let board_pattern_bit_white = self.make_board_pattern_bit[i](sym_board.bit_board[Board::WHITE]);
                let board_pattern = self.table_ternary.fast_bitboard_to_ternary(board_pattern_bit_black, board_pattern_bit_white);
                eval += self.eval_from_board_patterns[step][i][board_pattern as usize];
            }
        }

        if board.next_turn ==  Board::WHITE {
            eval = -eval;
        }

        eval
    }
    
    pub fn put_piece_eval_from_board_pattern_for_learn(&self,board: &mut Board) -> Result<(), PutPieceErr>  {
    
        let legal_moves = board.put_able();
        if legal_moves == 0 {
            return Err(PutPieceErr::NoValidPlacement);
        }

        let mut move_scores = Vec::new();
        
        let mut moves = legal_moves;
        while  moves != 0 {
            let mut virt_board = board.clone();
            let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
            moves &= moves - 1; // 最も小さい位のbitを消す
            virt_board.put_piece(put_place)?;  
            let current_score = -self.eval_from_board_pattern_for_learn(&virt_board);
             move_scores.push((current_score, put_place));
        }


        move_scores.sort_unstable_by(|(a,_), (b,_)| b.cmp(a));
        let max_score = move_scores[0].0;
        let variation = (max_score / 8).abs();
        let lower_bound = max_score - variation; // Ensure no underflow
        let mut candidate_count = 1;
        for (i,&(score,_)) in move_scores.iter().enumerate() {
            if score < lower_bound {
                candidate_count = i;
                break;
            }
        }
        //let mut rng = rand::thread_rng();
        let random_index =  if candidate_count == 1 {0} else {
            let mut rng = self.rng.clone();
            rng.gen_range(0..candidate_count)
        };


        board.put_piece( move_scores[random_index].1)
    
    }
        
    
    pub fn learn_eval_from_board_pattern(&mut self) {
        let mut board = Board::new();
        let put_ai = |board: &mut Board | -> Result<(), PutPieceErr> {
            let depth_search =
                64 - (board.bit_board[0].count_ones() + board.bit_board[1].count_ones());
            if depth_search <= 6 {
                board.put_piece(end_game_full_solver_nega_alpha_move_ordering(&board))
            } else {
                board.put_piece(mid_game_solver_nega_alpha_variation(&board, 6, 3))
            }
        };
        let mut board_pattern = [[0_u64; Evaluator::PATTERN_NUM];  61];
        for i in 1..=60 {
            let put_result = put_ai(&mut board);
            if put_result.is_err() {
                board.next_turn ^= 1;
                let put_result = put_ai(&mut board);
                if put_result.is_err() {
                    break;
                }
            }
            let sym_boards = board.get_all_symmetries();
            for sym_board in sym_boards.iter() {
                for pattern in 0..Evaluator::PATTERN_NUM {
                    let board_pattern_black = self.make_board_pattern_bit[pattern](sym_board.bit_board[Board::BLACK]);
                    let board_pattern_white = self.make_board_pattern_bit[pattern](sym_board.bit_board[Board::WHITE]);
                    board_pattern[i][pattern] = self.table_ternary.fast_bitboard_to_ternary(board_pattern_black, board_pattern_white);
                }
            }
        }
        let eval = board.bit_board[Board::BLACK].count_ones() as i64 - board.bit_board[Board::WHITE].count_ones() as i64;
        for i in 1..=60 {
            let step = i as usize / Evaluator::EVAL_CHANGE_INTERVAL;
            for pattern in 0..Evaluator::PATTERN_NUM {
                self.eval_from_board_patterns[step][pattern][board_pattern[i][pattern] as usize] += eval;
            }
        }
    }

    pub fn learn_eval_from_board_pattern2(&mut self) {
        let mut board = Board::new();
        let mut board_pattern = [[0_u64; Evaluator::PATTERN_NUM];  61];

        let mut put_ai = |board: &mut Board | -> Result<(), PutPieceErr> {
            if board.next_turn == Board::BLACK {
                board.put_piece(mid_game_solver_nega_alpha_variation(&board, 4, 3))
            } else {
                self.put_piece_eval_from_board_pattern_for_learn(board)
            }
        };


        for i in 1..=60 {
            let put_result = put_ai( &mut board );
            if put_result.is_err() {
                board.next_turn ^= 1;
                let put_result = put_ai( &mut board );
                if put_result.is_err() {
                    break;
                }
            }
            let sym_boards = board.get_all_symmetries();
            for sym_board in sym_boards.iter() {
                for pattern in 0..Evaluator::PATTERN_NUM {
                    let board_pattern_black = self.make_board_pattern_bit[pattern](sym_board.bit_board[Board::BLACK]);
                    let board_pattern_white = self.make_board_pattern_bit[pattern](sym_board.bit_board[Board::WHITE]);
                    board_pattern[i][pattern] = self.table_ternary.fast_bitboard_to_ternary(board_pattern_black, board_pattern_white);
                }
            }
        }
        let eval = board.bit_board[Board::BLACK].count_ones() as i64 - board.bit_board[Board::WHITE].count_ones() as i64;
        for i in 1..=60 {
            let step = i as usize / Evaluator::EVAL_CHANGE_INTERVAL;
            for pattern in 0..Evaluator::PATTERN_NUM {
                self.eval_from_board_patterns[step][pattern][board_pattern[i][pattern] as usize] += eval;
            }
        }
    }

    pub fn learn_eval_from_board_pattern3(&mut self) {
        let mut board = Board::new();

        let put_ai = |board: &mut Board | -> Result<(), PutPieceErr> {
            let depth_search =
                64 - (board.bit_board[0].count_ones() + board.bit_board[1].count_ones());
            if depth_search <= 3 {
                board.put_piece(end_game_full_solver_nega_alpha_move_ordering(&board))
            } else {
                self.put_piece_eval_from_board_pattern_for_learn(board)
            }
        };

        let mut board_pattern = [[0_u64; Evaluator::PATTERN_NUM];  61];
        for i in 1..=60 {
            let put_result = put_ai(&mut board);
            if put_result.is_err() {
                board.next_turn ^= 1;
                let put_result = put_ai(&mut board);
                if put_result.is_err() {
                    break;
                }
            }
            let sym_boards = board.get_all_symmetries();
            for sym_board in sym_boards.iter() {
                for pattern in 0..Evaluator::PATTERN_NUM {
                    let board_pattern_black = self.make_board_pattern_bit[pattern](sym_board.bit_board[Board::BLACK]);
                    let board_pattern_white = self.make_board_pattern_bit[pattern](sym_board.bit_board[Board::WHITE]);
                    board_pattern[i][pattern] = self.table_ternary.fast_bitboard_to_ternary(board_pattern_black, board_pattern_white);
                }
            }
        }
        let eval = board.bit_board[Board::BLACK].count_ones() as i64 - board.bit_board[Board::WHITE].count_ones() as i64;
        for i in 1..=60 {
            let step = i as usize / Evaluator::EVAL_CHANGE_INTERVAL;
            for pattern in 0..Evaluator::PATTERN_NUM {
                self.eval_from_board_patterns[step][pattern][board_pattern[i][pattern] as usize] += eval;
            }
        }
    }
    


        // ------------------------------------------------------------------------------------------------------------------------------------------------------------


}
