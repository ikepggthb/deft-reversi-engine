
use std::collections::BTreeMap;

use crate::board::*;
use rand::Rng;
pub static mut TCOUNT: i64 = 0;

const SCORE_INF: i32 = 100000i32;


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


    const SCORES: [i32; 64] = [
        120, -40,  1,  0,  0,  1, -40, 120,
        -40, -60, -5, -4, -4, -5, -60, -40,
         1,  -5,  -1, -2, -2, -1,  -5,   1,
         0,  -4,  -2, -1, -1, -2,  -4,   0,
         0,  -4,  -2, -1, -1, -2,  -4,   0,
         1,  -5,  -1, -2, -2, -1,  -5,   1,
        -40, -60, -5, -4, -4, -5, -60, -40,
        120, -40,  0,  0,  0,  0, -40, 120,
    ];


    let m1 = [0x7E00000000000000u64, 0x1010101010100, 0x80808080808000, 0x7e];
    let m2 = [0x8100000000000000u64, 0x100000000000001, 0x8000000000000080, 0x81];


    let mut place_score = 0;

    let player_board = board.bit_board[(board.next_turn ^1) as usize];
    let opponent_board = board.bit_board[board.next_turn  as usize];

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
        if player_piece_count + opponent_piece_count < 40 {
            opponent_piece_count - player_piece_count
        } else {0};
    
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

    if depth_rest < 6  {
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
        let e = -nega_alpha_move_ordering_mid_game(&mut current_put_board.clone(), -SCORE_INF, SCORE_INF, 2);
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
        let e = -nega_scout_mid_game(&mut current_put_board.clone(), -SCORE_INF, SCORE_INF, depth - 3);
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

    let ordering_depth;
    if depth_rest > 10 {
        ordering_depth = 4;
    } else if depth_rest > 8 {
        ordering_depth = 2;
    }else{
        ordering_depth = 2;
    }
    
    let mut put_boards: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);

    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e = -nega_scout_mid_game(&mut current_put_board.clone(), -SCORE_INF, SCORE_INF, ordering_depth);
        put_boards.push((e, current_put_board));
    }
    if put_boards.len() > 2{
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
            best_score = score;
        }
        alpha = alpha.max(score)
    }


    best_score
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