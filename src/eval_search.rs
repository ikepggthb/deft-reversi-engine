use crate::board::*;
use crate::search::*;

use crate::ai::*;
use crate::t_table;

const SCORE_INF: i32 = 10000;
const MOVE_ORDERING_EVAL_LEVEL: i32 = 2;
const MOVE_ORDERING_EVAL_LEVEL_SIMPLE_SEARCH: i32 = 1;
const SWITCH_SIMPLE_SEARCH_LEVEL: i32 = 8;
const SWITCH_NEGAALPHA_SEARCH_LEVEL: i32 = 7;


pub fn negaalpha_eval_for_move_ordering(board: &Board, mut alpha: i32, beta: i32, lv: i32) -> i32
{    
    if lv <= 0 {
        return simplest_eval(board);
    }

    let mut legal_moves = board.put_able();

    // 合法手がない
    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            return  -simplest_eval(&board);
        }
        return -negaalpha_eval_for_move_ordering(&board, -beta, -alpha, lv);
    }
    
    // 探索範囲: [alpha, beta]
    let mut best_score = -SCORE_INF;

    while legal_moves != 0 {
        let mut current_board = board.clone();
        let put_place = (!legal_moves + 1) & legal_moves;
        legal_moves &= legal_moves - 1; // bitを削除
        current_board.put_piece_fast(put_place);
        let score = -negaalpha_eval_for_move_ordering(&current_board, -beta, -alpha, lv - 1);
        if score >= beta {
            return score;
        }
        if score > alpha {alpha = score};
        if score > best_score {best_score = score}; 
    }

    best_score
}

pub fn negaalpha_eval(board: &Board, mut alpha: i32, beta: i32, lv: i32, search: &mut Search) -> i32
{    
    if lv == 0 {
        search.node_count += 1;
        search.leaf_node_count += 1;
        return simplest_eval(board);
    }

    let mut legal_moves = board.put_able();

    // 合法手がない
    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            search.node_count += 1;
            search.leaf_node_count += 1;
            return  -simplest_eval(&board);
        }
        return -negaalpha_eval(&board, -beta, -alpha, lv, search);
    }
    
    // 探索範囲: [alpha, beta]
    search.node_count += 1;
    let mut best_score = -SCORE_INF;

    while legal_moves != 0 {
        let mut current_board = board.clone();
        let put_place = (!legal_moves + 1) & legal_moves;
        legal_moves &= legal_moves - 1; // bitを削除
        current_board.put_piece_fast(put_place);
        let score = -negaalpha_eval(&current_board, -beta, -alpha, lv - 1, search);
        if score >= beta {
            return score;
        }
        if score > alpha {alpha = score};
        if score > best_score {best_score = score}; 
    }

    best_score
}


pub fn nws_eval_simple(board: &Board, mut alpha: i32, lv: i32, search: &mut Search) -> i32
{
    let mut beta = alpha + 1;

    if lv < SWITCH_NEGAALPHA_SEARCH_LEVEL {
        return negaalpha_eval(board, alpha, beta, lv, search);
    }

    // 探索範囲: [alpha, beta]
    let legal_moves: u64 = board.put_able();

    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            board.next_turn ^= 1;
            search.node_count += 1;
            search.leaf_node_count += 1;
            return simplest_eval(&board);
        }
        search.node_count += 1;
        return -nws_eval_simple(&board, -beta, lv, search);
    }

    search.node_count += 1;

    // move ordering
    let put_boards = move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL_SIMPLE_SEARCH);

    let mut this_node_alpha = alpha;
    let mut best_score = i32::MIN;
    for current_put_board in put_boards.iter() {
        let current_put_board = &current_put_board.board;
        let score = -nws_eval_simple(current_put_board, -beta, lv - 1, search);
        if score >= beta {
            return score;
        }
        if score > this_node_alpha {this_node_alpha = score};
        if score > best_score {best_score = score}; 
    }

    best_score
}

pub fn pvs_eval_simple(board: &Board, mut alpha: i32,mut beta: i32, lv: i32, search: &mut Search) -> i32
{   
    if lv < SWITCH_NEGAALPHA_SEARCH_LEVEL {
        return negaalpha_eval(board, alpha, beta, lv, search);
    }

    if alpha > beta { panic!()};

    // 探索範囲: [alpha, beta]
    let legal_moves = board.put_able();

    // pass or end ?
    if legal_moves == 0 { // 合法手がないならば
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても合法手がない -> ゲーム終了
            board.next_turn ^= 1;
            search.node_count += 1;
            search.leaf_node_count += 1;
            return simplest_eval(&mut board);
        }

        // passしたら、合法手がある -> 探索を続ける
        search.node_count += 1;
        return -pvs_eval_simple(&board, -beta, -alpha, lv, search);
    }

    search.node_count += 1;

    // move ordering
    let put_boards =  move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL_SIMPLE_SEARCH);

    let mut put_boards_iter = put_boards.iter();
    
    let mut this_node_alpha = alpha;
    let mut best_score; //  =  - inf

    // first move
    let first_child_board = put_boards_iter.next().unwrap();
    best_score =  -pvs_eval_simple(&first_child_board.board, -beta, -this_node_alpha, lv - 1, search);
    if best_score >= beta {
        return best_score;
    }
    if best_score > this_node_alpha { this_node_alpha = best_score};

    // other move
    for current_put_board in put_boards_iter {
        let current_put_board = &current_put_board.board;
        let mut score = -nws_eval_simple(current_put_board, -this_node_alpha - 1, lv - 1, search);
        if score >= beta {
            return score;
        }
        if score > best_score {
            if score > this_node_alpha {this_node_alpha = score};
            // 再探索
            score = -pvs_eval_simple(current_put_board, -beta, -this_node_alpha, lv - 1, search);
            if score >= beta { 
                return score;
             }
            best_score = score;
            if score > this_node_alpha {this_node_alpha = score};
        }
    }

    best_score
}



pub fn nws_eval(board: &Board, mut alpha: i32, lv: i32, search: &mut Search) -> i32
{
    let mut beta = alpha + 1;

    if lv < SWITCH_SIMPLE_SEARCH_LEVEL {
        return nws_eval_simple(board, alpha, lv, search);
    }

    if let None = search.t_table {
        return nws_eval_simple(board, alpha, lv, search)
    }

    // 探索範囲: [alpha, beta]
    let legal_moves: u64 = board.put_able();

    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            board.next_turn ^= 1;
            search.node_count += 1;
            search.leaf_node_count += 1;
            return simplest_eval(&board);
        }
        search.node_count += 1;
        return -nws_eval(&board, -beta, lv, search);
    }

    search.node_count += 1;


    if let Some(score) = t_table_cut_off(board, &mut alpha, &mut beta, search.t_table.as_mut().unwrap()) {
        return score;
    }
    
    // move ordering
    let put_boards = move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL);

    let mut this_node_alpha = alpha;
    let mut best_score = i32::MIN;
    for current_put_board in put_boards.iter() {
        let current_put_board = &current_put_board.board;
        let score = -nws_eval(current_put_board, -beta, lv - 1, search);
        if score >= beta {
            search.t_table.as_mut().unwrap().add(board, score, SCORE_INF);
            return score;
        }
        if score > this_node_alpha {this_node_alpha = score};
        if score > best_score {best_score = score};
    }


    if best_score > alpha {
        search.t_table.as_mut().unwrap().add(board, best_score, best_score);
    } else {
        search.t_table.as_mut().unwrap().add(board, -SCORE_INF, best_score);
    }

    best_score
}

pub fn pvs_eval(board: &Board, mut alpha: i32,mut beta: i32, lv: i32, search: &mut Search) -> i32
{   

    if lv < SWITCH_SIMPLE_SEARCH_LEVEL {
        return pvs_eval_simple(board, alpha, beta, lv, search);
    }

    if alpha > beta { panic!()};

    if let None = search.t_table {
        return pvs_eval_simple(board, alpha, beta, lv, search);
    }

    // 探索範囲: [alpha, beta]
    let legal_moves = board.put_able();

    // pass or end ?
    if legal_moves == 0 { // 合法手がないならば
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても合法手がない -> ゲーム終了
            board.next_turn ^= 1;
            search.node_count += 1;
            search.leaf_node_count += 1;
            return simplest_eval(&board);
        }

        // passしたら、合法手がある -> 探索を続ける
        search.node_count += 1;
        return -pvs_eval(&board, -beta, -alpha, lv, search);
    }

    search.node_count += 1;

    // TranspositionTable Cut off

    // t_tableを、毎回、as_nut().unwarp()で呼び出していることについて

    // 1. `if let Some(t_table) = t_table {...}`` のようにせずに、unwarp()を使用している理由
    //     この地点で `search.t_table` が `None` であることはあり得ない。
    //     関数の始めに `search.t_table` が `None` でないことをチェックしており、
    //     この構造体のフィールドはこの関数のスコープ内で他に変更されていない。
    //     (探索の途中にわざわざ、置換表を削除することはないだろう。)
    //     したがって、`unwrap()` はここでは安全に使用できる。
    //     もし、予期せぬ状態（`search.t_table` が `None` になる）が発生した場合は、
    //     これは重大なプログラムの論理エラーを示している可能性があるため、
    //     むしろパニックによって即座に検出されるべきである。

    // 2. `search.t_table.as_mut().unwrap()`を、
    // `&mut TranspositionTable`として変数に保存して使用しない理由
    //     search.t_table.as_mut().unwrap()を、
    //     "&mut TranspositionTable"として変数に保存して使用することはできない。
    //     なぜなら、 次の盤面を探索する際に、&mut Searchを渡さなければならないからである。
    //     Rustの所有権システムの仕様より、
    //     Search 構造体の TranspositionTable フィールドに対する可変な参照が存在する間、
    //     同じ Search インスタンスに対して別の可変な参照を作成することができない

    if let Some(score) = t_table_cut_off(board, &mut alpha, &mut beta, search.t_table.as_mut().unwrap()) {
        return score;
    }

    // move ordering
    let put_boards =  move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL);

    let mut put_boards_iter = put_boards.iter();
    
    let mut this_node_alpha = alpha;
    let mut best_score; //  =  - inf

    // first move
    let first_child_board = put_boards_iter.next().unwrap();
    best_score =  -pvs_eval(&first_child_board.board, -beta, -this_node_alpha, lv - 1, search);
    if best_score >= beta { 
        search.t_table.as_mut().unwrap().add(board, best_score, SCORE_INF);
        return best_score;
    }
    if best_score > this_node_alpha { this_node_alpha = best_score};

    // other move
    for current_put_board in put_boards_iter {
        let current_put_board = &current_put_board.board;
        let mut score = -nws_eval(current_put_board, -this_node_alpha - 1, lv - 1, search);
        if score >= beta {
            search.t_table.as_mut().unwrap().add(board, score, SCORE_INF);
            return score;
        }
        if score > best_score {
            // 再探索
            if score > this_node_alpha {this_node_alpha = score};
            score = -pvs_eval(current_put_board, -beta, -this_node_alpha, lv - 1, search);
            if score >= beta { 
                search.t_table.as_mut().unwrap().add(board, score, SCORE_INF);
                return score;
             }
             best_score = score;
            if score > this_node_alpha {this_node_alpha = score};
        }
    }

    if best_score > alpha { // alpha < best_score < beta
        search.t_table.as_mut().unwrap().add(board, best_score, best_score);
    } else { // best_score <= alpha
        search.t_table.as_mut().unwrap().add(board, -SCORE_INF, best_score);
    }

    best_score
}



