#![allow(dead_code)]
/**
 * ランダム移動カード
 */

use consoles::asserts::*;
use teigi::conv::*;
use syazo::sasite_seisei::*;
use syazo::sasite_sentaku::*;
use std::collections::HashSet;
use teigi::shogi_syugo::*;
use tusin::usi::*;
use jotai::uchu::*;
use siko::randommove;
use siko::results::jisatusyu_result::*;

/**
 * ランダム移動
 *
 * km_dst : 移動した先の駒
 */
pub fn get_ido_ss_by_km_random(uchu :&Uchu, km_dst:&Koma)->Sasite{
    
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    for _i_retry in 0..1000000 {
        // 移動したい先の升
        let ms_dst = randommove::rnd_ms();
        assert_banjo_ms(ms_dst, "get_ido_ss_by_km_random");

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo ( &uchu, ms_dst, &km_dst, &mut ss_hashset );
        insert_ss_by_ms_km_on_da    ( &uchu, ms_dst, &km_dst, &mut ss_hashset );
        let ss = choice_1ss_by_hashset( &ss_hashset );

        if ss.exists(){ return ss;}
    }
    // 投了
    Sasite::new()
}

/**
 * 指し手１つをランダム選出
 */
pub fn get_ss_by_random(uchu :&Uchu)->Sasite{
    
    let mut ss_hashset = HashSet::new();

    // 数回リトライ
    'random: for _i_retry in 0..1000000 {
        // 移動したい先の升
        let ms_dst = randommove::rnd_ms();
        assert_banjo_ms(ms_dst, "Ｇet_ss_by_random");

        // 手番の、移動した先の駒
        let km_dst = sn_kms_to_km( &uchu.get_teban(&Jiai::Ji), randommove::rnd_kms() );

        ss_hashset.clear();
        insert_ss_by_ms_km_on_banjo ( &uchu, ms_dst, &km_dst, &mut ss_hashset );
        insert_ss_by_ms_km_on_da    ( &uchu, ms_dst, &km_dst, &mut ss_hashset );
        let ss = choice_1ss_by_hashset( &ss_hashset );

        // 移動後は、玉が利きに飛び込まないか？
        if is_jisatusyu( &uchu, &ss ) {
            continue 'random;
        }

        if ss.exists(){ return ss;}
    }
    // 投了
    Sasite::new()
}
