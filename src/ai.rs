
use std::collections::BTreeMap;

use crate::board::*;
use rand::Rng;
pub static mut TCOUNT: i64 = 0;

use crate::t_table::*;

const SCORE_INF: i32 = 100000i32;

#[allow(dead_code)]
pub fn end_game_full_solver_negamax(board: &Board) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }
    let mut max_score = -64;
    let mut max_score_move = 0u64;
    
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        virt_board.put_piece_fast(put_place);
        let this_score = - negamax(&mut virt_board);
        // eprintln!("this_score: {}",this_score );
        if this_score > max_score {
            max_score = this_score;
            max_score_move = put_place;
        }
    }
    // unsafe {
    //     eprintln!("searched nodes: {}", TCOUNT);
    // }
    // eprintln!("full solver: {}", max_score);
    max_score_move
} 

pub fn negamax(board: &mut Board) -> i32{

    let mut moves = board.put_able();
    let mut best_score = i32::MIN;
    unsafe {TCOUNT += 1;}
    while moves != 0 {
        let mut current_board = board.clone();
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        current_board.put_piece_fast(put_place);
        let score = -negamax(&mut current_board);
        best_score = best_score.max(score);
    }

    if best_score == i32::MIN {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
            
            // ここは、処理を高速化するため、passしたのををもとに戻すを省略していることに注意
            // 本来であれば以下のようになる
            // passをもとに戻す
            //     board.next_turn ^= 1 
            // 「最後に打った次の評価値」すなわち、「boardの自分の手番の評価値」すなわち、「最後に打った手番の負の評価値」を返す
            //     return board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn  ^ 1].count_ones() as i32; 
            // もちろん、終盤ソルバーなので、最後の石数差を評価値としている。
        }
        return -negamax(board);
    }

    best_score
}

#[allow(dead_code)]
pub fn end_game_full_solver_nega_alpha(board: &Board) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }

    let mut alpha = -SCORE_INF;
    let mut max_score_move = 0u64;
    let beta = SCORE_INF;
    
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        virt_board.put_piece_fast(put_place);
        let this_score = -nega_alpha(&mut virt_board, -beta, -alpha);
        // eprintln!("this_score: {}",this_score);
        if this_score > alpha {
            alpha = this_score;
            max_score_move = put_place;
        }
    }
    // unsafe { 
    //     eprintln!("searched nodes: {}", TCOUNT);
    // }
    // eprintln!("full solver: {}", alpha);
    max_score_move
} 


#[allow(dead_code)]
pub fn end_game_full_solver_nega_alpha_return_detail(board: &Board) -> (u64, i32){
    let mut moves = board.put_able();
    if moves == 0 {
        return (0, board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32);
    }

    let mut alpha = -SCORE_INF;
    let mut max_score_move = 0u64;
    let beta = SCORE_INF;
    
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        virt_board.put_piece_fast(put_place);
        let this_score = -nega_alpha(&mut virt_board, -beta, -alpha);
        // eprintln!("this_score: {}",this_score);
        if this_score > alpha {
            alpha = this_score;
            max_score_move = put_place;
        }
    }
    // unsafe { 
    //     eprintln!("searched nodes: {}", TCOUNT);
    // }
    // eprintln!("full solver: {}", alpha);
    (max_score_move, alpha)
} 



pub fn nega_alpha(board: &mut Board, mut alpha: i32,beta: i32) -> i32{
    unsafe {TCOUNT += 1;}
    if (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]) == u64::MAX {
        return  board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32;
    }
    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    if moves == 0 {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            // 終盤ソルバーなので、最後の石数差を評価値としている。
            // ここは、処理を高速化するため、passをもとに戻すを省略していることに注意
            return  -(board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32);
        }
        return -nega_alpha(board, -beta, -alpha);
    }
    let mut best_score = -SCORE_INF;

    while moves != 0 {
        let mut current_board = board.clone();
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        current_board.put_piece_fast(put_place);
        let score = -nega_alpha(&mut current_board, -beta, -alpha);
        if score >= beta {
            return score;
        }
        alpha = alpha.max(score);
        best_score = best_score.max(score);
    }

    best_score
}

#[allow(dead_code)]
pub fn end_game_full_solver_nega_alpha_move_ordering_return_detail(board: &Board) -> (u64, i32){
    let mut moves = board.put_able();
    if moves == 0 {
        return (0, board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32);
    }
    
    unsafe {TCOUNT = 0;}

    // move ordering
    let mut put_board: Vec<(i32, Board, u64)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e  = -nega_alpha_move_ordering_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, 8);
        put_board.push((e, current_put_board, put_place));
    }

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut max_score_move = 0u64;
    
    put_board.sort_unstable_by(|(a,_, _), (b, _, _)| b.partial_cmp(a).unwrap());

    for (_,current_put_board, put_place) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering_from_eval(current_put_board, -beta, -alpha);
        if score > alpha {
            alpha = score;
            max_score_move = *put_place;
        }
    }

    (max_score_move, alpha)
} 
// use std::time::Instant;
pub fn end_game_full_solver_nega_alpha_move_ordering(board: &Board) -> u64{
    // let start = Instant::now();
    
    eprintln!("my_turn: {}", if board.next_turn == Board::BLACK {"Black"} else {"White"});
    let mut moves = board.put_able();
    if moves == 0 {
        eprintln!("put place is none.");
        return 0;
    }
    
    unsafe {TCOUNT = 0;}

    // move ordering
    let mut put_board: Vec<(i32, Board, u64)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e  = -nega_alpha_move_ordering_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, 9);
         eprintln!("move_ordering_score: {}",e);
        put_board.push((e, current_put_board, put_place));
    }
     eprintln!("move_ordering end.");

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut max_score_move = 0u64;
    
    put_board.sort_unstable_by(|(a,_, _), (b, _, _)| b.partial_cmp(a).unwrap());

    
    for (_,current_put_board, put_place) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering_from_eval(current_put_board, -beta, -alpha);
        
        //let score = -nega_alpha_move_ordering(current_put_board, -beta, -alpha);
        
        eprintln!("this_score: {}",score);
        if score > alpha {
            alpha = score;
            max_score_move = *put_place;
        }
    }

    // let end = start.elapsed();
    // eprintln!("{}秒経過しました。", end.as_secs_f64());
    unsafe {
        eprintln!("searched nodes: {}", TCOUNT);
        //  eprintln!("nps: {}", TCOUNT as f64/ end.as_secs_f64());
    }
    // eprintln!("full solver: {}", alpha);


    max_score_move
} 

pub fn nega_alpha_move_ordering_from_eval(board: &mut Board, mut alpha: i32,beta: i32) -> i32{

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {TCOUNT += 1;}
    
    let rest_depth = (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]).count_zeros();

    if rest_depth <= 18 {
        unsafe {TCOUNT -= 1;}
        return nega_alpha_move_ordering(board, alpha, beta);
    }

    // move ordering
    let mut put_board: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e = -nega_alpha_move_ordering_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, 5);
        put_board.push((e, current_put_board));

    }

    put_board.sort_unstable_by(|(a,_), (b, _)| b.partial_cmp(a).unwrap());
    
    let mut best_score = i32::MIN;
    for (_,current_put_board) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering_from_eval(current_put_board, -beta, -alpha);
        if score >= beta {
            return score;
        }
        alpha = alpha.max(score);
        best_score = best_score.max(score);
    }

    if best_score == i32::MIN {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
            
            // ここは、処理を高速化するため、passしたのををもとに戻すを省略していることに注意
            // 本来であれば以下のようになる
            // passをもとに戻す
            //     board.next_turn ^= 1 
            // 「最後に打った次の評価値」すなわち、「boardの自分の手番の評価値」すなわち、「最後に打った手番の負の評価値」を返す
            //     return board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn  ^ 1].count_ones() as i32; 
            // もちろん、終盤ソルバーなので、最後の石数差を評価値としている。
        }
        return -nega_alpha_move_ordering_from_eval(board, -beta, -alpha);
    }

    best_score
}



pub fn nega_alpha_move_ordering(board: &mut Board, mut alpha: i32,beta: i32) -> i32{
     let rest_depth = (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]).count_zeros();
     unsafe {TCOUNT += 1;}
     if rest_depth == 0 {
         return  board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32;
     }
    if rest_depth < 7  {
        unsafe {TCOUNT -= 1;}
        return nega_alpha(board, alpha, beta);
    }

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    if moves == 0 {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            // ここは、処理を高速化するため、passしたのををもとに戻すを省略していることに注意
            // 終盤ソルバーなので、最後の石数差を評価値としている。
            return  -(board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32);
        }
        return -nega_alpha_move_ordering(board, -beta, -alpha);
    }
    
    // move ordering

    let mut put_board: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        put_board.push((current_put_board.put_able().count_ones() as i32, current_put_board));

    }
    put_board.sort_unstable_by(|(a,_), (b, _)| a.partial_cmp(b).unwrap());
    
    
    let mut best_score = i32::MIN;
    for (_,current_put_board) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering(current_put_board, -beta, -alpha);
        if score >= beta {
            return score;
        }
        alpha = alpha.max(score);
        best_score = best_score.max(score);
    }

    best_score
}

use std::time::Instant;
pub fn end_game_full_solver_nega_scout(board: &Board) -> u64{
    let start = Instant::now();

    let mut transposition_table = TranspositionTable::new();
    
    eprintln!("my_turn: {}", if board.next_turn == Board::BLACK {"Black"} else {"White"});
    let mut moves = board.put_able();
    if moves == 0 {
        eprintln!("put place is none.");
        return 0;
    }
    
    unsafe {TCOUNT = 0;}

    // move ordering
    let mut put_boards: Vec<(i32, Board, u64)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e  = -nega_scout_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, 8);
         eprintln!("* {}, move_ordering_score: {}",Board::move_bit_to_str(put_place).unwrap(),e);
        put_boards.push((e, current_put_board, put_place));
    }
     eprintln!("move_ordering end.");

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut max_score_move = 0u64;
    
    put_boards.sort_unstable_by(|(a,_, _), (b, _, _)| b.partial_cmp(a).unwrap());

    let mut put_boards_iter = put_boards.iter_mut();
    let first_child_board = put_boards_iter.next().unwrap();
    alpha = -nega_scout_by_eval(&mut first_child_board.1, -beta, -alpha, &mut transposition_table);
    max_score_move = first_child_board.2;

    eprintln!("put: {}, nega scout score: {}",Board::move_bit_to_str(max_score_move).unwrap(), alpha);
    for (_,current_put_board, put_place) in put_boards_iter {
        let mut score = -nega_alpha_move_ordering_tt(current_put_board, -alpha - 1, -alpha, &mut transposition_table);
        if score > alpha {
            alpha = score;
            score = -nega_scout_by_eval(current_put_board, -beta, -alpha, &mut transposition_table);
            alpha = alpha.max(score);
            max_score_move = *put_place;
        }
        eprintln!("put: {}, nega scout score: {}",Board::move_bit_to_str(*put_place).unwrap(), score);
    }

    let end = start.elapsed();
    eprintln!("{}秒経過しました。", end.as_secs_f64());
    unsafe {
        eprintln!("searched nodes: {}", TCOUNT);
        eprintln!("nps: {}", TCOUNT as f64/ end.as_secs_f64());
    }
    eprintln!("full solver: {}", alpha);

    max_score_move
} 


pub fn nega_scout_by_eval(board: &mut Board, mut alpha: i32,mut beta: i32, transposition_table: &mut TranspositionTable) -> i32{

    
    let rest_depth = (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]).count_zeros();

    if rest_depth < 16 {
        
        // この関数をデバッグするため、以下はすべてコメントアウト
        // return nega_alpha_move_ordering(board, alpha, beta);
        // return nega_scout(board, alpha, beta);
        return nega_scout_tt(board, alpha, beta, transposition_table);
        // return nega_alpha_move_ordering_tt(board, alpha, beta, transposition_table);
    }

    unsafe {TCOUNT += 1;}
    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();

    if moves == 0 {
        board.next_turn ^= 1; //パス
        if board.put_able() == 0 {
            // パスしても置くところがない == ゲーム終了
            // パスしたので、正負を逆にしている
            return  board.bit_board[board.next_turn ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
        }
        return -nega_scout_by_eval(board, -beta, -alpha, transposition_table);
    }

    // 置換表確認
    let mut this_node_alpha = alpha;
    if let Some(t) = transposition_table.get(board) {
        if t.max <= alpha {return t.max}
        else if t.min >= beta {return t.min;}
        else if t.max == t.min {return t.max;}

        this_node_alpha = alpha.max(t.min);
        beta = beta.min(t.max);
    }

    let ordering_depth;
    if rest_depth < 16 {
        ordering_depth = 4;
    } else if rest_depth < 18 {
        ordering_depth = 4;
    } else {
        ordering_depth = 6;
    }

    // move ordering
    let mut child_nodes: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut child_node = board.clone(); child_node.put_piece_fast(put_place);
        let e = -nega_scout_mid_game(&mut child_node, -SCORE_INF, SCORE_INF, ordering_depth);
        child_nodes.push((e, child_node));
    }

    child_nodes.sort_unstable_by(|(a,_), (b, _)| b.partial_cmp(a).unwrap());
    
    let mut put_boards_iter = child_nodes.iter_mut();

    let first_child_node = put_boards_iter.next().unwrap();
    let mut this_node_best_score =  -nega_scout_by_eval(&mut first_child_node.1, -beta, -this_node_alpha, transposition_table);
    if this_node_best_score >= beta { 
        transposition_table.add(board, this_node_best_score, SCORE_INF);
        return this_node_best_score;
    }
    this_node_alpha = this_node_alpha.max(this_node_best_score);

    for (_,child_node) in put_boards_iter {
        let mut score = -nega_alpha_move_ordering_from_eval_tt(child_node, -this_node_alpha - 1, -this_node_alpha, transposition_table);
        if score >= beta {
            transposition_table.add(board, score, SCORE_INF);
            return score;
        }
        if  score > this_node_best_score {
            this_node_alpha = this_node_alpha.max(score);
            this_node_best_score = score;
            score = -nega_scout_by_eval(child_node, -beta, -this_node_alpha, transposition_table);
            if score >= beta { 
                transposition_table.add(board, score, SCORE_INF);
                return score;
            }
        }
        this_node_best_score = this_node_best_score.max(score);
        this_node_alpha = this_node_alpha.max(score);
    }

    if this_node_best_score > alpha {
        transposition_table.add(board, this_node_best_score, this_node_best_score);
    } else {
        transposition_table.add(board, -SCORE_INF, this_node_best_score)
    }

    this_node_alpha
}

pub fn nega_alpha_move_ordering_from_eval_tt(board: &mut Board, mut alpha: i32,mut beta: i32, transposition_table: &mut TranspositionTable) -> i32{

    let rest_depth = (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]).count_zeros();

    if rest_depth < 18 {
        // return nega_alpha_move_ordering(board, alpha, beta);
        return nega_alpha_move_ordering_tt(board, alpha, beta, transposition_table);
    }

    unsafe {TCOUNT += 1;}
    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();

    if moves == 0 {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
        }
        return -nega_alpha_move_ordering_from_eval_tt(board, -beta, -alpha, transposition_table);
    }

    if let Some(t) = transposition_table.get(board) {
        if t.max <= alpha {return t.max}
        else if t.min >= beta {return t.min;}
        else if t.max == t.min {return t.max;}

        alpha = alpha.max(t.min);
        beta = beta.min(t.max);
    }
    let ordering_depth ;
    if rest_depth > 20 {
        ordering_depth = 3;
    } else {
        ordering_depth = 2;
    }


    // move ordering
    let mut put_board: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e =  -nega_alpha_move_ordering_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, ordering_depth);
        put_board.push((e, current_put_board));
    }

    if put_board.len() > 2 {
        put_board.sort_unstable_by(|(a,_), (b, _)| b.partial_cmp(a).unwrap());
    }
    let mut this_node_alpha = alpha;
    let mut best_score = i32::MIN;
    for (_,current_put_board) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering_from_eval_tt(current_put_board, -beta, -this_node_alpha, transposition_table);
        if score >= beta {
            transposition_table.add(board, score, SCORE_INF);
            return score;
        }
        this_node_alpha = this_node_alpha.max(score);
        best_score = best_score.max(score);
    }

    if best_score > alpha {
        transposition_table.add(board, best_score, best_score);
    } else {
        transposition_table.add(board, -SCORE_INF, best_score);
    }

    best_score
}

pub fn nega_alpha_move_ordering_tt(board: &mut Board, mut alpha: i32,mut beta: i32, transposition_table: &mut TranspositionTable) -> i32{

    let rest_depth = (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]).count_zeros();
    // 探索範囲: [alpha, beta]
    if rest_depth < 8  {
        return nega_alpha_move_ordering(board, alpha, beta);
        // return nega_alpha(board, alpha, beta);
    }
    
    unsafe {TCOUNT += 1;}
    if (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]) == u64::MAX {
        return  board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32;
    }

    let mut moves = board.put_able();

    if moves == 0 {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
            
            // ここは、処理を高速化するため、passしたのををもとに戻すを省略していることに注意
            // 本来であれば以下のようになる
            // passをもとに戻す
            //     board.next_turn ^= 1 
            // 「最後に打った次の評価値」すなわち、「boardの自分の手番の評価値」すなわち、「最後に打った手番の負の評価値」を返す
            //     return board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn  ^ 1].count_ones() as i32; 
            // もちろん、終盤ソルバーなので、最後の石数差を評価値としている。
        }
        return -nega_alpha_move_ordering_tt(board, -beta, -alpha, transposition_table);
    }

    if let Some(t) = transposition_table.get(board) {
        if t.max <= alpha {return t.max}
        else if t.min >= beta {return t.min;}
        else if t.max == t.min {return t.max;}

        alpha = alpha.max(t.min);
        beta = beta.min(t.max);
    }

    // move ordering
    let mut put_board: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        put_board.push((current_put_board.put_able().count_ones() as i32, current_put_board));

    }
    if put_board.len() > 3 {
        put_board.sort_unstable_by(|(a,_), (b, _)| a.partial_cmp(b).unwrap());
    }
    let mut this_node_alpha = alpha;

    let mut best_score = i32::MIN;
    for (_,current_put_board) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering_tt(current_put_board, -beta, -this_node_alpha, transposition_table);
        if score >= beta {
            transposition_table.add(board, score, SCORE_INF);
            return score;
        }
        this_node_alpha = this_node_alpha.max(score);
        best_score = best_score.max(score);
    }

    if best_score > alpha {
        transposition_table.add(board, best_score, best_score);
    } else {
        transposition_table.add(board, -SCORE_INF, best_score);
    }

    best_score
}


pub fn nega_scout_tt(board: &mut Board, mut alpha: i32,mut beta: i32, transposition_table: &mut TranspositionTable) -> i32{

    let rest_depth = (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]).count_zeros();
    if rest_depth < 8  {
        return nega_alpha_move_ordering(board, alpha, beta);
    }



    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {TCOUNT += 1;}

    if moves == 0 {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
            
            // ここは、処理を高速化するため、passしたのををもとに戻すを省略していることに注意
            // 本来であれば以下のようになる
            // passをもとに戻す
            //     board.next_turn ^= 1 
            // 「最後に打った次の評価値」すなわち、「boardの自分の手番の評価値」すなわち、「最後に打った手番の負の評価値」を返す
            //     return board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn  ^ 1].count_ones() as i32; 
            // もちろん、終盤ソルバーなので、最後の石数差を評価値としている。
        }
        return -nega_scout_tt(board, -beta, -alpha, transposition_table);
    }

    let mut this_node_alpha = alpha;
    if let Some(t) = transposition_table.get(board) {
        if t.max <= alpha {return t.max}
        else if t.min >= beta {return t.min;}
        else if t.max == t.min {return t.max;}

        this_node_alpha = alpha.max(t.min);
        beta = beta.min(t.max);
    }


    // move ordering
    let mut put_boards: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        put_boards.push((current_put_board.put_able().count_ones() as i32, current_put_board));
    }

    put_boards.sort_unstable_by(|(a,_), (b, _)| a.partial_cmp(b).unwrap());
    let mut put_boards_iter = put_boards.iter_mut();

    
    let first_child_board = put_boards_iter.next().unwrap();
    let mut this_node_best_score =  -nega_scout_tt(&mut first_child_board.1, -beta, -this_node_alpha, transposition_table);
    if this_node_best_score >= beta { 
        transposition_table.add(board, this_node_best_score, SCORE_INF);
        return this_node_best_score;
    }
    this_node_alpha = alpha.max(this_node_best_score);

    for (_,current_put_board) in put_boards_iter {
        let mut score = -nega_alpha_move_ordering_tt(current_put_board, -this_node_alpha - 1, -this_node_alpha, transposition_table);
        if score >= beta {
            transposition_table.add(board, score, SCORE_INF);
            return score;
        }
        if this_node_best_score < score {
            this_node_alpha = this_node_alpha.max(score);
            this_node_best_score = score;
            score = -nega_scout_tt(current_put_board, -beta, -this_node_alpha, transposition_table);
            if beta <= score { 
                transposition_table.add(board, score, SCORE_INF);
                return score;
             }
        }
        this_node_alpha = this_node_alpha.max(score);
        this_node_best_score = this_node_best_score.max(score);
    }

    if this_node_best_score > alpha {
        transposition_table.add(board, this_node_best_score, this_node_best_score);
    } else {
        transposition_table.add(board, -SCORE_INF, this_node_best_score);
    }

    this_node_alpha
}

pub fn nega_scout(board: &mut Board, mut alpha: i32,mut beta: i32) -> i32{

    let rest_depth = 64 - board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones();
    if rest_depth < 10  {
        return nega_alpha_move_ordering(board, alpha, beta);
        // return nega_alpha(board, alpha, beta);
    }

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {TCOUNT += 1;}

    if moves == 0 {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
            
            // ここは、処理を高速化するため、passしたのををもとに戻すを省略していることに注意
            // 本来であれば以下のようになる
            // passをもとに戻す
            //     board.next_turn ^= 1 
            // 「最後に打った次の評価値」すなわち、「boardの自分の手番の評価値」すなわち、「最後に打った手番の負の評価値」を返す
            //     return board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn  ^ 1].count_ones() as i32; 
            // もちろん、終盤ソルバーなので、最後の石数差を評価値としている。
        }
        return -nega_scout(board, -beta, -alpha);
    }

    // move ordering
    let mut put_boards: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        put_boards.push((current_put_board.put_able().count_ones() as i32, current_put_board));
    }

    put_boards.sort_unstable_by(|(a,_), (b, _)| a.partial_cmp(b).unwrap());
    let mut put_boards_iter = put_boards.iter_mut();

    
    let first_child_board = put_boards_iter.next().unwrap();
    let mut this_node_best_score =  -nega_scout(&mut first_child_board.1, -beta, -alpha);
    if this_node_best_score >= beta { return this_node_best_score; }
    let mut this_node_alpha = alpha.max(this_node_best_score);

    for (_,current_put_board) in put_boards_iter {
        let mut score = -nega_scout(current_put_board, -this_node_alpha - 1, -this_node_alpha);
        if score >= beta {
            return score;
        }
        if this_node_alpha < score {
            this_node_alpha = score;
            score = -nega_scout(current_put_board, -beta, -this_node_alpha);
            if beta <= score { 
                return score;
             }
            this_node_alpha = this_node_alpha.max(score);
        }
        this_node_best_score = this_node_best_score.max(score);
    }

    this_node_best_score
}


pub fn end_game_full_solver_mtd_f(board: &Board) -> u64{
    // let start = Instant::now();

    let mut transposition_table = TranspositionTable::new();

    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }
    let mut max_score = -64;
    let mut max_score_move = 0u64;
    let mut f = 0;
    eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        virt_board.put_piece_fast(put_place);
        let this_score = mtd_f(&mut virt_board, f, &mut transposition_table);
        f = this_score;
        eprintln!("this_score: {}",this_score );
        if this_score > max_score {
            max_score = this_score;
            max_score_move = put_place;
            
        }
    }
    unsafe {
        eprintln!("searched nodes: {}", TCOUNT);
    }
    // eprintln!("full solver: {}", max_score);
    max_score_move
} 

pub fn mtd_f(board: &mut Board, mut f: i32, transposition_table: &mut TranspositionTable) -> i32{
    
    let mut upper_bound = SCORE_INF;
    let mut lower_bound = -SCORE_INF;

    while lower_bound < upper_bound {
        let beta;
        if f == lower_bound {
            beta = f + 1;
        } else {
            beta = f;
        }
        f = nega_scout_by_eval(board, beta-1, beta, transposition_table);
        if f < beta {
            upper_bound = f;
        }else {
            lower_bound = f;
        }
    }

    f
}


#[allow(dead_code)]
pub fn put_random_piece(board: &mut Board) -> Result<(), PutPieceErr> {
    let legal_moves = board.put_able();
    if legal_moves == 0 {
        return Err(PutPieceErr::NoValidPlacement);
    }

    let mut bit_indices = [0; 64];
    let mut count = 0;
    let mut moves = legal_moves;
    while moves != 0 {
        let bit_index = moves.trailing_zeros();
        bit_indices[count as usize] = bit_index;
        count += 1;
        moves &= moves - 1; 
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..count);
    let selected_bit_index = bit_indices[random_index as usize];

    board.put_piece(1 << selected_bit_index)
}

#[allow(dead_code)]
pub fn put_eval_zero_simple (board: &mut Board) -> Result<(), PutPieceErr> {
    let legal_moves = board.put_able();
    if legal_moves == 0 {
        return Err(PutPieceErr::NoValidPlacement);
    }
    const SCORES: [i32; 64] = [
        120, -20, 20,  5,  5, 20, -20, 120,
        -20, -40, -5, -5, -5, -5, -40, -20,
         20,  -5, 15,  3,  3, 15,  -5,  20,
          5,  -5,  3,  3,  3,  3,  -5,   5,
          5,  -5,  3,  3,  3,  3,  -5,   5,
         20,  -5, 15,  3,  3, 15,  -5,  20,
        -20, -40, -5, -5, -5, -5, -40, -20,
        120, -20, 20,  5,  5, 20, -20, 120,
    ];

    let mut max_score = i32::MIN;
    let mut moves = legal_moves;
    let mut max_score_index = 0;

    while  moves != 0 {
        let bit_index = moves.trailing_zeros() as usize;
        moves &= moves - 1; // 1番小さい桁の1を0にする。
        let current_score: i32 = SCORES[bit_index];
        if current_score > max_score {
            max_score = current_score;
            max_score_index = bit_index;
        }
    }

    // eprintln!("{}", max_score);
    board.put_piece(1 << max_score_index)
}


pub fn simplest_eval (board: &mut Board) -> i32 {
    // TODO: playerとopponentが逆になっている
    // const SCORES: [i32; 64] = [
    //     120, -40, 20, 10, 10, 20, -40, 120,
    //     -40, -60, -5, -5, -5, -5, -60, -40,
    //      20,  -5, 15,  8,  8, 15,  -5,  20,
    //      10,  -5,  8,  8,  8,  8,  -5,  10,
    //      10,  -5,  8,  8,  8,  8,  -5,  10,
    //      20,  -5, 15,  8,  8, 15,  -5,  20,
    //     -40, -60, -5, -5, -5, -5, -60, -40,
    //     120, -40, 20, 10, 10, 20, -40, 120,
    // ];
    const SCORES: [i32; 64] = [
        120, -40,  8,  8,  8,  8, -40, 120,
        -40, -60, -5, -4, -4, -5, -60, -40,
         8,  -5,  -1, -2, -2, -1,  -5,   8,
         8,  -4,  -2, -1, -1, -2,  -4,   8,
         8,  -4,  -2, -1, -1, -2,  -4,   8,
         8,  -5,  -1, -2, -2, -1,  -5,   8,
        -40, -60, -5, -4, -4, -5, -60, -40,
        120, -40,  8,  8,  8,  8, -40, 120,
    ];

    let m1 = [0x7E00000000000000u64, 0x1010101010100, 0x80808080808000, 0x7e];
    let m2 = [0x8100000000000000u64, 0x100000000000001, 0x8000000000000080, 0x81];


    let mut place_score = 0;

    let player_board = board.bit_board[board.next_turn ^1];
    let opponent_board = board.bit_board[board.next_turn];

    for i in 0..4 {
        if ((player_board & m1[i]) | (opponent_board & m2[i])) == m1[i] {
            place_score += 120;
        }
        if ((opponent_board & m1[i]) | (player_board & m2[i])) == m1[i] {
            place_score -= 120;
        }
        let side = (m1[i] | m2[i]);
        if side & (player_board | opponent_board) == side {
            place_score += ((player_board & side).count_ones() as i32 - (opponent_board & side).count_ones() as i32)  * 20;
        }

    }
    
    let mut player_board_bit = player_board;
    let mut opponent_board_bit = opponent_board;
    while  player_board_bit != 0 {
        let bit_index = player_board_bit.trailing_zeros() as usize;
        player_board_bit &= player_board_bit - 1; // 1番小さい桁の1を0にする。
        place_score += SCORES[bit_index];
    }
    while  opponent_board_bit != 0 {
        let bit_index = opponent_board_bit.trailing_zeros() as usize;
        opponent_board_bit &= opponent_board_bit - 1; // 1番小さい桁の1を0にする。
        place_score -= SCORES[bit_index];
    }


    let player_piece_count = player_board.count_ones() as i32;
    let opponent_piece_count = opponent_board.count_ones() as i32;
    let piece_count_score = 
        if player_piece_count + opponent_piece_count < 50 {
            opponent_piece_count - player_piece_count
        } else {(player_piece_count - opponent_piece_count) * (player_piece_count + opponent_piece_count - 50) * 2};
    
    let opponent_mobility = board.put_able().count_ones() as i32;
    board.next_turn = board.next_turn ^ 1;
    let player_mobility = board.put_able().count_ones() as i32;
    board.next_turn = board.next_turn ^ 1;
    let mobility_score = player_mobility - opponent_mobility;

    //// eprintln!("{}, {}, {}", score * 10, (player_mobility * 60 - opponent_mobility * 50), (opponent_piece_count - player_piece_count ) * 30);
    (place_score * 10 + mobility_score * 85 + piece_count_score * 40) / 40

}


pub fn put_eval_one_simple (board: &mut Board) -> Result<(), PutPieceErr> {
    let legal_moves = board.put_able();
    if legal_moves == 0 {
        return Err(PutPieceErr::NoValidPlacement);
    }

    let mut max_score = i32::MIN;
    let mut max_score_put_place = 0;
    let mut moves = legal_moves;
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        virt_board.put_piece(put_place)?;   
        let current_score: i32 = simplest_eval(&mut virt_board);
        if current_score > max_score {
            max_score = current_score;
            max_score_put_place = put_place;
        }
    }

    // eprintln!("{}", max_score);

    board.put_piece(max_score_put_place)

}


pub fn mid_game_solver_nega_alpha(board: &Board, depth: i32) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }

    
    let mut alpha = -SCORE_INF;
    let mut max_score_move = 0u64;
    let beta = SCORE_INF;
    
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        let _ = virt_board.put_piece(put_place);
        let this_score = -nega_alpha_mid_game(&mut virt_board, -beta, -alpha, depth - 1);
        // eprintln!("this_score: {}",this_score);
        if this_score > alpha {
            alpha = this_score;
            max_score_move = put_place;
        }
    }
    // unsafe { 
    //     eprintln!("searched nodes: {}", TCOUNT);
    // }
    // eprintln!("full solver: {}", alpha);
    max_score_move
} 

pub fn mid_game_solver_nega_alpha_variation(board: &Board, depth: i32, variation: i32) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }

    
    let alpha = -SCORE_INF;
    let beta = SCORE_INF;
    
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}

    let mut move_scores = BTreeMap::new();

    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        let _ = virt_board.put_piece(put_place);
        let this_score = -nega_alpha_mid_game(&mut virt_board, -beta, -alpha, depth - 1);
        // eprintln!("this_score: {}",this_score);
        move_scores.insert(this_score, put_place);
    }
    // unsafe { 
    //     eprintln!("searched nodes: {}", TCOUNT);
    // }
    let max_score = move_scores.last_key_value().unwrap().0;
    let mut candidate_move = Vec::new();
    let lower_bound = max_score - variation; // Ensure no underflow
    for (&s, &m) in move_scores.range(lower_bound..) {
        candidate_move.push((s,m));
    }
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..candidate_move.len());
    //// eprintln!("mid solver: {}", max_score);
    //// eprintln!("mid solver put: {}", candidate_move[random_index].0);
    candidate_move[random_index].1
} 

pub fn nega_alpha_mid_game(board: &mut Board, mut alpha: i32,beta: i32, depth_rest: i32) -> i32{

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    let mut best_score = i32::MIN;
    unsafe {TCOUNT += 1;}

    if depth_rest <= 0 {
        return -simplest_eval(board);
    }

    while moves != 0 {
        let mut current_board = board.clone();
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        current_board.put_piece_fast(put_place);
        let score = -nega_alpha_mid_game(&mut current_board, -beta, -alpha, depth_rest - 1);
        if score >= beta {
            return score;
        }
        alpha = alpha.max(score);
        best_score = best_score.max(score);
    }

    if best_score == i32::MIN {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            // return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
            board.next_turn ^= 1;
            return -simplest_eval(board);
        }
        return -nega_alpha_mid_game(board, -beta, -alpha, depth_rest - 1);
    }

    best_score
}


pub fn mid_game_solver_nega_alpha_move_ordering(board: &Board, depth: i32) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }
    
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}

    // move ordering
    let mut put_board: Vec<(i32, Board, u64)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e = -nega_alpha_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, depth - 3);
        put_board.push((e, current_put_board, put_place));

    }

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut max_score_move = 0u64;
    
    put_board.sort_unstable_by(|(a,_, _), (b, _, _)| b.partial_cmp(a).unwrap());

    
    for (_,current_put_board, put_place) in put_board.iter_mut() {
        let score =-nega_alpha_move_ordering_mid_game(current_put_board, -beta, -alpha, depth - 1);
        // eprintln!("this_score: {}",score);
        if score > alpha {
            alpha = score;
            max_score_move = *put_place;
        }
    }

    // unsafe {
    //     eprintln!("searched nodes: {}", TCOUNT);
    // }
    // eprintln!("full solver: {}", alpha);
    max_score_move
} 



pub fn nega_alpha_move_ordering_mid_game(board: &mut Board, mut alpha: i32,beta: i32, depth_rest: i32) -> i32{

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {TCOUNT += 1;}

    if depth_rest < 4  {
        unsafe {TCOUNT -= 1;}
        return nega_alpha_mid_game(board, alpha, beta, depth_rest);
    }

    // move ordering
    
    let mut put_board: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e = -nega_alpha_move_ordering_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, depth_rest - 3);
        put_board.push((e, current_put_board));

    }
    put_board.sort_unstable_by(|(a,_), (b, _)| b.partial_cmp(a).unwrap());
    
    let mut best_score = i32::MIN;
    for (_,current_put_board) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering_mid_game(current_put_board, -beta, -alpha, depth_rest - 1);
        if score >= beta {
            return score;
        }
        alpha = alpha.max(score);
        best_score = best_score.max(score);
    }

    if best_score == i32::MIN {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            board.next_turn ^= 1;
            return  -simplest_eval(board);
        }
        return -nega_alpha_move_ordering_mid_game(board, -beta, -alpha,depth_rest - 1);
    }
    best_score
}



pub fn mid_game_solver_nega_scout(board: &Board, depth: i32) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }
    
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}

    // move ordering
    let mut put_boards: Vec<(i32, Board, u64)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e = -nega_scout_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, depth - 3);
        put_boards.push((e, current_put_board, put_place));
    }

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut max_score_move = 0u64;
    
    put_boards.sort_unstable_by(|(a,_, _), (b, _, _)| b.partial_cmp(a).unwrap());

    let mut put_boards_iter = put_boards.iter_mut();
    let first_child_board = put_boards_iter.next().unwrap();
    alpha = -nega_scout_mid_game(&mut first_child_board.1, -beta, -alpha, depth - 1);
    max_score_move = first_child_board.2;

    for (_,current_put_board, put_place) in put_boards_iter {
        let mut score =-nega_alpha_move_ordering_mid_game(current_put_board, -alpha - 1, -alpha, depth - 1);
        // eprintln!("this_score: {}",score);
        if score > alpha {
            alpha = score;
            score = -nega_scout_mid_game(current_put_board, -beta, -alpha, depth - 1);
            alpha = alpha.max(score);
            max_score_move = *put_place;
        }
    }

    // unsafe {
    //     eprintln!("searched nodes: {}", TCOUNT);
    // }
    // eprintln!("full solver: {}", alpha);
    max_score_move
} 


pub fn nega_scout_mid_game(board: &mut Board, mut alpha: i32,beta: i32, depth_rest: i32) -> i32{

    if depth_rest < 6  {
        return nega_alpha_mid_game(board, alpha, beta, depth_rest);
    }
    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {TCOUNT += 1;}

    if moves == 0 {
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            board.next_turn ^= 1;
            return  -simplest_eval(board);
        }
        return -nega_scout_mid_game(board, -beta, -alpha,depth_rest - 1);
    }

    // move ordering
    
    let mut put_boards: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e = -nega_scout_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, depth_rest - 3);
        put_boards.push((e, current_put_board));

    }
    if put_boards.len() > 1{
        put_boards.sort_unstable_by(|(a,_), (b, _)| b.partial_cmp(a).unwrap());
    }
    
    let mut put_boards_iter = put_boards.iter_mut();
    let first_child_board = put_boards_iter.next().unwrap();
    let mut best_score =  -nega_scout_mid_game(&mut first_child_board.1, -beta, -alpha, depth_rest - 1);
    if best_score >= beta { return best_score; }
    alpha = alpha.max(best_score);
    
    for (_,current_put_board) in put_boards_iter {
        let mut score = -nega_alpha_move_ordering_mid_game(current_put_board, -alpha - 1, -alpha, depth_rest - 1);
        if score >= beta { return score; }
        if alpha < score {
            alpha = score;
            score = -nega_scout_mid_game(current_put_board, -beta, -alpha, depth_rest - 1);
            if score >= beta { return score; }
            alpha = alpha.max(score);
        }
        best_score = best_score.max(score);
    }


    alpha
}


// pub fn mid_game_solver_nega_alpha_board_pattarn(board: &Board, eval: &Evaluator, depth: i32) -> u64{
//     let mut moves = board.put_able();
//     if moves == 0 {
//         return 0;
//     }

//     const SCORE_INF: i32 = 100000000i32;
//     let mut alpha = -SCORE_INF;
//     let mut max_score_move = 0u64;
//     let beta = SCORE_INF;
    
//     // eprintln!("my_turn: {}", board.next_turn);
//     unsafe {TCOUNT = 0;}
//     while  moves != 0 {
//         let mut virt_board = board.clone();
//         let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
//         moves &= moves - 1; // 最も小さい位のbitを消す
//         let _ =virt_board.put_piece(put_place);
//         //let this_score = -nega_alpha_mid_game_board_pattarn(&mut virt_board, eval, -beta, -alpha, depth - 1);
//         let this_score = -nega_alpha_mid_game_board_pattarn(&mut virt_board, eval, -beta, SCORE_INF, depth - 1);
//         eprintln!("this_score: {}, {}",this_score, put_place);
//         if this_score > alpha {
//             alpha = this_score;
//             max_score_move = put_place;
//         }
//     }
//     // unsafe { 
//     //     eprintln!("searched nodes: {}", TCOUNT);
//     // }
//     eprintln!("mid solver: {}", alpha);
//     max_score_move
// } 
// pub fn nega_alpha_mid_game_board_pattarn(board: &mut Board, eval: &Evaluator, mut alpha: i32,beta: i32, depth_rest: i32) -> i32{

//     if depth_rest <= 0 {
//         return eval.eval_from_board_pattern(&board) as i32;
//     }
//     // 探索範囲: [alpha, beta]
//     let mut moves = board.put_able();
//     let mut best_score = i32::MIN;
//     unsafe {TCOUNT += 1;}


//     while moves != 0 {
//         let mut current_board = board.clone();
//         let put_place = (!moves + 1) & moves;
//         moves &= moves - 1;
//         current_board.put_piece_fast(put_place);
//         let score = -nega_alpha_mid_game_board_pattarn(&mut current_board, eval, -beta, -alpha, depth_rest - 1);
//         if score >= beta {
//             return score;
//         }
//         alpha = alpha.max(score);
//         best_score = best_score.max(score);
//     }

//     if best_score == i32::MIN {
//         board.next_turn ^= 1; //pass
//         if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
//             // return  board.bit_board[board.next_turn  ^ 1].count_ones() as i32 - board.bit_board[board.next_turn].count_ones() as i32;
//             return -eval.eval_from_board_pattern(&board) as i32;
//         }
//         return -nega_alpha_mid_game_board_pattarn(board, eval, -beta, -alpha, depth_rest - 1);
//     }

//     best_score
// }