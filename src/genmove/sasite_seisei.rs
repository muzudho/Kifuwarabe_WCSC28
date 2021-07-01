//!
//! 現局面を使った指し手生成
//!

use super::super::entities::consoles::asserts::*;
use super::super::entities::jotai::uchu::*;
use super::super::entities::teigi::conv::*;
use super::super::entities::teigi::shogi_syugo::*;
use super::super::entities::tusin::usi::*;
use super::super::genmove::sasite_element::*;
use std::collections::HashSet;

///
/// 現局面の、任意の移動先升の、
/// - 盤上の駒の移動
/// - 打
/// の指し手を生成。
///
/// 王手回避漏れや、千日手などのチェックは行っていない
///
pub fn insert_potential_move(uchu: &Uchu, some_moves_hashset: &mut HashSet<u64>) {
    // +----------------+
    // | 盤上の駒の移動 |
    // +----------------+
    for rank_from in 1..10 {
        for file_from in 1..10 {
            let from = file_rank_to_sq(file_from, rank_from);
            let pc_from = uchu.ky.get_pc_by_sq(from);
            let phase = pc_to_ph(&pc_from);

            if match_ph(&phase, &uchu.get_teban(&Jiai::Ji)) {
                // 手番の駒

                let mut to_hashset: HashSet<Square> = HashSet::new();
                insert_dst_by_sq_pc(
                    from,
                    &pc_from,
                    false, // 成らず
                    &uchu,
                    &mut to_hashset,
                );

                // g_writeln("テスト ポテンシャルムーブ insert_dst_by_sq_pc(成らず).");
                // use consoles::visuals::dumps::*;
                // print_sq_hashset( &to_hashset );

                for to in &to_hashset {
                    some_moves_hashset.insert(
                        Sasite {
                            src: from,
                            dst: *to,
                            pro: false, // 成らず
                            drop: PieceType::Empty,
                        }
                        .to_hash(),
                    );
                }

                to_hashset.clear();
                insert_dst_by_sq_pc(
                    from,
                    &pc_from,
                    true, // 成り
                    &uchu,
                    &mut to_hashset,
                );
                for to in &to_hashset {
                    some_moves_hashset.insert(
                        Sasite {
                            src: from,
                            dst: *to,
                            pro: true, // 成り
                            drop: PieceType::Empty,
                        }
                        .to_hash(),
                    );
                }
            }
        }
    }

    // +----+
    // | 打 |
    // +----+
    for rank_to in 1..10 {
        for file_to in 1..10 {
            let to = file_rank_to_sq(file_to, rank_to);
            let to_pc = uchu.ky.get_pc_by_sq(to);
            match to_pc {
                Piece::Empty => {
                    // 駒が無いところに打つ

                    let mut drop_pt_hashset = HashSet::new();
                    for pt_hand in MGS_ARRAY.iter() {
                        let pc_hand = ph_pt_to_pc(&uchu.get_teban(&Jiai::Ji), pt_hand);
                        if 0 < uchu.ky.get_mg(&pc_hand) {
                            // 駒を持っていれば
                            insert_drop_pt_by_sq_pc(to, &pc_hand, &uchu, &mut drop_pt_hashset);
                        }
                    }
                    for num_pt_drop in drop_pt_hashset {
                        let pt = num_to_pt(num_pt_drop);
                        some_moves_hashset.insert(
                            Sasite {
                                src: SS_SRC_DA, // 駒大
                                dst: to,        // どの升へ行きたいか
                                pro: false,     // 打に成りは無し
                                drop: pt,       // 打った駒種類
                            }
                            .to_hash(),
                        );
                    }
                }
                _ => {}
            }
        } //suji
    } //dan
}

///
/// 1. 移動先升指定  to
/// 2. 移動先駒指定  to_pc
///
/// 盤上の駒の移動の最初の１つ。打を除く
///
pub fn insert_move_by_sq_pc_on_board(
    uchu: &Uchu,
    to: Square,
    to_pc: &Piece,
    some_moves_hashset: &mut HashSet<u64>,
) {
    assert_banjo_ms(to, "insert_move_by_sq_pc_on_board");

    // 手番の先後、駒種類
    let (phase, _pt_to) = pc_to_ph_pt(&to_pc);

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_ph(&uchu.ky.get_sn_by_ms(to), &phase) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Sasite::new();

    ss_hash_builder.dst = to;

    // 移動元の升
    let mut mv_from_hashset: HashSet<Square> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    insert_nopromote_from_by_sq_pc(to, &to_pc, &uchu, &mut mv_from_hashset);
    for from in &mv_from_hashset {
        assert_banjo_ms(*from, "insert_move_by_sq_pc_on_board(成らず)");

        ss_hash_builder.src = *from;
        // 成らず
        ss_hash_builder.pro = false;
        ss_hash_builder.drop = PieceType::Empty;
        some_moves_hashset.insert(ss_hash_builder.to_hash());
    }

    // +--------------+
    // | 盤上（成り） |
    // +--------------+
    mv_from_hashset.clear();
    insert_beforepromote_from_by_sq_pc(to, &to_pc, &uchu, &mut mv_from_hashset);
    for from in &mv_from_hashset {
        assert_banjo_ms(*from, "insert_move_by_sq_pc_on_board(成り)");

        ss_hash_builder.src = *from;
        // 成り
        ss_hash_builder.pro = true;
        ss_hash_builder.drop = PieceType::Empty;
        some_moves_hashset.insert(ss_hash_builder.to_hash());
    }
}
///
/// 打
///
/// 1. 移動先升指定  to
/// 2. 移動先駒指定  to_pc
///
pub fn insert_move_by_sq_pc_on_drop(
    uchu: &Uchu,
    to: Square,
    to_pc: &Piece,
    some_moves_hashset: &mut HashSet<u64>,
) {
    assert_banjo_ms(to, "insert_move_by_sq_pc_on_drop");

    // 手番の先後、駒種類
    let (phase, _pt_to) = pc_to_ph_pt(&to_pc);

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_ph(&uchu.ky.get_sn_by_ms(to), &phase) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Sasite::new();

    ss_hash_builder.dst = to;

    // 移動元の升
    //let mut mv_from_hashset : HashSet<Square> = HashSet::new();

    // +----+
    // | 打 |
    // +----+

    let mut drop_pt_hashset: HashSet<usize> = HashSet::new();
    insert_drop_pt_by_sq_pc(to, &to_pc, &uchu, &mut drop_pt_hashset);
    // 打
    for num_pt_drop in drop_pt_hashset.iter() {
        let pt_drop = num_to_pt(*num_pt_drop);

        let hash_ss = Sasite {
            src: SS_SRC_DA,
            dst: to,
            pro: false,
            drop: pt_drop,
        }
        .to_hash();
        some_moves_hashset.insert(hash_ss);
    }
}
