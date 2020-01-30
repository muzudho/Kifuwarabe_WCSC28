#![allow(dead_code)]
//!
//! ランダム移動カード
//!

use super::super::super::consoles::asserts::*;
use super::super::super::jotai::uchu::*;
use super::super::super::siko::randommove;
use super::super::super::siko::results::jisatusyu_result::*;
use super::super::super::syazo::sasite_seisei::*;
use super::super::super::syazo::sasite_sentaku::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::shogi_syugo::*;
use super::super::super::tusin::usi::*;
use std::collections::HashSet;

/**
 * ランダム移動
 *
 * km_dst : 移動した先の駒
 */
pub fn get_ido_ss_by_km_random(uchu: &Uchu, km_dst: &Koma) -> Sasite {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1000000 {
        // 移動したい先の升
        let ms_dst = randommove::rnd_ms();
        assert_banjo_ms(ms_dst, "get_ido_ss_by_km_random");

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo(&uchu, ms_dst, &km_dst, &mut ss_hashset);
        insert_ss_by_ms_km_on_da(&uchu, ms_dst, &km_dst, &mut ss_hashset);
        let ss = choice_1ss_by_hashset(&ss_hashset);

        if ss.exists() {
            return ss;
        }
    }
    // 投了
    Sasite::new()
}

/**
 * 指し手１つをランダム選出
 */
pub fn get_ss_by_random(uchu: &Uchu) -> Sasite {
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1000000 {
        // 移動したい先の升
        let ms_dst = randommove::rnd_ms();
        assert_banjo_ms(ms_dst, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let km_dst = sn_kms_to_km(&uchu.get_teban(&Jiai::Ji), randommove::rnd_kms());

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo(&uchu, ms_dst, &km_dst, &mut ss_hashset);
        insert_ss_by_ms_km_on_da(&uchu, ms_dst, &km_dst, &mut ss_hashset);
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
