use crate::board::*;
use crate::eval::Evaluator;
use crate::search::*;
use crate::t_table::*;

const SCORE_INF: i32 = i32::MAX;

const MOVE_ORDERING_EVAL_LEVEL: i32 = 2;
const MOVE_ORDERING_EVAL_LEVEL_SIMPLE_SEARCH: i32 = 1;
const SWITCH_SIMPLE_SEARCH_LEVEL: i32 = 8;
const SWITCH_NEGAALPHA_SEARCH_LEVEL: i32 = 6;


/// NegaAlpha法を用いて、オセロの盤面の評価値を計算する。
///
/// 探索速度を向上させるため、葉に近いノードで使用される。
/// 
/// # 引数
/// * `board`  - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha`  - 探索の下限値を示すアルファ値。
/// * `beta`   - 探索の上限値を示すベータ値。
/// * `lv`     - 探索レベル (あと何手先まで読むか)
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面のスコアを表す整数値。
///   スコアは現在のプレイヤーから見た盤面のスコアを表す。
///
pub fn negaalpha_eval(board: &Board, mut alpha: i32, beta: i32, lv: i32, search: &mut Search) -> i32
{    
    if lv == 0 {
        search.node_count += 1;
        search.leaf_node_count += 1;
        return search.eval_func.clac_features_eval(board);
    }

    let mut legal_moves = board.put_able();

    // 合法手がない
    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            search.node_count += 1;
            search.leaf_node_count += 1;
            
            board.next_turn ^= 1;
            return search.eval_func.clac_features_eval(&board);
            //return  -implest_eval(&board);
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

/// 関数`pvs_perfect_simple`で用いられるヌルウィンドウ探索（Null Window Search, NWS）
/// 
/// `alpha`から、`alpha + 1`までの範囲で、alpha-beta探索を行う。
///
/// # 引数
/// * `board` - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha` - 探索の下限値を示すアルファ値。
/// * `lv`     - 探索レベル (あと何手先まで読むか)
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへの可変な参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面の評価値を表す整数値。
///   現在のプレイヤーから見た盤面の評価値を表す。
///
/// # 注記
/// * 置換表を使用しない。
/// * 最後の残り数手は、`negaalpha_eval`関数を使用した探索結果を用いる。
///     * 最後の残り数手は、`SWITCH_NEGAALPHA_SEARCH_LEVEL`で定義される。
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
            return search.eval_func.clac_features_eval(&board);
            // return simplest_eval(&board);
        }
        search.node_count += 1;
        return -nws_eval_simple(&board, -beta, lv, search);
    }

    search.node_count += 1;

    // move ordering
    let put_boards = move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL_SIMPLE_SEARCH, search);

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


/// Principal Variation Search (PVS) を用いて、盤面の評価値を計算する。
///
///  PVS(Negascout)について :
///   https://ja.wikipedia.org/wiki/Negascout
///
/// ## 引数
/// * `board`  - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha`  - 探索の下限値を示すアルファ値。
/// * `beta`   - 探索の上限値を示すベータ値。
/// * `lv`     - 探索レベル (あと何手先まで読むか)
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された評価値を表す整数値。
///   スコアは現在のプレイヤーから見た盤面の評価値を表す。
///
/// # 注記
/// * 置換表を使用しない。
/// * 最後の残り数手は、`negaalpha_eval`関数を使用した探索結果を用いる。
///     * 最後の残り数手は、`SWITCH_NEGAALPHA_SEARCH_LEVEL`で定義される。
/// 
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
            return search.eval_func.clac_features_eval(&board);
            // return simplest_eval(&mut board);
        }

        // passしたら、合法手がある -> 探索を続ける
        search.node_count += 1;
        return -pvs_eval_simple(&board, -beta, -alpha, lv, search);
    }

    search.node_count += 1;

    // move ordering
    let put_boards =  move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL_SIMPLE_SEARCH, search);

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


/// 関数`pvs_perfect`で用いられるヌルウィンドウ探索（Null Window Search, NWS）
/// 
/// `alpha`から、`alpha + 1`までの範囲で、alpha-beta探索を行う。
///
/// # 引数
/// * `board` - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha` - 探索の下限値を示すアルファ値。
/// * `lv`     - 探索レベル (あと何手先まで読むか)
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへの可変な参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面の評価値を表す整数値。
///   現在のプレイヤーから見た盤面の評価値を表す。
///
/// # 注記
/// * 置換表が存在しない場合は、`nvs_perfect_simple` 関数に切り替える。
/// * `nws_eval_simple` と大きく異なるところは、置換表を使用していることである。
/// * 最後の残り数手は、`nws_eval_simple`関数を使用した探索結果を用いる。
///     * 最後の残り数手は、`SWITCH_SIMPLE_SEARCH_LEVEL`で定義される。
pub fn nws_eval(board: &Board, mut alpha: i32, lv: i32, search: &mut Search) -> i32
{
    let mut beta = alpha + 1;

    if lv < SWITCH_SIMPLE_SEARCH_LEVEL {
        return nws_eval_simple(board, alpha, lv, search);
    }

    if search.t_table.is_none() {
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
            return search.eval_func.clac_features_eval(&board);
            // return simplest_eval(&board);
        }
        search.node_count += 1;
        return -nws_eval(&board, -beta, lv, search);
    }

    search.node_count += 1;


    if let Some(score) = t_table_cut_off(board, &mut alpha, &mut beta, search.get_mut_t_table().unwrap()) {
        return score;
    }
    
    // move ordering
    let put_boards = move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL, search);

    let mut this_node_alpha = alpha;
    let mut best_score = i32::MIN;
    for current_put_board in put_boards.iter() {
        let current_put_board = &current_put_board.board;
        let score = -nws_eval(current_put_board, -beta, lv - 1, search);
        if score >= beta {
            search.get_mut_t_table().unwrap().add(board, score, SCORE_INF);
            return score;
        }
        if score > this_node_alpha {this_node_alpha = score};
        if score > best_score {best_score = score};
    }

    if best_score > alpha {
        search.get_mut_t_table().unwrap().add(board, best_score, best_score);
    } else {
        search.get_mut_t_table().unwrap().add(board, -SCORE_INF, best_score);
    }

    best_score
}


/// Principal Variation Search (PVS) を用いて、盤面の評価値を計算する。
///
///  PVS(Negascout)について :
///   https://ja.wikipedia.org/wiki/Negascout
///
/// # 引数
/// * `board`  - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha`  - 探索の下限値を示すアルファ値。
/// * `beta`   - 探索の上限値を示すベータ値。
/// * `lv`     - 探索レベル (あと何手先まで読むか)
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された評価値を表す整数値。
///   スコアは現在のプレイヤーから見た盤面の評価値を表す。
///
/// # 例
/// ```
/// let board = Board::new(); // オセロの初期盤面を生成
/// let mut search = Search::new();
/// let alpha = -SCORE_INF; // 初期アルファ値の設定
/// let beta = SCORE_INF; // 初期ベータ値の設定
/// let lv = 10; // 10手先まで読む
/// let score = pvs_eval(&board, alpha, beta, lv, &mut search);
/// println!("Score: {}", score);
/// ```
///
/// # 注記
/// * 置換表が存在しない場合は、`pvs_perfect_simple` 関数に切り替える。
/// * `pvs_eval_simple` と大きく異なることは、置換表を使用していることである。
/// * 最後の残り数手は、`pvs_eval_simple`関数を使用した探索結果を用いる。
///     * 最後の残り数手は、`SWITCH_SIMPLE_SEARCH_LEVEL`で定義される。
/// 
pub fn pvs_eval ( board     : &Board,
                  mut alpha : i32,
                  mut beta  : i32,
                  lv        : i32,
                  search    : &mut Search)
                  -> i32
{   
    if lv < SWITCH_SIMPLE_SEARCH_LEVEL {
        return pvs_eval_simple(board, alpha, beta, lv, search);
    }

    #[cfg(debug_assertions)]
    if alpha > beta { panic!()};

    if search.t_table.is_none() {
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
            return search.eval_func.clac_features_eval(&board);
            // return simplest_eval(&board);
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

    // 2. `search.get_mut_t_table().unwrap()`を、
    // `&mut TranspositionTable`として変数に保存して使用しない理由
    //     search.get_mut_t_table().unwrap()を、
    //     "&mut TranspositionTable"として変数に保存して使用することはできない。
    //     なぜなら、 次の盤面を探索する際に、&mut Searchを渡さなければならないからである。
    //     Rustの所有権システムの仕様より、
    //     Search 構造体の TranspositionTable フィールドに対する可変な参照が存在する間、
    //     同じ Search インスタンスに対しての可変な参照を作成することができない

    if let Some(score) = t_table_cut_off(board, &mut alpha, &mut beta, search.get_mut_t_table().unwrap()) {
        return score;
    }

    // move ordering
    let put_boards =  move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL,  search);

    let mut put_boards_iter = put_boards.iter();
    
    let mut this_node_alpha = alpha;
    let mut best_score; //  =  - inf

    // first move
    let first_child_board = put_boards_iter.next().unwrap();
    best_score =  -pvs_eval(&first_child_board.board, -beta, -this_node_alpha, lv - 1, search);
    if best_score >= beta { 
        search.get_mut_t_table().unwrap().add(board, best_score, SCORE_INF);
        return best_score;
    }
    if best_score > this_node_alpha { this_node_alpha = best_score};

    // other move
    for put_board in put_boards_iter {
        let put_board = &put_board.board;
        let mut score = -nws_eval( put_board, -this_node_alpha - 1, lv - 1, search);
        if score >= beta {
            search.get_mut_t_table().unwrap().add(board, score, SCORE_INF);
            return score;
        }
        if score > best_score {
            // 再探索
            if score > this_node_alpha {this_node_alpha = score};
            score = -pvs_eval(put_board, -beta, -this_node_alpha, lv - 1, search);
            if score >= beta { 
                search.get_mut_t_table().unwrap().add(board, score, SCORE_INF);
                return score;
             }
             best_score = score;
            if score > this_node_alpha {this_node_alpha = score};
        }
    }

    if best_score > alpha { // alpha < best_score < beta
        search.get_mut_t_table().unwrap().add(board, best_score, best_score);
    } else { // best_score <= alpha
        search.get_mut_t_table().unwrap().add(board, -SCORE_INF, best_score);
    }

    best_score
}



