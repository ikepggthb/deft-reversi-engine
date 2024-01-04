
pub mod board;
pub mod ai;
pub mod perfect_search;
pub mod perfect_solver;
mod bit;
mod eval;
mod learn;
mod search;

mod t_table;
// ---

pub use board::*;
pub use ai::*;
pub use perfect_solver::*;
// use eval::*;
// use learn::*;


pub struct BoardManager {
    board_record: Vec<Board>
}

impl BoardManager {
    pub fn new() -> Self {
        let mut bm = Self { board_record: Vec::new() };
        bm.board_record.push(Board::new());
        bm
    }
    
    pub fn current_board(&self) -> Board {
        self.board_record.last().unwrap().clone()
    }

    pub fn undo(&mut self) -> Board {
        self.board_record.pop().unwrap()
    }

    pub fn add(&mut self, board: Board){
        self.board_record.push(board);
    }

    pub fn clean(&mut self) {
        self.board_record.clear();
        self.board_record.push(Board::new());
    }
}