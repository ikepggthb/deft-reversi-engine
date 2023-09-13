
use crate::board::*;
use rand::Rng;
pub static mut tcount: i64 = 0;

pub fn end_game_full_solver_negamax(board: &Board) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }
    let mut max_score = -64;
    let mut max_score_move = 0u64;
    
    eprintln!("my_turn: {}", board.next_turn);
    unsafe {tcount = 0;}
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        virt_board.put_piece(put_place);
        let this_score = - negamax(&mut virt_board);
        eprintln!("this_score: {}",this_score );
        if this_score > max_score {
            max_score = this_score;
            max_score_move = put_place;
        }
    }
    unsafe { eprintln!("searched nodes: {}", tcount);}
    eprintln!("full solver: {}", max_score);
    max_score_move
} 

pub fn negamax(board: &mut Board) -> i32{

    let mut moves = board.put_able();
    let mut best_score = i32::MIN;
    unsafe {tcount += 1;}
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

pub fn end_game_full_solver_nega_alpha(board: &Board) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }

    const score_inf: i32 = 100000i32;
    let mut alpha = -score_inf;
    let mut max_score_move = 0u64;
    let beta = score_inf;
    
    eprintln!("my_turn: {}", board.next_turn);
    unsafe {tcount = 0;}
    while  moves != 0 {
        let mut virt_board = board.clone();
        let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
        moves &= moves - 1; // 最も小さい位のbitを消す
        virt_board.put_piece(put_place);
        let this_score = -nega_alpha(&mut virt_board, -beta, -alpha);
        eprintln!("this_score: {}",this_score);
        if this_score > alpha {
            alpha = this_score;
            max_score_move = put_place;
        }
    }
    unsafe { eprintln!("searched nodes: {}", tcount);}
    eprintln!("full solver: {}", alpha);
    max_score_move
} 


pub fn nega_alpha(board: &mut Board, mut alpha: i32,beta: i32) -> i32{

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    let mut best_score = i32::MIN;
    unsafe {tcount += 1;}

    
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


pub fn end_game_full_solver_nega_alpha_move_ordering(board: &Board) -> u64{
    let mut moves = board.put_able();
    if moves == 0 {
        return 0;
    }
    const score_inf: i32 = 100000i32;
    eprintln!("my_turn: {}", board.next_turn);
    unsafe {tcount = 0;}

    // move ordering
    let mut put_board: Vec<(i32, Board, u64)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        put_board.push((current_put_board.put_able() as i32, current_put_board, put_place));
    }

    let mut alpha = -score_inf;
    let beta = score_inf;
    let mut max_score_move = 0u64;
    
    put_board.sort_unstable_by(|(a,_, _), (b, _, _)| a.partial_cmp(b).unwrap());

    
    for (_,current_put_board, put_place) in put_board.iter_mut() {
        let score = -nega_alpha_move_ordering(current_put_board, -beta, -alpha);
        eprintln!("this_score: {}",score);
        if score > alpha {
            alpha = score;
            max_score_move = *put_place;
        }
    }

    unsafe { eprintln!("searched nodes: {}", tcount);}
    eprintln!("full solver: {}", alpha);
    max_score_move
} 


pub fn nega_alpha_move_ordering(board: &mut Board, mut alpha: i32,beta: i32) -> i32{

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    unsafe {tcount += 1;}

    if board.bit_board[Board::BLACK].count_ones() + board.bit_board[Board::WHITE].count_ones() >= 58  {
        unsafe {tcount -= 1;}
        return nega_alpha(board, alpha, beta);
    }

    // move ordering
    let mut put_board: Vec<(i32, Board)> = Vec::with_capacity(moves.count_ones() as usize);
    while moves != 0 {
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        let mut current_put_board = board.clone();
        current_put_board.put_piece_fast(put_place);
        put_board.push((current_put_board.put_able() as i32, current_put_board));

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

    eprintln!("{}", max_score);
    board.put_piece(1 << max_score_index)
}

pub fn simplest_eval (board: &Board, turn: usize) -> i32 {
    const SCORES: [i32; 64] = [
        120, -40, 20, 10, 10, 20, -40, 120,
        -40, -60, -5, -5, -5, -5, -60, -40,
         20,  -5, 15,  3,  3, 15,  -5,  20,
         10,  -5,  3,  3,  3,  3,  -5,  10,
         10,  -5,  3,  3,  3,  3,  -5,  10,
         20,  -5, 15,  3,  3, 15,  -5,  20,
        -40, -60, -5, -5, -5, -5, -60, -40,
        120, -40, 20, 10, 10, 20, -40, 120,
    ];

    let mut score = 0;
    let mut p_bit = board.bit_board[turn];
    let mut n_bit = board.bit_board[turn ^ 1];
    while  p_bit != 0 {
        let bit_index = p_bit.trailing_zeros() as usize;
        p_bit &= p_bit - 1; // 1番小さい桁の1を0にする。
        score += SCORES[bit_index];
    }
    while  n_bit != 0 {
        let bit_index = n_bit.trailing_zeros() as usize;
        n_bit &= n_bit - 1; // 1番小さい桁の1を0にする。
        score -= SCORES[bit_index];
    }

    let mut board_for_m = board.clone();
    // let mask = 0b10111101_00111100_11111111_11111111_11111111_11111111_00111100_10111101_u64;
    let mask = 0b11111111_10111101_11111111_11111111_11111111_11111111_10111101_11111111_u64;
    let opponent_mobility = (board_for_m.put_able() & mask).count_ones() as i32;
    board_for_m.next_turn = board_for_m.next_turn ^ 1;
    let player_mobility = (board_for_m.put_able() & mask).count_ones() as i32;
    board_for_m.next_turn = board_for_m.next_turn ^ 1;
    score + (player_mobility - opponent_mobility) * 7

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
        let current_score: i32 = simplest_eval(&virt_board,board.next_turn);
        if current_score > max_score {
            max_score = current_score;
            max_score_put_place = put_place;
        }
    }

    eprintln!("{}", max_score);

    board.put_piece(max_score_put_place)

}


