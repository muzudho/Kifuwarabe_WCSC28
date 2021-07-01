//!
//! 利き数
//!

use super::super::super::super::genmove::sasite_element::*;
use super::super::super::consoles::asserts::*;
use super::super::super::jotai::uchu::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::shogi_syugo::*;
use std::collections::HashSet;

/// 盤上の利き升調べ
///
/// 用途：自殺手防止他
pub fn read_kikisu(uchu: &mut Uchu) {
    // ゼロ・リセット
    for pc in PC_ARRAY.iter() {
        &uchu.kiki_su_by_pc[pc_to_num(pc)].clear();
    }

    for phase in SN_ARRAY.iter() {
        &uchu.kiki_su_by_sn[sn_to_num(phase)].clear();
    }

    // カウント
    for to_pc in PC_ARRAY.iter() {
        for x in SUJI_1..SUJI_10 {
            // 9..0 みたいに降順に書いても動かない？
            for y in DAN_1..DAN_10 {
                let to = file_rank_to_sq(x, y);
                assert_banjo_ms(to, "think 利き調べ");

                // 移動元の升
                let mut mv_src_hashset: HashSet<Square> = HashSet::new();
                insert_nopromote_from_by_sq_pc(to, &to_pc, &uchu, &mut mv_src_hashset);
                insert_beforepromote_from_by_sq_pc(to, &to_pc, &uchu, &mut mv_src_hashset);
                // 打は考えない。盤上の利き数なので
                let kikisu = mv_src_hashset.len();
                let phase = pc_to_ph(&to_pc);

                // 駒別
                uchu.kiki_su_by_pc[pc_to_num(&to_pc)].add_su_by_ms(to, kikisu as i8);

                // 先後別
                uchu.kiki_su_by_sn[sn_to_num(&phase)].add_su_by_ms(to, kikisu as i8);
            }
        }
    }
}
