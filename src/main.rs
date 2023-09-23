mod ai;
mod board;
mod bit;
mod eval;
mod learn;
// ---

use eval::*;
use ai::*;
use board::*;
use learn::*;

use std::io::{stdin, stdout, Write};
use std::thread;
use std::sync::{Arc, Mutex};



fn start_eval_clac ( ) -> Evaluator{
    let mut e = Evaluator::new();
    for i in 0..10000 {
        eprintln!("count: {}", i);
        e.learn_eval_from_board_pattern();
    }
    // for _ in 0..10000 {
    //     e.learn_eval_from_board_pattern2();
    // }
    // for i in 0..100000 {
    //     eprintln!("count: {}", i);
    //     e.learn_eval_from_board_pattern3();
    // }    
    match e.save_to_file("output.txt") {
        Ok(_) => println!("ファイルの保存に成功しました！"),
        Err(e) => eprintln!("ファイルへの書き込みエラー: {}", e),
    }

    e
}


fn start_eval_clac_thread() -> Evaluator {
    let num_threads = 4;
    let iterations_per_thread = 10000 / num_threads;

    // 各スレッドの結果を格納するベクター
    let mut handles = vec![];

    for _ in 0..num_threads {
        let handle = thread::spawn(move || {
            let mut e = Evaluator::new();
            for i in 0..iterations_per_thread {
                eprintln!("count: {}", i);
                e.learn_eval_from_board_pattern();
            }
            e
        });
        handles.push(handle);
    }

    // 各スレッドの結果を取得し、eval_from_board_patternsを合計する
    let mut result_evaluator = Evaluator::new();

    for handle in handles {
        let e = handle.join().unwrap();
        for i in 0..(60/Evaluator::EVAL_CHANGE_INTERVAL + 1) {
            for j in 0..Evaluator::PATTERN_NUM {
                for k in 0..59049 {
                    result_evaluator.eval_from_board_patterns[i][j][k] += e.eval_from_board_patterns[i][j][k];
                }
            }
        }
    }


    match result_evaluator.save_to_file("output.txt") {
        Ok(_) => println!("ファイルの保存に成功しました！"),
        Err(e) => eprintln!("ファイルへの書き込みエラー: {}", e),
    }

    result_evaluator
}

// fn main() -> std::io::Result<()> {
//     //start()?;
//     let mut le = LearnEvaluation::new();
//     le.input_record("./0000_egaroucid_6_3_0_lv11/0000000.txt")?;
//     Ok(())
// }

fn print_board(board: &Board) {
    for y in 0..8 {
        for x in 0..8 {
            let mask = 1u64 << y * 8 + x;
            if board.bit_board[Board::BLACK] & mask != 0 {
                print!("+");
            } else if board.bit_board[Board::WHITE] & mask != 0 {
                print!("-");
            } else {
                print!(".");
            }
        }
        println!();
    }
}



fn main () {
    let board = Board::new();
    print_board(&board);

}