use crate::board::*;
use rand::Rng;

#[derive(Clone)]
pub struct TableData {
    exists: bool,
    pub board: Board,
    pub max: i32,
    pub min: i32,
}

impl TableData {
    fn make_blank() -> Self{
        Self {
            exists: false,
            board: Board {bit_board: [0, 0],next_turn: 0},
            max: 0,
            min: 0
        }
    }
}

const TABLE_SIZE: usize = 1 << 18;
pub struct TranspositionTable {
    table: Vec::<TableData>,
    rand_table: Vec<Vec<u32>>
}

impl TranspositionTable {
    pub fn new() -> Self{
        let rand_table = Self::gen_rand_table();
        Self {
            table: vec![TableData::make_blank(); TABLE_SIZE],
            rand_table: rand_table
        }

    }

    fn gen_rand_table() -> Vec<Vec<u32>> {
        let mut rng = rand::thread_rng();
        let mut table = vec![vec![0u32; TABLE_SIZE]; 8];
    
        for i in 0..8 {
            for j in 0..TABLE_SIZE {
                table[i][j] = rng.gen_range(0..(TABLE_SIZE- 1) as u32);
            }
        }
    
        table
    }

    #[inline(always)]
    pub fn hash_board(&self, board: &Board) -> usize{
        let player_board_bit = board.bit_board[Board::BLACK];
        let opponent_board_bit = board.bit_board[Board::WHITE];

        return (self.rand_table[0][(player_board_bit & 0xFFFF) as usize] ^
            self.rand_table[1][((player_board_bit >> 16) & 0xFFFF) as usize] ^
            self.rand_table[2][((player_board_bit >> 32) & 0xFFFF) as usize] ^
            self.rand_table[3][((player_board_bit >> 48) & 0xFFFF) as usize] ^
            self.rand_table[4][((opponent_board_bit >> 48) & 0xFFFF) as usize] ^
            self.rand_table[5][((opponent_board_bit >> 32) & 0xFFFF) as usize] ^
            self.rand_table[6][((opponent_board_bit >> 16) & 0xFFFF) as usize] ^
            self.rand_table[7][(opponent_board_bit & 0xFFFF) as usize]
            ^ 0b1000100010001 )as usize
    }

    #[inline(always)]
    pub fn add(&mut self, board: &Board, min: i32, max: i32) {
        let index = self.hash_board(board);
        self.table[index] = TableData {
            exists: true,
            board: board.clone(),
            max: max,
            min: min
        }
    }

    #[inline(always)]
    pub fn get(&self, board: &Board) -> Option<&TableData>{
        let index = self.hash_board(board) as usize;
        let x = &self.table[index];
        if !x.exists {return None;}
        if x.board.bit_board[Board::BLACK] == board.bit_board[Board::BLACK] &&
           x.board.bit_board[Board::WHITE] == board.bit_board[Board::WHITE] &&
           x.board.next_turn == board.next_turn {
            return Some(x);
        } else {
            return None;
        }
    }
}