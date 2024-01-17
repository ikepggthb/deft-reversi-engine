/*

-----------------------
Pattern 1:
 
X.XXXX.X
..XXXX..
........
........
........
........
........
........

------------------
Pattern 2:
 
XXXXXXXX
.X....X.
........
........
........
........
........
........

-----------------------
Pattern 3:
 
X......X
XXXXXXXX
........
........
........
........
........
........

-----------------------
Pattern 4:
 
........
........
XXXXXXXX
........
........
........
........
........


-----------------------
Pattern 5:
 
........
........
........
XXXXXXXX
........
........
........
........

-----------------------
Pattern 6:
 
XXX.....
XXX.....
XXX.....
........
........
........
........
........

-------------------------
Pattern 7:
 
XXXX....
XXX.....
XX......
X.......
........
........
........
........

-------------------------
Pattern 8:
 
XX..X...
XX.X....
..X.....
.X......
X.......
........
........
........

-------------------------
Pattern 9:
 
.....X..
....X...
...X....
..X.....
.X......
X.......
........
........

-------------------------
Pattern 10:
 
......X.
.....X..
....X...
...X....
..X.....
.X......
X.......
........

-------------------------
Pattern 11:
 
.......X
......X.
.....X..
....X...
...X....
..X.....
.X......
X.......


2

3*2 + 1

3^2*2 + 3 * 1

...


-------------------------
*/
const A1: u8 = 0;
const B1: u8 = 1;
const C1: u8 = 2;
const D1: u8 = 3;
const E1: u8 = 4;
const F1: u8 = 5;
const G1: u8 = 6;
const H1: u8 = 7;
const A2: u8 = 8;
const B2: u8 = 9;
const C2: u8 = 10;
const D2: u8 = 11;
const E2: u8 = 12;
const F2: u8 = 13;
const G2: u8 = 14;
const H2: u8 = 15;
const A3: u8 = 16;
const B3: u8 = 17;
const C3: u8 = 18;
const D3: u8 = 19;
const E3: u8 = 20;
const F3: u8 = 21;
const G3: u8 = 22;
const H3: u8 = 23;
const A4: u8 = 24;
const B4: u8 = 25;
const C4: u8 = 26;
const D4: u8 = 27;
const E4: u8 = 28;
const F4: u8 = 29;
const G4: u8 = 30;
const H4: u8 = 31;
const A5: u8 = 32;
const B5: u8 = 33;
const C5: u8 = 34;
const D5: u8 = 35;
const E5: u8 = 36;
const F5: u8 = 37;
const G5: u8 = 38;
const H5: u8 = 39;
const A6: u8 = 40;
const B6: u8 = 41;
const C6: u8 = 42;
const D6: u8 = 43;
const E6: u8 = 44;
const F6: u8 = 45;
const G6: u8 = 46;
const H6: u8 = 47;
const A7: u8 = 48;
const B7: u8 = 49;
const C7: u8 = 50;
const D7: u8 = 51;
const E7: u8 = 52;
const F7: u8 = 53;
const G7: u8 = 54;
const H7: u8 = 55;
const A8: u8 = 56;
const B8: u8 = 57;
const C8: u8 = 58;
const D8: u8 = 59;
const E8: u8 = 60;
const F8: u8 = 61;
const G8: u8 = 62;
const H8: u8 = 63;

const P3_0: i32 = 1;
const P3_1: i32 = 3;
const P3_2: i32 = 9;
const P3_3: i32 = 27;
const P3_4: i32 = 81;
const P3_5: i32 = 243;
const P3_6: i32 = 729;
const P3_7: i32 = 2187;
const P3_8: i32 = 6561;
const P3_9: i32 = 19683;
const P3_10: i32 = 59049;
const P3: [i32; 11] = [1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049];

const MAX_PATTERN_SQUARE: usize = 10;
const N_ROTATION: usize = 4;
const N_PATTERN: usize = 11;
const N_FEAUTURE: usize = N_PATTERN * 4;

const TERMINATED: u8 = u8::MAX;

struct CoordToFeature {
    n_pattern_square: u8,
    feature_coord: [[u8;MAX_PATTERN_SQUARE];N_ROTATION]
}

const FEATURE_COORD:  [CoordToFeature;N_PATTERN] = 
    [
        // 1
        CoordToFeature {
            n_pattern_square: 10,
            feature_coord:[ 
                [A1, C1, D1, E1, F1, H1, C2, D2, E2, F2],
                [A8, A6, A5, A4, A3, A1, B6, B5, B4, B3],
                [H8, F8, E8, D8, C8, A8, F7, E7, D7, C7],
                [H1, H3, H4, H5, H6, H8, G3, G4, G5, G6]
            ],
        },
        // 2
        CoordToFeature {
            n_pattern_square: 10,
            feature_coord:[
                [A1, B1, C1, D1, E1, F1, G1, H1, B2, G2],
                [A8 ,A7, A6, A5, A4, A3, A2, A1, B7, B2], 
                [H8, G8, F8, E8, D8, C8, B8, A8, G7, B7],
                [H1, H2, H3, H4, H5, H6, H7, H8, G2, G7]
            ],
        },
        // 3
        CoordToFeature {
            n_pattern_square: 10,
            feature_coord:[
                [A1, H1, A2, B2, C2, D2, E2, F2, G2, H2],
                [A8, A1, B8, B7, B6, B5, B4, B3, B2, B1],
                [H8, A8, H7, G7, F7, E7, D7, C7, B7, A7],
                [H1, H8, G1, G2, G3, G4, G5, G6, G7, G8]
            ]
        },
        // 4
        CoordToFeature {
            n_pattern_square: 8,
            feature_coord:[
                [A3, B3, C3, D3, E3, F3, G3, H3, TERMINATED, TERMINATED],
                [C8, C7, C6, C5, C4, C3, C2, C1, TERMINATED, TERMINATED],
                [H6, G6, F6, E6, D6, C6, B6, A6, TERMINATED, TERMINATED],
                [F1, F2, F3, F4, F5, F6, F7, F8, TERMINATED, TERMINATED]
            ]
        },
        // 5
        CoordToFeature {
            n_pattern_square: 8,
            feature_coord:[ 
                [A4, B4, C4, D4, E4, F4, G4, H4, TERMINATED, TERMINATED],
                [D8, D7, D6, D5, D4, D3, D2, D1, TERMINATED, TERMINATED],
                [H5, G5, F5, E5, D5, C5, B5, A5, TERMINATED, TERMINATED],
                [E1, E2, E3, E4, E5, E6, E7, E8, TERMINATED, TERMINATED]
            ]
        },
        // 6
        CoordToFeature {
            n_pattern_square: 9,
            feature_coord:[
                [A1, B1, C1, A2, B2, C2, A3, B3, C3, TERMINATED],
                [A8, A7, A6, B8, B7, B6, C8, C7, C6, TERMINATED],
                [H8, G8, F8, H7, G7, F7, H6, G6, F6, TERMINATED],
                [H1, H2, H3, G1, G2, G3, F1, F2, F3, TERMINATED]
            ]
        },
        // 7
        CoordToFeature {
            n_pattern_square: 10,
            feature_coord:[
                [A1, B1, C1, D1, A2, B2, C2, A3, B3, A4],
                [A8, A7, A6, A5, B8, B7, B6, C8, C7, D8],
                [H8, G8, F8, E8, H7, G7, F7, H6, G6, H5],
                [H1, H2, H3, H4, G1, G2, G3, F1, F2, E1]
            ]
        },
        // 8
        CoordToFeature {
            n_pattern_square: 9,
            feature_coord:[
                [A1, B1, E1, A2, B2, D2, C3, B4, A5, TERMINATED],
                [A8, A7, A4, B8, B7, B5, C6, D7, E8, TERMINATED],
                [H8, G8, D8, H7, G7, E7, F6, G5, H4, TERMINATED],
                [H1, H2, H5, G1, G2, G4, F3, E2, D1, TERMINATED]
            ]
        },
        // 9
        CoordToFeature {
            n_pattern_square: 6,
            feature_coord:[
                [F1, E2, D3, C4, B5, A6, TERMINATED, TERMINATED, TERMINATED, TERMINATED],
                [A3, B4, C5, D6, E7, F8, TERMINATED, TERMINATED, TERMINATED, TERMINATED],
                [C8, D7, E6, F5, G4, H3, TERMINATED, TERMINATED, TERMINATED, TERMINATED],
                [H6, G5, F4, E3, D2, C1, TERMINATED, TERMINATED, TERMINATED, TERMINATED]
            ]
        },
        // 10
        CoordToFeature {
            n_pattern_square: 7,
            feature_coord:[
                [G1, F2, E3, D4, C5, B6, A7, TERMINATED, TERMINATED, TERMINATED],
                [A2, B3, C4, D5, E6, F7, G8, TERMINATED, TERMINATED, TERMINATED],
                [B8, C7, D6, E5, F4, G3, H2, TERMINATED, TERMINATED, TERMINATED],
                [H7, G6, F5, E4, D3, C2, B1, TERMINATED, TERMINATED, TERMINATED]
            ]
        },
        // 11
        CoordToFeature {
            n_pattern_square: 8,
            feature_coord:[
                [H1, G2, F3, E4, D5, C6, B7, A8, TERMINATED, TERMINATED],
                [A1, B2, C3, D4, E5, F6, G7, H8, TERMINATED, TERMINATED],
                [A8, B7, C6, D5, E4, F3, G2, H1, TERMINATED, TERMINATED],
                [H8, G7, F6, E5, D4, C3, B2, A1, TERMINATED, TERMINATED]
            ]
        }
    ];

use deft_reversi_engine::eval_search;

use crate::board::*;

const N_FEATURE_MAX: usize = P3_10 as usize;

pub struct Evaluator {
    feature_bit: [[u16; N_ROTATION]; N_PATTERN],
    pattern_eval: Vec<Vec<Vec<Vec<i32>>>>
}

impl Evaluator {
    pub fn new() -> Self
    {
        Self{
             feature_bit: [[0; N_ROTATION]; N_PATTERN],
             pattern_eval: vec![vec![vec![vec![0;N_FEATURE_MAX];N_PATTERN];30];2]
        } 
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

    pub fn clac_eval(&self, board: &Board) -> i32
    {
        let p = board.bit_board[board.next_turn];
        let o = board.bit_board[board.next_turn^1];

        let move_count = board.move_count();

        let mut eval  = 0;
        
        for pattern in 0..N_PATTERN {
            let e = &self.pattern_eval[board.next_turn][move_count as usize / 2][pattern];
            let f = &self.feature_bit[pattern];

            // for each rotaion
            eval += e[f[0] as usize];
            eval += e[f[1] as usize];
            eval += e[f[2] as usize];
            eval += e[f[3] as usize];
        }

        eval
    }
}
