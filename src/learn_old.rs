use crate::board::*;
use crate::eval::*;
use crate::ai::*;

use std::fs::File;
use std::io::{BufReader, BufRead, Error};

pub struct LearnEvaluation{
    records: Vec<Vec<Board>>,
    eval: Evaluator,
    correct_eval: Vec<i64>,
}

impl LearnEvaluation {
     pub fn new() -> Self {
        Self { 
            records: Vec::new(),
            eval: Evaluator::new(),
            correct_eval: Vec::new(),
        }
    }

    pub fn move_str_to_bit(&self, move_chars: &[char]) -> u64 {
        let x = match move_chars[0] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("error: input record")
        };
        let y = move_chars[1].to_digit(10).unwrap() as i32;
        
        1u64 << y * Board::BOARD_SIZE + x
    }

    pub fn print_board(&self, board: &Board) {
        for y in 0..8 {
            for x in 0..8 {
                let mask = 1u64 << y * Board::BOARD_SIZE + x;
                if board.bit_board[Board::BLACK] & mask != 0 {
                    print!("1");
                } else if board.bit_board[Board::WHITE] & mask != 0  {
                    print!("0");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
    pub fn input_record(&mut self, file_name: &str)  -> Result<(), Error>  {
        let path = file_name;
        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        for line in buffered.lines() {
            let mut record: Vec<Board>  = Vec::with_capacity(60);
            let line: Vec<char> = line.unwrap().chars().collect();
            let mut board = Board::new();
            for i in (0..line.len()).step_by(2) {
                let bit = self.move_str_to_bit(&line[i..i+2]);
                //board.put_piece_fast(bit);
                board.put_piece(bit);
                //self.print_board(&board);
                record.push(board.clone());
            }
            self.records.push(record);
        }
        Ok(())
    }


    pub fn sum_errors_eval_weight(&self, learn_data_num: usize, move_count: usize) -> i64{
        let mut sum_errors = 0;
        for learn_data_count in 0..learn_data_num {
            sum_errors =  self.correct_eval[learn_data_count] - self.eval.eval_from_board_pattern(&self.records[learn_data_count][move_count]);
        }

        sum_errors   
    }

    pub fn learn(&mut self) {
        let learn_data_num = 10000;
        for start_move_count in (0..60).step_by(Evaluator::EVAL_CHANGE_INTERVAL) {
            let step = start_move_count / Evaluator::EVAL_CHANGE_INTERVAL;
            let d = vec![vec![0; 59049]; 3];
            let sum_errors = {
                let mut sum = 0;
                for move_count in start_move_count..(start_move_count+Evaluator::EVAL_CHANGE_INTERVAL){
                    self.sum_errors_eval_weight(learn_data_num, move_count);
                }
                sum
            };

            let alpha = 2 / (learn_data_num + Evaluator::EVAL_CHANGE_INTERVAL);

            for i in 0..learn_data_num {
                
            }
        }
    }

    // pub fn eb(&self, w: &Vec<Vec<i64>>, p: &Vec<Vec<Vec<u64>>>) -> i64{
    //     let mut eval = 0;
    //     for n in 0..Evaluator::EVAL_CHANGE_INTERVAL{
    //         for rotation in 0..4 {
    //             for p_num  in 0..Evaluator::PATTERN_NUM {
    //                 eval = w[p_num][p[n][rotation][p_num] as usize];
    //             }
    //         }
    //     }
    //     eval
    // }

    
    // pub fn learn_eval(&mut self) {
    //     //vec![vec![vec![0;59049]; Evaluator::PATTERN_NUM]; 60/Evaluator::EVAL_CHANGE_INTERVAL + 1],
    //     let learn_data_num = 1000usize;
    //     // let learn_data = vec![vec![vec![vec![0;59049]; Evaluator::PATTERN_NUM]; 60/Evaluator::EVAL_CHANGE_INTERVAL + 1]; learn_data_num];
        
    //     let learn_data = vec![vec![vec![vec![0;Evaluator::PATTERN_NUM]; 4]; 60]; learn_data_num];
    //     let learn_data_teacher_data = vec![0; learn_data_num];

    //     let eval_w = vec![vec![vec![0;59049]; Evaluator::PATTERN_NUM]; 60/Evaluator::EVAL_CHANGE_INTERVAL + 1];
    //     // [step][pattern_No][pattern]

    //     let mut ej = 0;
    //     for i in 0..learn_data_num {
    //         for step in  0..(60/Evaluator::EVAL_CHANGE_INTERVAL + 1){
    //             for j in 0..Evaluator::PATTERN_NUM {
    //                 let e = eb(&eval_w[step], &learn_data[i]);
    //             }
    //         }
    //     }
    // }
}

