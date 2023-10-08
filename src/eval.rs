
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

use crate::board;
use crate::board::*;
use crate::ai::*;

// use crate::bit::*;

use std::fs::File;
use std::io::{BufWriter, Write, Error, BufRead, BufReader};
use rand::Rng;

// use rand::Rng;

struct LookUpTableTernary {
    lookup_table: Vec<u64>
}

impl LookUpTableTernary {
    // 入力するbit数
    const BIT_SIZE: usize = 10;

    fn new() -> Self{
        let mut table = vec![0u64; 1<<(LookUpTableTernary::BIT_SIZE+LookUpTableTernary::BIT_SIZE)];
        for black in 0..(1 << LookUpTableTernary::BIT_SIZE) {
            for white in 0..(1 << LookUpTableTernary::BIT_SIZE) {
                if black & white == 0 { // 重複がない場合のみ (黒と白の2つのビットボードのビットは重複しない)
                    let index = black | (white << LookUpTableTernary::BIT_SIZE);
                    table[index as usize] = Self::compute_ternary(black as u64, white as u64);
                }
            }
        };
        Self { lookup_table: table }
    }


    fn compute_ternary(black: u64, white: u64) -> u64 {
        let mut ternary_board: u64 = 0;
        let mut mul = 1;
        for i in 0..LookUpTableTernary::BIT_SIZE {
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

    #[inline(always)]
    fn fast_bitboard_to_ternary(&self, black: u64, white: u64) -> u64 {
        let index = (black | (white << LookUpTableTernary::BIT_SIZE)) & 0xFFFFF;
        self.lookup_table[index as usize]
    }

    
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

fn board_pattern3(bit: u64) -> u64 { // corner
    let bit1 = bit & 0b00000111;
    let bit2 = (bit >> 5) & 0b00111000;
    let bit3 = (bit >> 10) & 0b1_11000000;
    bit1 | bit2 | bit3
}

fn board_pattern4(bit: u64) -> u64 { // corner
    bit & 0b11111111
}

fn board_pattern5(bit: u64) -> u64 { // corner
    (bit >> 8) & 0b11111111
}

fn board_pattern6(bit: u64) -> u64 { // corner
    (bit >> 16) & 0b11111111
}


type PatternFunction = fn(u64) -> u64;
const PATTERN_NUM: usize = 7;
const PATTERN_FUNCTIONS: [PatternFunction; PATTERN_NUM] = [
    board_pattern0,
    board_pattern1,
    board_pattern2,
    board_pattern3,
    board_pattern4,
    board_pattern5,
    board_pattern6
];

pub struct Evaluator {
    lookup_table: LookUpTableTernary,
    eval_feature_table: Vec<Vec<Vec<f64>>>,
    eval_constant_term: Vec<f64>
}

impl Evaluator {
    pub fn new() -> Self{
        Self { 
            lookup_table: LookUpTableTernary::new(),
            // 59049 = 3^9
            // 評価値 = eval_feature_table[何手目][評価1][ボードパターン(3進数)] + eval_feature_table[何手目][評価2][ボードパターン] ... + eval_定数項[何手目];
            eval_feature_table: vec![vec![vec![0.0;59049];PATTERN_NUM];61],    
            eval_constant_term: vec![0.0;61]
        }
    }

    fn board_pattern0(bit: u64) -> u64 { // edge + 2X
        let bit1 = bit & 0b00000000_11111111;
        let bit2 = (bit >> 1) & 0b00000001_00000000;
        let bit3 = (bit >> 5) & 0b00000010_00000000;    
        bit1 | bit2 | bit3
    }
    fn eval(&self, board: &Board) -> f64 {

        let mut result = 0f64;
        let move_count = board.move_count();
        for rotated_board in board.get_all_rotations() {

            for (i, func) in PATTERN_FUNCTIONS.iter().enumerate() {
                let pattern_player = func(rotated_board.bit_board[board.next_turn]);
                let pattern_opponent = func(rotated_board.bit_board[board.next_turn ^ 1]);
                let pattern_ternary = self.lookup_table.fast_bitboard_to_ternary(pattern_player, pattern_opponent);
                result += self.eval_feature_table[move_count as usize][i][pattern_ternary as usize];
            }
        }

        result + self.eval_constant_term[move_count as usize]
    }

    pub fn learn(&mut self, learn_move_count: i32) {
        let mut board = Board::new();

        loop {
            if board.move_count() == learn_move_count {
                break;
            }

            let result = put_random_piece(&mut board);
            if result.is_err() {
                board.next_turn ^= 1;
                if board.put_able() == 0 {
                    return self.learn(learn_move_count);
                }
            }
        }


        let mut correct_score = end_game_full_solver_nega_alpha_move_ordering_return_detail(&board).1 as f64;
        let mut eval = self.eval(&board);

        let error = correct_score - eval;
        let learning_rate = 0.0002;

        for rotated_board in board.get_all_rotations() {

            for (i, func) in PATTERN_FUNCTIONS.iter().enumerate() {
                let pattern_player = func(rotated_board.bit_board[board.next_turn]);
                let pattern_opponent = func(rotated_board.bit_board[board.next_turn ^ 1]);
                let pattern_ternary = self.lookup_table.fast_bitboard_to_ternary(pattern_player, pattern_opponent);
                self.eval_feature_table[learn_move_count as usize][i][pattern_ternary as usize] += 2f64 * error * learning_rate / PATTERN_NUM as f64 / 4.0 ;
            }
        }
        
        self.eval_constant_term[learn_move_count as usize] += 2f64 * error * learning_rate / 5000f64 / 4f64;

    }

    pub fn learn_debug(&mut self, learn_move_count: i32) -> f64 {
        let mut board = Board::new();

        loop {
            if board.move_count() == learn_move_count {
                break;
            }

            let result = put_random_piece(&mut board);
            if result.is_err() {
                board.next_turn ^= 1;
                if board.put_able() == 0 {
                    return self.learn_debug(learn_move_count);
                }
            }
        }

        // if board.next_turn == Board::BLACK {
        //     return self.learn_debug(learn_move_count);
        // }


        let mut correct_score = end_game_full_solver_nega_alpha_move_ordering_return_detail(&board).1 as f64;
        let mut eval = self.eval(&board);
        let error = correct_score - eval;
        // board.print_board();
        println!("turn: {},定数項: {}, 評価値: {}, 理論値: {}",board.next_turn , self.eval_constant_term[learn_move_count as usize], eval, correct_score );

        println!("評価値と教師データの差: {}, 誤差: {}", error, error*error);

        error.abs()

    }
    
}

/*

評価値 = 評価[何手目][評価1][ボードパターン(3進数)] + 評価[何手目][評価2][ボードパターン] ... + 評価_定数項[何手目];

*/