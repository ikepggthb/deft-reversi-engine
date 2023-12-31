use crate::board::*;
use crate::ai::*;
use crate::t_table::TranspositionTable;

const SCORE_INF: i32 = 100000i32;


#[inline(always)]
pub fn num_of_empties(board: &Board) -> i32
{
    (board.bit_board[0] | board.bit_board[1]).count_zeros() as i32
}

/// 
pub struct PutBoard {
    eval: i32,
    pub board: Board,
    pub put_place: u64
}

/// 評価関数に基づいて、手の順序を決定するための関数。
///
/// この関数は、オセロの盤面上で着手可能な各手に対して、評価値の基づく探索を実施し、
/// それぞれの手の評価値に基づいて手を並び替える。
///
/// # 引数
/// * `board` - 現在のオセロの盤面を表す `Board` オブジェクトの参照。
/// * `legal_moves` - 現在の手番のプレイヤーが打てる合法手を表すビットボード。
/// * `lv` - 探索の深さを表す整数値。
///
/// # 戻り値
/// * `PutBoard` オブジェクトのベクター。
///   * 各手の評価値(`i32`), 
///   * 着手された盤面(`Board`), 
///   * ビットボードで表された着手した箇所(`u64`)
/// 
/// # 注記
/// * `move_ordering_ffs`との違い
///   * `move_ordering_eval`は、評価値の高い順に並び替える。
///   * `move_ordering_ffs`は、相手の合法手が少ない順に並び替える。
pub fn move_ordering_eval(board: &Board, mut legal_moves: u64, lv: i32) -> Vec<PutBoard>
{
    let mut put_boards: Vec<PutBoard> = Vec::with_capacity(legal_moves.count_ones() as usize);

    while legal_moves != 0 {
        let put_place = (!legal_moves + 1) & legal_moves;
        legal_moves &= legal_moves - 1;
        let mut put_board = board.clone();
        put_board.put_piece_fast(put_place);
        let e =  -nega_scout_mid_game(&mut put_board.clone(), -SCORE_INF, SCORE_INF, lv);
        put_boards.push(PutBoard{eval: e, board: put_board, put_place: put_place});
    }

    if put_boards.len() > 2 {
        put_boards.sort_unstable_by(|a, b| b.eval.partial_cmp(&a.eval).unwrap());
    }

    put_boards
}




/// 速さ優先探索(Fast First Search)のための、move ordering
/// 合法手を評価し、手の順序を決定するための関数。
///
/// オセロの盤面上で着手可能な各手に対して、
/// 合法手が少ない順に手を並び替える。
///
/// # 引数
/// * `board` - 現在のオセロの盤面を表す `Board` オブジェクトの参照。
/// * `legal_moves` - 現在の手番のプレイヤーが打てる合法手を表すビットボード。
///
/// # 戻り値
/// * `PutBoard` オブジェクトのベクター。
///   * 各手の評価値(`i32`), 
///   * 着手された盤面(`Board`), 
///   * ビットボードで表された着手した箇所(`u64`)
/// 
/// # 注記
/// * `move_ordering_eval`との違い
///   * `move_ordering_eval`は、評価値の高い順に並び替える。
///   * `move_ordering_ffs`は、相手の合法手が少ない順に並び替える。
#[inline(always)]
pub fn move_ordering_ffs(board: &Board, mut legal_moves: u64) -> Vec<PutBoard>
{
    let mut put_boards: Vec<PutBoard> = Vec::with_capacity(legal_moves.count_ones() as usize);

    while legal_moves != 0 {
        let put_place = (!legal_moves + 1) & legal_moves;
        legal_moves &= legal_moves - 1;
        let mut put_board = board.clone();
        put_board.put_piece_fast(put_place);
        put_boards.push(PutBoard{eval: put_board.put_able().count_ones() as i32, board: put_board, put_place: put_place})
    }

    if put_boards.len() > 3{
        put_boards.sort_unstable_by(|a, b| a.eval.partial_cmp(&b.eval).unwrap());
    }
    put_boards
}


#[inline(always)]
pub fn t_table_cut_off(
    board   :       & Board,
    alpha   :    &mut i32,
    beta    :    &mut i32,
    t_table :       & TranspositionTable ) -> Option<i32>
{
    if let Some(t) = t_table.get(board) {
        if t.max <= *alpha {return Some(t.max);}
        else if t.min >= *beta {return Some(t.min);}
        else if t.max == t.min {return Some(t.max);}
        if t.min > *alpha {*alpha = t.min};
        if t.max < *beta {*beta = t.max};
    }
    None
}

pub struct Search {
    pub node_count: u64,
    pub leaf_node_count: u64,
    pub t_table: TranspositionTable,
    pub origin_board: Board,
}

impl Search {
    pub fn new(board :&Board, t_table: TranspositionTable) -> Search{
        Search{
            node_count: 0,
            leaf_node_count: 0,
            t_table: t_table,
            origin_board: board.clone()
        }
    }
}