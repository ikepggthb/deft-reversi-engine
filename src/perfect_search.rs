use crate::board::*;
use crate::search::*;

const SCORE_INF: i32 = i32::MAX;

/// 空きマスが残り`SWITCH_EMPTIES_SIMPLE_PVS`以下である場合、
/// `pvs_perfect`から、`pvs_perfect_simple`へ切り替える
const SWITCH_EMPTIES_SIMPLE_PVS: i32 = 8;

/// 空きマスが残り`SWITCH_EMPTIES_SIMPLE_NWS`以下である場合、
/// `nws_perfect`から、`nws_perfect_simple`へ切り替える
const SWITCH_EMPTIES_SIMPLE_NWS: i32 = 8;

/// 空きマスが残り`SWITCH_EMPTIES_NEGA_ALPHA`以下である場合、
/// `pvs_perfect_simple`や`nws_perfect_simple`から、`negaalpha_perfect`へ切り替える
const SWITCH_EMPTIES_NEGA_ALPHA: i32 = 6;

/// `pvs_perfect`, `nws_perfect`でのmove orderingにおいて、
/// 空きマスが`SWITCH_EMPTIES_MOVE_ORDER`より多い場合、
/// 評価関数とNegascout探索を用いた`move_ordering_eval`を使用する。
/// SWITCH_EMPTIES_MOVE_ORDER以下である場合は、`move_ordering_ffs`を使用する。
const SWITCH_EMPTIES_MOVE_ORDER: i32 = 18;

/// `pvs_perfect`, `nws_perfect`でのmove orderingにおいて、評価関数とNegascout探索を用いた`move_ordering_eval`を使用する場合の、探索の深さ
const MOVE_ORDERING_EVAL_LEVEL: i32 = 4;

/// オセロの盤面に基づいて最終スコアを計算
///
/// この関数は、現在のプレイヤーと対戦相手の石の数に基づいて、
/// ゲーム終了時のスコアを計算する。空きマスがある場合は、それらを勝っている側の
/// スコアに加算する。
///
/// # 引数
/// * `board` - スコアを計算するオセロの盤面を表す `Board` オブジェクトの参照。
///
/// # 戻り値
/// * 計算されたゲームの最終スコアを表す整数値。
///
/// # 例
/// ```
/// let board = Board::new(); // ゲーム終了時の盤面を生成
/// let score = solve_score(&board);
/// println!("Final Score: {}", score);
/// ```
///
/// # 注記
/// * スコアは、現在のプレイヤーの石の数から対戦相手の石の数を引いた値である。
/// * 空きマスが存在する場合、それらを勝っている側のスコアに加算する。
#[inline(always)]
pub fn solve_score(board: &Board) -> i32 {
    let n_player = board.bit_board[board.next_turn].count_ones() as i32;
    let n_opponent = board.bit_board[board.next_turn^1].count_ones() as i32;
    let diff = n_player - n_opponent;
    if diff > 0 {
        let n_empties = 64 - n_player - n_opponent;
        diff + n_empties
    } else if diff < 0 {
        let n_empties = 64 - n_player - n_opponent;
        diff - n_empties
    } else {
        0
    }
}

/// 空きマスがないオセロ盤面の最終スコアを高速に計算
///
/// # 引数
/// * `board` - スコアを計算するオセロの盤面(`Board` オブジェクトの参照)。
///
/// # 戻り値
/// * 最終スコアを表す整数値。
///
/// # 例
/// ```
/// let board = Board::new(); // 空きマスがないゲーム終了時の盤面を生成
/// let score = solve_score_0_empties(&board);
/// println!("Final Score: {}", score);
/// ```
///
/// # 注記
/// * この関数は、盤面上に空きマスがない場合にのみ正確なスコアを返します。
/// * スコアは、現在のプレイヤーの石の数から対戦相手の石の数を引いた値だが、
///   現在のプレイヤーの石の数の2倍から64を引いた値を用いることで高速化している。
#[inline(always)]
pub fn solve_score_0_empties(board: &Board) -> i32
{
    2 * (board.bit_board[board.next_turn].count_ones() as i32) - 64
}


/// NegaAlpha法を用いて、完全読みを行い、オセロの盤面のスコアを計算する。
///
/// 探索速度を向上させるため、葉に近いノードで使用される。
/// 
/// # 引数
/// * `board` - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha` - 探索の下限値を示すアルファ値。
/// * `beta` - 探索の上限値を示すベータ値。
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面のスコアを表す整数値。
///   スコアは現在のプレイヤーから見た盤面のスコアを表す。
///
pub fn negaalpha_perfect(board: &Board, mut alpha: i32, beta: i32, search: &mut Search) -> i32
{    
    // 空きマスがない場合
    if (board.bit_board[Board::BLACK] | board.bit_board[Board::WHITE]) == u64::MAX {
        search.node_count += 1;
        search.leaf_node_count += 1;
        return  solve_score_0_empties(board);
    }
    
    let mut legal_moves = board.put_able();

    // 合法手がない
    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            search.node_count += 1;
            search.leaf_node_count += 1;
            return  -solve_score(&board);
        }
        return -negaalpha_perfect(&board, -beta, -alpha, search);
    }
    
    // 探索範囲: [alpha, beta]
    search.node_count += 1;
    let mut best_score = -SCORE_INF;

    while legal_moves != 0 {
        let mut current_board = board.clone();
        let put_place = (!legal_moves + 1) & legal_moves;
        legal_moves &= legal_moves - 1; // bitを削除
        current_board.put_piece_fast(put_place);
        let score = -negaalpha_perfect(&current_board, -beta, -alpha, search);
        if score >= beta {
            return score;
        }
        alpha = alpha.max(score);
        best_score = best_score.max(score);
    }

    best_score
}




/// 関数`pvs_perfect_simple`で用いられるヌルウィンドウ探索（Null Window Search, NWS）
///
/// # 引数
/// * `board` - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha` - 探索の下限値を示すアルファ値。
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面のスコアを表す整数値。
///   スコアは現在のプレイヤーから見た盤面のスコアを表す。
///
/// # 注記
/// * 終盤の局面では、`negaalpha_perfect` 関数に切り替わります。
pub fn nws_perfect_simple(board: &Board, mut alpha: i32, search: &mut Search) -> i32
{    
    // 探索範囲: [alpha, beta]
    let beta = alpha + 1;
    
    if num_of_empties(board) < SWITCH_EMPTIES_NEGA_ALPHA  {
        return negaalpha_perfect(board, alpha, beta, search);
    }

    let legal_moves = board.put_able();

    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            search.node_count += 1;
            search.leaf_node_count += 1;
            board.next_turn ^= 1;
            return  solve_score(&board);
        }
        search.node_count += 1;
        return -nws_perfect_simple(&board, -beta, search);
    }
    

   search.node_count += 1;
    // move ordering
    let put_boards = move_ordering_ffs(board, legal_moves);

    let mut best_score = i32::MIN;
    for current_put_board in put_boards.iter() {
        let current_put_board = &current_put_board.board;
        let score = -nws_perfect_simple(current_put_board, -beta, search);
        if score >= beta {
            return score;
        }
        alpha = alpha.max(score);
        best_score = best_score.max(score);
   }

   best_score
}


/// Principal Variation Search (PVS) を用いて、完全読みを行い、オセロの盤面のスコアを計算する。
/// 
/// `pvs_perfect`とは異なり、探索速度を優先するため、置換表を使用しない。
/// 浅い探索で用いられる。
/// 
///  PVS(Negascout)について :
///   https://ja.wikipedia.org/wiki/Negascout
/// 
/// 
/// # 引数
/// * `board` - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha` - 探索の下限値を示すアルファ値。
/// * `beta` - 探索の上限値を示すベータ値。
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面のスコアを表す整数値。
///   スコアは現在のプレイヤーから見た盤面のスコアを表す。
pub fn pvs_perfect_simple(board: &Board, alpha: i32,beta: i32, search: &mut Search) -> i32{

    if num_of_empties(board) < SWITCH_EMPTIES_NEGA_ALPHA  {
        return negaalpha_perfect(board, alpha, beta, search);
    }

    // 探索範囲: [alpha, beta]
    let legal_moves = board.put_able();

    if legal_moves == 0 {
        let mut board = board.clone();
        board.next_turn ^= 1; //pass
        if board.put_able() == 0 { // passしても置くところがない == ゲーム終了
            board.next_turn ^= 1;
            search.node_count += 1;
            search.leaf_node_count += 1;
            return  solve_score(&board);
        }
        search.node_count += 1;
        return -pvs_perfect_simple(&board, -beta, -alpha, search);
    }
    
    search.node_count += 1;

    // move ordering
    let mut put_boards = move_ordering_ffs(board, legal_moves);
    let mut put_boards_iter = put_boards.iter_mut();
    
    let mut this_node_alpha = alpha;
    let mut best_score;
    
    // first move
    let first_child_board = put_boards_iter.next().unwrap();
    best_score =  -pvs_perfect_simple(&first_child_board.board, -beta, -this_node_alpha, search);
    if best_score >= beta { 
        return best_score;
    }
    this_node_alpha = this_node_alpha.max(best_score);

    // other move
    for current_put_board in put_boards_iter {
        let current_put_board = &current_put_board.board;
        let mut score = -nws_perfect_simple(current_put_board, -this_node_alpha - 1, search);
        if score >= beta {
            return score;
        }
        if best_score < score {
            this_node_alpha = this_node_alpha.max(score);
            score = -pvs_perfect_simple(current_put_board, -beta, -this_node_alpha, search);
            if beta <= score { 
                return score;
            }
            best_score = score;
        }
        this_node_alpha = this_node_alpha.max(score);

    }

    best_score
}


/// 関数`pvs_perfect`で用いられるヌルウィンドウ探索（Null Window Search, NWS）
///
/// # 引数
/// * `board` - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha` - 探索の下限値を示すアルファ値。
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面のスコアを表す整数値。
///   スコアは現在のプレイヤーから見た盤面の評価値を表す。
///
/// # 注記
/// * 終盤の局面では、`negaalpha_perfect` 関数に切り替わります。

pub fn nws_perfect(board: &Board, mut alpha: i32, search: &mut Search) -> i32
{
    let mut beta = alpha + 1;

    let n_empties = num_of_empties(board);
    if n_empties < SWITCH_EMPTIES_SIMPLE_NWS  {
        return nws_perfect_simple(board, alpha, search);
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
            return solve_score(&board);
        }
        search.node_count += 1;
        return -nws_perfect(&board, -beta, search);
    }

    search.node_count += 1;

    if let Some(score) = t_table_cut_off(board, &mut alpha, &mut beta, &search.t_table) {
        return score;
    }

    // move ordering
    let put_boards = {
        if n_empties > SWITCH_EMPTIES_MOVE_ORDER {
            move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL)
        } else {
            move_ordering_ffs(board, legal_moves)
        }
    };

    let mut this_node_alpha = alpha;
    let mut best_score = i32::MIN;
    for current_put_board in put_boards.iter() {
        let current_put_board = &current_put_board.board;
        let score = -nws_perfect(current_put_board, -beta, search);
        if score >= beta {
            search.t_table.add(board, score, SCORE_INF);
            return score;
        }
        this_node_alpha = this_node_alpha.max(score);
        best_score = best_score.max(score);
    }

    if best_score > alpha {
        search.t_table.add(board, best_score, best_score);
    } else {
        search.t_table.add(board, -SCORE_INF, best_score);
    }

    best_score
}

/// Principal Variation Search (PVS) を用いて、完全読みを行い、オセロの盤面のスコアを計算する。
///
///  PVS(Negascout)について :
///   https://ja.wikipedia.org/wiki/Negascout
///
/// # 引数
/// * `board` - 評価するオセロの盤面を表す `Board` オブジェクトの参照。
/// * `alpha` - 探索の下限値を示すアルファ値。
/// * `beta` - 探索の上限値を示すベータ値。
/// * `search` - 探索の状態を追跡する `Search` オブジェクトへのミュータブルな参照。
///
/// # 戻り値
/// * 探索結果として計算された盤面のスコアを表す整数値。
///   スコアは現在のプレイヤーから見た盤面の評価値を表す。
///
/// # 例
/// ```
/// let board = Board::new(); // オセロの初期盤面を生成
/// let mut search = Search::new();
/// let alpha = -SCORE_INF; // 初期アルファ値の設定
/// let beta = SCORE_INF; // 初期ベータ値の設定
/// let score = pvs_perfect(&board, alpha, beta, &mut search);
/// println!("Score: {}", score);
/// ```
///
/// # 注記
/// * 終盤の局面では、`pvs_perfect_simple` 関数に切り替わります。
/// * 置換表を使用して探索効率を向上させます。
pub fn pvs_perfect(board: &Board, mut alpha: i32,mut beta: i32, search: &mut Search) -> i32
{
    let n_empties = num_of_empties(board);
    if n_empties < SWITCH_EMPTIES_SIMPLE_PVS  {
        return pvs_perfect_simple(board, alpha, beta, search);
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
            return  solve_score(&board);
        }

        // passしたら、合法手がある -> 探索を続ける
        search.node_count += 1;
        return -pvs_perfect(&board, -beta, -alpha, search);
    }

    search.node_count += 1;

    // TranspositionTable Cut off
    if let Some(score) = t_table_cut_off(board, &mut alpha, &mut beta, &search.t_table) {
        return score;
    }

    // move ordering
    let put_boards = {
        if n_empties > SWITCH_EMPTIES_MOVE_ORDER {
            move_ordering_eval(board, legal_moves, MOVE_ORDERING_EVAL_LEVEL)
        } else {
            move_ordering_ffs(board, legal_moves)
        }
    };

    let mut put_boards_iter = put_boards.iter();
    
    let mut this_node_alpha = alpha;
    let mut best_score; //  =  - inf

    // first move
    let first_child_board = put_boards_iter.next().unwrap();
    best_score =  -pvs_perfect(&first_child_board.board, -beta, -this_node_alpha, search);
    if best_score >= beta { 
        search.t_table.add(board, best_score, SCORE_INF);
        return best_score;
    }
    this_node_alpha = this_node_alpha.max(best_score);

    // other move
    for current_put_board in put_boards_iter {
        let current_put_board = &current_put_board.board;
        let mut score = -nws_perfect(current_put_board, -this_node_alpha - 1, search);
        if score >= beta {
            search.t_table.add(board, score, SCORE_INF);
            return score;
        }
        if score > best_score {

            this_node_alpha = this_node_alpha.max(score);
            // 再探索
            score = -pvs_perfect(current_put_board, -beta, -this_node_alpha, search);
            if score >= beta { 
                search.t_table.add(board, score, SCORE_INF);
                return score;
             }
             best_score = score;
        }
        this_node_alpha = this_node_alpha.max(score);
    }

    if best_score > alpha { // alpha < best_score < beta
        search.t_table.add(board, best_score, best_score);
    } else { // best_score <= alpha
        search.t_table.add(board, -SCORE_INF, best_score);
    }

    best_score
}


