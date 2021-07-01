#![allow(dead_code)]
//!
//! ランダム移動カード
//!

use super::super::super::super::genmove::sasite_seisei::*;
use super::super::super::super::genmove::sasite_sentaku::*;
use super::super::super::consoles::asserts::*;
use super::super::super::jotai::uchu::*;
use super::super::super::siko::randommove;
use super::super::super::siko::results::jisatusyu_result::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::shogi_syugo::*;
use super::super::super::tusin::usi::*;
use std::collections::HashSet;

///
/// ランダム移動
///
/// to_pc : 移動した先の駒
///
pub fn get_ido_move_by_pc_random(uchu: &Uchu, to_pc: &Piece) -> Sasite {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1000000 {
        // 移動したい先の升
        let to = randommove::rnd_sq();
        assert_banjo_ms(to, "get_ido_move_by_pc_random");

        ss_hashset.clear();
        insert_move_by_sq_pc_on_board(&uchu, to, &to_pc, &mut ss_hashset);
        insert_move_by_sq_pc_on_drop(&uchu, to, &to_pc, &mut ss_hashset);
        let ss = choice_1ss_by_hashset(&ss_hashset);

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    Sasite::new()
}

///
/// 指し手１つをランダム選出
///
pub fn get_ss_by_random(uchu: &Uchu) -> Sasite {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1000000 {
        // 移動したい先の升
        let to = randommove::rnd_sq();
        assert_banjo_ms(to, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let to_pc = ph_pt_to_pc(&uchu.get_teban(&Person::Friend), randommove::rnd_pt());

        ss_hashset.clear();
        insert_move_by_sq_pc_on_board(&uchu, to, &to_pc, &mut ss_hashset);
        insert_move_by_sq_pc_on_drop(&uchu, to, &to_pc, &mut ss_hashset);
        let ss = choice_1ss_by_hashset(&ss_hashset);

        // 移動後は、玉が利きに飛び込まないか？
        if is_jisatusyu(&uchu, &ss) {
            continue 'random;
        }

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    Sasite::new()
}
