mod ai;
mod board;
mod bit;
mod eval;
mod learn;
mod t_table;

mod search;
mod perfect_search;
mod perfect_solver;
// ---

use eval::*;
use ai::*;
use board::*;
use learn::*;

use perfect_solver::*;


use std::time;

use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::sync::{Arc, Mutex};




// fn start_eval_clac ( ) -> Evaluator{
//     let mut e = Evaluator::new();
//     for i in 0..10000 {
//         eprintln!("count: {}", i);
//         e.learn_eval_from_board_pattern();
//     }
//     // for _ in 0..10000 {
//     //     e.learn_eval_from_board_pattern2();
//     // }
//     // for i in 0..100000 {
//     //     eprintln!("count: {}", i);
//     //     e.learn_eval_from_board_pattern3();
//     // }    
//     match e.save_to_file("output.txt") {
//         Ok(_) => println!("ファイルの保存に成功しました！"),
//         Err(e) => eprintln!("ファイルへの書き込みエラー: {}", e),
//     }

//     e
// }


// fn start_eval_clac_thread() -> Evaluator {
//     let num_threads = 4;
//     let iterations_per_thread = 10000 / num_threads;

//     // 各スレッドの結果を格納するベクター
//     let mut handles = vec![];

//     for _ in 0..num_threads {
//         let handle = thread::spawn(move || {
//             let mut e = Evaluator::new();
//             for i in 0..iterations_per_thread {
//                 eprintln!("count: {}", i);
//                 e.learn_eval_from_board_pattern();
//             }
//             e
//         });
//         handles.push(handle);
//     }

//     // 各スレッドの結果を取得し、eval_from_board_patternsを合計する
//     let mut result_evaluator = Evaluator::new();

//     for handle in handles {
//         let e = handle.join().unwrap();
//         for i in 0..(60/Evaluator::EVAL_CHANGE_INTERVAL + 1) {
//             for j in 0..Evaluator::PATTERN_NUM {
//                 for k in 0..59049 {
//                     result_evaluator.eval_from_board_patterns[i][j][k] += e.eval_from_board_patterns[i][j][k];
//                 }
//             }
//         }
//     }


//     match result_evaluator.save_to_file("output.txt") {
//         Ok(_) => println!("ファイルの保存に成功しました！"),
//         Err(e) => eprintln!("ファイルへの書き込みエラー: {}", e),
//     }

//     result_evaluator
// }

// fn main() -> std::io::Result<()> {
//     //start()?;
//     let mut le = LearnEvaluation::new();
//     le.input_record("./0000_egaroucid_6_3_0_lv11/0000000.txt")?;
//     Ok(())
// }



use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn ffo_test() -> Result<(),  std::io::Error> {
    for i in 40..=50 {
        let filename = format!("ffotest/end{}.pos", i);
        let board = match read_ffo_test_files(&filename) {
            Ok(it) => it,
            Err(err) => {
                eprintln!("Error reading the file {}: {}", filename, err);
                continue;
            },
        };
    
        println!("#{} ", i);

        let now = time::Instant::now();
        let solver_result = 
            match winning_solver(&board, true) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Error occurred in perfect solver.");
                    panic!();
                }
            };
        
        let end = now.elapsed();
        println!("time: {:?}, nps: {}", end, solver_result.node_count as f64 / end.as_secs_f64());

        
        println!();

    }

    Ok(())
}

fn read_ffo_test_files<P: AsRef<Path>>(filename: P) -> io::Result<Board> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut board = Board { bit_board: [0;2], next_turn: Board::BLACK };

    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    for (i,c) in first_line.chars().enumerate() {
        match c {
            'O' => {
                board.bit_board[Board::WHITE] |= 1 << i;
            },
            'X' => {
                board.bit_board[Board::BLACK] |= 1 << i;
            }
            _ => ()
        }
    }
    
    let second_line = lines.next().unwrap().unwrap();
    println!("{}",first_line);
    println!("{}",second_line);
    if second_line.contains("Black") {
        board.next_turn = Board::BLACK;
    }else {
        board.next_turn = Board::WHITE;
    }

    Ok(board)
}

fn main () {
    ffo_test();
    //let mut eval = Evaluator::new();

    // let learn_move_count = 56;
    //  eval.learn_debug(learn_move_count);

    // let learn_count = 5000000;
    // let p = learn_count / 100;
     
    // for j in 0..1 {
       
    //     for i in 0..learn_count {
    //         eval.learn(learn_move_count);

    //         if i % p == 0 {
    //             println!("{}%", i / p);
    //         }
    //     }

    //     let mut error_sum = 0f64;
    //     for k in 0..100 {
    //         error_sum += eval.learn_debug(learn_move_count);
    //     }
    //     println!("誤差平均: {}", error_sum / 100f64);
    // }

}