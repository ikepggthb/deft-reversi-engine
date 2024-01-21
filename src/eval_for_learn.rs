use std::fs::File;
use std::fs;
use serde::{Deserialize, Serialize};
use std::io::*;

use crate::eval::evaluator_const::*;
use crate::board::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct EvaluationScoresForLearn {
    pub pattern_eval: Vec<Vec<f64>>,
    pub player_mobility_eval: Vec<f64>,
    pub opponent_mobility_eval: Vec<f64>,
    pub const_eval: f64
}

#[derive(Serialize, Deserialize)]
pub struct EvaluatorForLearn {
    pub version: String,
    pub n_deta_set: i32,
    pub n_iteration: i32,
    pub eval: Vec<Vec<EvaluationScoresForLearn>>,
    #[serde(skip)]
    pub feature_bit: [[u16; N_ROTATION]; N_PATTERN],
}


impl Default for EvaluationScoresForLearn {
    fn default() -> Self {        
        Self{
            pattern_eval: vec![vec![0.0 ;N_FEATURE_MAX];N_PATTERN],
            player_mobility_eval: vec![0.0; N_MOBILITY_MAX],
            opponent_mobility_eval: vec![0.0; N_MOBILITY_MAX],
            const_eval: 0.0,
        } 
    }
}

impl Default for EvaluatorForLearn {
    fn default() -> Self {        
        Self{
            version: "0".to_string(),
            n_deta_set: 0,
            n_iteration: 0,
            eval: vec![vec![EvaluationScoresForLearn::default();N_PHASE]; 2],
            feature_bit: [[0; N_ROTATION]; N_PATTERN],
        } 
    }
}



impl EvaluatorForLearn {
    pub fn new() -> Self
    {
        Self::default()
    }

    pub fn clac_features(&mut self, board: &Board)
    {
        self.feature_bit = [[0; N_ROTATION]; N_PATTERN];
        
        let p = board.bit_board[board.next_turn];
        let o = board.bit_board[board.next_turn^1];
        
        for pattern in 0..N_PATTERN {
            for rotation in 0..N_ROTATION {
                let fbit = &mut self.feature_bit[pattern][rotation];
                for coord_i in 0..FEATURE_COORD[pattern].n_pattern_square {
                    let coord = FEATURE_COORD[pattern].feature_coord[rotation][coord_i as usize];
                    
                    #[cfg(debug_assertions)]
                    if coord == TERMINATED {panic!()}

                    let color = 2 * (1 & p >> coord) + (1 & o >> coord);
                    *fbit = *fbit * 3u16 + color as u16;
                }
            }
        }
    }

    pub fn clac_eval(&self, board: &Board) -> f64
    {
        let move_count = board.move_count();
        let phase = move_count as usize / 2;

        let mut evaluation  = 0.0;
        
        let eval_scores = &self.eval[board.next_turn][phase];
        for pattern in 0..N_PATTERN {
            let e = &eval_scores.pattern_eval[pattern];
            let f = &self.feature_bit[pattern];

            // for each rotaion
            evaluation += e[f[0] as usize];
            evaluation += e[f[1] as usize];
            evaluation += e[f[2] as usize];
            evaluation += e[f[3] as usize];
        }

        let player_mobility = board.put_able().count_ones();
        let opponent_mobility = {
            let mut b = board.clone();
            b.next_turn ^= 1;
            b.put_able().count_ones()
        };

        evaluation += eval_scores.player_mobility_eval[player_mobility as usize];
        evaluation += eval_scores.opponent_mobility_eval[opponent_mobility as usize];
        evaluation += eval_scores.const_eval;

        evaluation 
    }

    const EVAL_FILE_PATH: &str = "output_eval.json";
    pub fn write_file(&self) -> std::io::Result<()>
    {
        // serialized
        let serialized: String = serde_json::to_string(self).unwrap();

        // write
        let mut file = File::create(Self::EVAL_FILE_PATH)?;
        file.write_all(serialized.as_bytes())?;

        use crate::eval;

        let mut e = eval::Evaluator::new();
        for i in 0..2 {
            for j in 0..N_PHASE {
                let ei16 = &mut e.eval[i][j];
                let ef64 = &self.eval[i][j];

                for pi in 0..N_PATTERN {
                    for fi in 0..N_FEATURE_MAX {
                        ei16.pattern_eval[pi][fi] = (ef64.pattern_eval[pi][fi] * SCORE_RATE as f64) as i16;
                    }
                }

                for mi in 0..N_MOBILITY_MAX {
                     ei16.player_mobility_eval[mi] = (ef64.player_mobility_eval[mi] * (SCORE_RATE as f64)) as i16;
                     ei16.opponent_mobility_eval[mi] = (ef64.opponent_mobility_eval[mi] * SCORE_RATE as f64) as i16;
                }
                ei16.const_eval = (ef64.const_eval * (SCORE_RATE as f64)) as i16;
            }
        }
        e.write_file();
        

        Ok(())
    }

    pub fn read_file() -> std::io::Result<EvaluatorForLearn>
    {
        let input = fs::read_to_string(Self::EVAL_FILE_PATH)?;
        let mut deserialized: EvaluatorForLearn = serde_json::from_str(&input).unwrap();
        Ok(deserialized)
    }

    pub fn read_string(input: String) -> std::io::Result<EvaluatorForLearn>
    {
        let mut deserialized: EvaluatorForLearn = serde_json::from_str(&input).unwrap();
        Ok(deserialized)
    }
}
