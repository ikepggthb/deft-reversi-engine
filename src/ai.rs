
use crate::board::*;
use rand::Rng;
static mut tcount: i64 = 0;

// pub fn end_game_full_solver(board: &Board) -> u64{
//     let mut moves = board.put_able();
//     if moves == 0 {
//         return 0;
//     }

//     let dfs = |first_board: Board, now_turns: usize| -> i32{

//         let mut min_score = 64;

//         let mut stack: Vec<Board> = Vec::new();
//         stack.push(first_board);

//         while let Some(mut current_board) = stack.pop() {
//             let mut moves = current_board.put_able();
//             if moves == 0 {
//                 current_board.next_turn ^= 1;
//                 moves = current_board.put_able();
//                 if moves == 0 {
//                     min_score = min_score.min(current_board.bit_board[now_turns].count_ones() as i32 - current_board.bit_board[now_turns ^ 1].count_ones() as i32);
//                 }
//             }
//             while  moves != 0 {
//                 let mut virt_board = current_board.clone();
//                 let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
//                 moves &= moves - 1; // 最も小さい位のbitを消す
//                 virt_board.put_piece(put_place);
//                 stack.push(virt_board);
//             }
//         }
//         min_score
//     };
    
//     let mut max_score = -64;
//     let mut max_score_move = 0u64;
//     while  moves != 0 {
//         let mut virt_board = board.clone();
//         let put_place = (!moves + 1) & moves; //最も小さい位のbitをマスクする
//         moves &= moves - 1; // 最も小さい位のbitを消す
//         virt_board.put_piece(put_place);
//         let this_score = dfs(virt_board.clone(), board.next_turn);
//         if this_score > max_score {
//             max_score = this_score;
//             max_score_move = put_place;
//         }

//     }
//     eprintln!("full solver: {}", max_score);
//     max_score_move
// } 

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

pub fn nega_alpha(board: &mut Board, alpha: i32, beta: i32) -> i32{

    // 探索範囲: [alpha, beta]
    let mut moves = board.put_able();
    let mut best_score = i32::MIN;
    unsafe {tcount += 1;}
    while moves != 0 {
        let mut current_board = board.clone();
        let put_place = (!moves + 1) & moves;
        moves &= moves - 1;
        current_board.put_piece_fast(put_place);
        let score = -nega_alpha(&mut current_board, alpha, beta);
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
        return -nega_alpha(board, -alpha, -beta);
    }

    best_score
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
        120, -20, 20,  5,  5, 20, -20, 120,
        -20, -40, -5, -5, -5, -5, -40, -20,
         20,  -5, 15,  3,  3, 15,  -5,  20,
          5,  -5,  3,  3,  3,  3,  -5,   5,
          5,  -5,  3,  3,  3,  3,  -5,   5,
         20,  -5, 15,  3,  3, 15,  -5,  20,
        -20, -40, -5, -5, -5, -5, -40, -20,
        120, -20, 20,  5,  5, 20, -20, 120,
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

    score
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


