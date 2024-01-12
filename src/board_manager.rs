use crate::Board;


#[derive(Clone)]
pub struct BoardManager {
    pub board_record: Vec<Board>
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