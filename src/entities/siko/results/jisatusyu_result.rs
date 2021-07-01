#![allow(dead_code)]
//!
//! 結果：自殺手。移動先が敵の利き
//!

use super::super::super::jotai::uchu::*;
use super::super::super::teigi::conv::*;
use super::super::super::tusin::usi::*;

/// 動かした先が、敵の利きに飛び込んでいれば、自殺手
/// TODO 利きを再計算したい
pub fn is_jisatusyu(uchu: &Uchu, ss: &Sasite) -> bool {
    // 移動元升、動かした駒の先後、駒種類、
    let pc_from = uchu.ky.get_pc_by_sq(ss.src);
    let (sn_teban, _pt) = pc_to_ph_pt(&pc_from);
    // 相手番の先後
    let sn_aite = hanten_sn(&sn_teban);

    // 升の利き数だが、指した後で再計算が要るはず
    let kikisu = uchu.kiki_su_by_sn[sn_to_num(&sn_aite)].get_su_by_ms(ss.dst);
    let result = 0 < kikisu;
    // g_writeln(&format!(
    //     "info is_jisatusyu={} pc_from={} sn_teban={} pt={} sn_aite={} ss.dst={} kikisu={}"
    //     ,result ,pc_from ,sn_teban ,pt ,sn_aite ,ss.dst ,kikisu
    // ));

    result
}
