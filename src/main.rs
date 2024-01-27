mod eval_simple;
mod board;
mod bit;
mod t_table;
mod search;
mod perfect_search;
mod eval_search;
mod solver;
mod board_manager;
mod eval;
mod learn;
mod ffo_test;
mod eval_for_learn;

// mod game;
// ---


use ffo_test::*;
use learn::*;
use eval::*;


fn main () {

    // learning();

    let mut eval_func = Evaluator::read_file().unwrap();
    ffo_test(&mut eval_func);

}