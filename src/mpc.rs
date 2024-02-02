use crate::{board_manager::*, learn};
use crate::{board::Board, perfect_search::solve_score};
use serde::{Deserialize, Serialize};

use crate::eval::evaluator_const::*;
use crate::eval_for_learn::*;

use std::{env, clone};
use std::fs::File;
use std::fs;
use std::io::prelude::*;


struct mpc {
    e: Vec<i32>
}