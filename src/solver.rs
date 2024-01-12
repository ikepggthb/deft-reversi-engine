use crate::board::*;
use crate::perfect_search::*;
use crate::eval_search::*;
use crate::search::*;
use crate::t_table::*;

// use std::time;

pub struct SolverResult {
    pub best_move: u64,
    pub eval: i32,
    pub node_count: u64,
    pub leaf_node_count: u64
}

pub enum SolverErr {
    NoMove,
}

const SCORE_INF: i32 = i32::MAX;
const MOVE_ORDERING_EVAL_LEVEL: i32 = 6;

pub fn perfect_solver(board: &Board, print_log: bool) -> Result<SolverResult, SolverErr>
{
    // let now = time::Instant::now();

    let legal_moves = board.put_able();
    if legal_moves == 0 {
        return Err(SolverErr::NoMove)
    }

    let mut search = Search::new(board, Some(TranspositionTable::new()));
    
    if print_log {
        println!("my_turn: {}", if board.next_turn == Board::BLACK {"Black"} else {"White"});
        println!("depth: {}", num_of_empties(board));
        board.print_board();
    };

    if print_log {print!("move_ordering....");};
    let mut put_boards = 
        if num_of_empties(board) < MOVE_ORDERING_EVAL_LEVEL {
            get_put_boards(board, legal_moves)
        } else {
            move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL)
        };
    if print_log {println!("OK");};

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut put_place_best_score ;
    
    let mut put_boards_iter = put_boards.iter_mut();
    let first_child_board = put_boards_iter.next().unwrap();
    alpha = -pvs_perfect(&mut first_child_board.board, -beta, -alpha, &mut search);
    put_place_best_score = first_child_board.put_place;
    if print_log { 
        println!("put: {}, nega scout score: {}",Board::move_bit_to_str(put_place_best_score).unwrap(), alpha);
    };

    for put_board in put_boards_iter {
        let current_put_board = &put_board.board;
        let put_place = put_board.put_place;
        let mut score = -nws_perfect(current_put_board, -alpha - 1, &mut search);
        if score > alpha {
            alpha = score;
            if print_log { 
                println!(" put: {}, null window score: {} => reserch [{},{}]",Board::move_bit_to_str(put_place).unwrap(), score, alpha, beta);
            }
            score = -pvs_perfect(current_put_board, -beta, -alpha, &mut search);
            alpha = score;
            put_place_best_score = put_place;
        }
        if print_log { 
            println!("put: {}, nega scout score: {}",Board::move_bit_to_str(put_place).unwrap(), score);
        }
    }

    // let end = now.elapsed();
    if print_log { 
        println!("best move: {}, score: {}{}",Board::move_bit_to_str(put_place_best_score).unwrap(), if alpha > 0 {"+"} else {""},alpha);
        println!("searched nodes: {}\nsearched leaf nodes: {}", search.node_count, search.leaf_node_count);
        // println!("time: {:?}, nps: {}", end, search.node_count as f64 / end.as_secs_f64());
    }
    

    Ok(SolverResult{
        best_move: put_place_best_score,
        eval: alpha,
        node_count: search.node_count,
        leaf_node_count: search.leaf_node_count
    })
}

pub fn winning_solver(board: &Board, print_log: bool) -> Result<SolverResult, SolverErr>
{
    // let now = time::Instant::now();

    let legal_moves = board.put_able();
    if legal_moves == 0 {
        return Err(SolverErr::NoMove)
    }

    let mut search = Search::new(board, Some(TranspositionTable::new()));
    
    if print_log {
        println!("my_turn: {}", if board.next_turn == Board::BLACK {"Black"} else {"White"});
        println!("depth: {}", num_of_empties(board));
        board.print_board();
    };

    if print_log {print!("move_ordering....");};
    let mut put_boards = 
        if num_of_empties(board) < MOVE_ORDERING_EVAL_LEVEL {
            get_put_boards(board, legal_moves)
        } else {
            move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL)
        };
    if print_log {println!("OK");};

    // [alpha, beta] = [0, 1]
    let mut put_place_best_score = 0;
    let mut eval = -1;
    let beta = 1;
    let mut draw_or_lose_board_index: Vec<usize> = Vec::new();

    for (i, put_board) in  put_boards.iter_mut().enumerate() {
        let current_put_board = &mut put_board.board;
        let put_place = put_board.put_place;
        let score = -nws_perfect(current_put_board, -beta,&mut search);
        if score > 0 {
            if print_log { 
                println!(" put: {}, Win",Board::move_bit_to_str(put_place).unwrap());
            }
            if eval <= 0 {
                put_place_best_score = put_place;
                eval = 1
            };
            break;
        } else if score < 0 {
            if print_log { 
                println!(" put: {}, Lose",Board::move_bit_to_str(put_place).unwrap());
            }
        } else {
            draw_or_lose_board_index.push(i);
            if eval < 0 {
                put_place_best_score = put_place;
                eval = 0
            };
            if print_log { 
                println!(" put: {}, Draw or Lose", Board::move_bit_to_str(put_place).unwrap());
            }
        }
    }

    if eval == 0 {
        // [alpha, beta] = [-1, 0]
        let beta = 0;

        for &i in draw_or_lose_board_index.iter(){
            let put_board = &mut put_boards[i];
            let current_put_board = &mut put_board.board;
            let put_place = put_board.put_place;
            let score = -nws_perfect(current_put_board, -beta,&mut search);
            if score == 0 {
                if print_log { 
                    println!(" put: {}, Draw", Board::move_bit_to_str(put_place).unwrap());
                }
                if eval < 0 {
                    put_place_best_score = put_place;
                    eval = 0
                };
                break;

            } else if score < 0 {
                if print_log { 
                    println!(" put: {}, Lose",Board::move_bit_to_str(put_place).unwrap());
                }
                eval = -1;
            } else {
                eprintln!("Error ocurred in winning_solver");
                panic!()
            }
        }   
    }
    if eval == -1 {
        put_place_best_score = put_boards[0].put_place;
    }
    

    // let end = now.elapsed();
    if print_log { 
        println!("best move: {}, score: {}",Board::move_bit_to_str(put_place_best_score).unwrap(), if eval > 0 {"Win"} else if eval < 0 {"Lose"} else {"Draw"});
        println!("searched nodes: {}\nsearched leaf nodes: {}", search.node_count, search.leaf_node_count);
        // println!("time: {:?}, nps: {}", end, search.node_count as f64 / end.as_secs_f64());
    }

    Ok(SolverResult{
        best_move: put_place_best_score,
        eval: eval,
        node_count: search.node_count,
        leaf_node_count: search.leaf_node_count
    })
}


pub fn eval_solver(board: &Board, lv: i32, print_log: bool) -> Result<SolverResult, SolverErr>
{
    // let now = time::Instant::now();

    let legal_moves = board.put_able();
    if legal_moves == 0 {
        return Err(SolverErr::NoMove)
    }

    let mut search = Search::new(board, Some(TranspositionTable::new()));
    
    if print_log {
        println!("my_turn: {}", if board.next_turn == Board::BLACK {"Black"} else {"White"});
        println!("depth: {}", num_of_empties(board));
        board.print_board();
    };

    if print_log {print!("move_ordering....");};
    let put_boards = 
        if lv - 4 <= 0 {
            get_put_boards(board, legal_moves)
        } else {
            
            move_ordering_eval(board, legal_moves, 6)
        };
    if print_log {println!("OK");};

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut put_place_best_score ;
    
    let mut put_boards_iter = put_boards.iter();
    let first_child_board = put_boards_iter.next().unwrap();
    alpha = -pvs_eval(&first_child_board.board, -beta, -alpha, lv - 1, &mut search);
    put_place_best_score = first_child_board.put_place;
    if print_log { 
        println!("put: {}, nega scout score: {}",Board::move_bit_to_str(put_place_best_score).unwrap(), alpha);
    };

    for put_board in put_boards_iter {
        let current_put_board = &put_board.board;
        let put_place = put_board.put_place;
        let mut score = -nws_eval(current_put_board, -alpha - 1, lv - 1, &mut search);
        if score > alpha {
            alpha = score;
            if print_log { 
                println!(" put: {}, null window score: {} => reserch [{},{}]",Board::move_bit_to_str(put_place).unwrap(), score, alpha, beta);
            }
            score = -pvs_eval(current_put_board, -beta, -alpha, lv - 1, &mut search);
            alpha = score;
            put_place_best_score = put_place;
        }
        if print_log { 
            println!("put: {}, nega scout score: {}",Board::move_bit_to_str(put_place).unwrap(), score);
        }
    }

    // let end = now.elapsed();
    if print_log { 
        println!("best move: {}, score: {}{}",Board::move_bit_to_str(put_place_best_score).unwrap(), if alpha > 0 {"+"} else {""},alpha);
        println!("searched nodes: {}\nsearched leaf nodes: {}", search.node_count, search.leaf_node_count);
        // println!("time: {:?}, nps: {}", end, search.node_count as f64 / end.as_secs_f64());
    }
    

    Ok(SolverResult{
        best_move: put_place_best_score,
        eval: alpha,
        node_count: search.node_count,
        leaf_node_count: search.leaf_node_count
    })
}