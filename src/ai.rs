
use std::collections::BTreeMap;

use crate::board::*;
use rand::Rng;
pub static mut TCOUNT: i64 = 0;

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

    const SCORE_INF: i32 = 100000i32;
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

    const SCORE_INF: i32 = 100000i32;
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

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    let mut best_score = i32::MIN;
    unsafe {TCOUNT += 1;}

    
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
        return -nega_alpha(board, -beta, -alpha);
    }

    best_score
}

#[allow(dead_code)]
pub fn end_game_full_solver_nega_alpha_move_ordering_return_detail(board: &Board) -> (u64, i32){
    let mut moves = board.put_able();
    if moves == 0 {
        return (0, board.bit_board[board.next_turn].count_ones() as i32 - board.bit_board[board.next_turn ^ 1].count_ones() as i32);
    }
    const SCORE_INF: i32 = 100000i32;
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
    
    put_board.sort_unstable_by(|(a,_, _), (b, _, _)| a.partial_cmp(b).unwrap());

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

    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }
    const SCORE_INF: i32 = 100000i32;
    // eprintln!("my_turn: {}", board.next_turn);
    unsafe {TCOUNT = 0;}

    // move ordering
    let mut put_board: Vec<(i32, Board, u64)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        let e  = -nega_alpha_move_ordering_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, 8);
        // println!("move_ordering_score: {}",e);
        put_board.push((e, current_put_board, put_place));
    }
    // println!("move_ordering end.");

    let mut alpha = -SCORE_INF;
    let beta = SCORE_INF;
    let mut max_score_move = 0u64;
    
    put_board.sort_unstable_by(|(a,_, _), (b, _, _)| a.partial_cmp(b).unwrap());

    
    for (_,current_put_board, put_place) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering_from_eval(current_put_board, -beta, -alpha);
        // println!("this_score: {}",score);
        if score > alpha {
            alpha = score;
            max_score_move = *put_place;
        }
    }

    // let end = start.elapsed();
    // println!("{}秒経過しました。", end.as_secs_f64());
    // unsafe {
    //     eprintln!("searched nodes: {}", TCOUNT);
    //      eprintln!("nps: {}", TCOUNT as f64/ end.as_secs_f64());
    // }
    // eprintln!("full solver: {}", alpha);


    max_score_move
} 

pub fn nega_alpha_move_ordering_from_eval(board: &mut Board, mut alpha: i32,beta: i32) -> i32{

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {TCOUNT += 1;}
    const SCORE_INF: i32 = 100000i32;

    if board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones() >= 42 {
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
        let e = -nega_alpha_move_ordering_mid_game(&mut current_put_board, -SCORE_INF, SCORE_INF, 2);
        put_board.push((e, current_put_board));

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

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {TCOUNT += 1;}

    if board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones() >= 56  {
        unsafe {TCOUNT -= 1;}
        return nega_alpha(board, alpha, beta);
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
        return -nega_alpha_move_ordering(board, -beta, -alpha);
    }

    best_score
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
        -20, -40, -5, -5,  5, -5, -40, -20,
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
        120, -40, 5, 5, 5, 5, -40, 120,
        -40, -60, -5, -4, -4, -5, -60, -40,
         5,  -5, -1, -2, -2, -1,  -5,  5,
         5,  -4, -2,  -1,  -1, -2,  -4,  5,
         5,  -4, -2,  -1,  -1, -2,  -4,  5,
         5,  -5, -1, -2, -2, -1,  -5,  5,
        -40, -60, -5, -4, -4, -5, -60, -40,
        120, -40, 5, 5, 5, 5, -40, 120,
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
            place_score += ((player_board & side).count_ones() as i32 - (opponent_board & side).count_ones() as i32)  * 15;
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
        if player_piece_count + opponent_piece_count < 55 {
            opponent_piece_count - player_piece_count
        } else {player_piece_count - opponent_piece_count};
    
    let opponent_mobility = board.put_able().count_ones() as i32;
    board.next_turn = board.next_turn ^ 1;
    let player_mobility = board.put_able().count_ones() as i32;
    board.next_turn = board.next_turn ^ 1;
    let mobility_score = 
        if player_piece_count + opponent_piece_count < 52 {
            player_mobility - opponent_mobility
        } else {
            (player_mobility - opponent_mobility) / 3
        };

    //// eprintln!("{}, {}, {}", score * 10, (player_mobility * 60 - opponent_mobility * 50), (opponent_piece_count - player_piece_count ) * 30);
    (place_score * 10 + mobility_score * 80 + piece_count_score * 40) / 40

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

    const SCORE_INF: i32 = 100000i32;
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

    const SCORE_INF: i32 = 100000i32;
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
    const SCORE_INF: i32 = 100000i32;
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
    const SCORE_INF: i32 = 100000i32;
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