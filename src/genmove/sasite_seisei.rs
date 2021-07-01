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
    for dan_src in 1..10 {
        for suji_src in 1..10 {
            let ms_src = suji_dan_to_ms(suji_src, dan_src);
            let km_src = uchu.ky.get_km_by_ms(ms_src);
            let sn = km_to_sn(&km_src);

            if match_sn(&sn, &uchu.get_teban(&Jiai::Ji)) {
                // 手番の駒

                let mut dst_hashset: HashSet<Square> = HashSet::new();
                insert_dst_by_ms_km(
                    ms_src,
                    &km_src,
                    false, // 成らず
                    &uchu,
                    &mut dst_hashset,
                );

                // g_writeln("テスト ポテンシャルムーブ insert_dst_by_ms_km(成らず).");
                // use consoles::visuals::dumps::*;
                // hyoji_ms_hashset( &dst_hashset );

                for ms_dst in &dst_hashset {
                    some_moves_hashset.insert(
                        Sasite {
                            src: ms_src,
                            dst: *ms_dst,
                            pro: false, // 成らず
                            drop: KmSyurui::Kara,
                        }
                        .to_hash(),
                    );
                }

                dst_hashset.clear();
                insert_dst_by_ms_km(
                    ms_src,
                    &km_src,
                    true, // 成り
                    &uchu,
                    &mut dst_hashset,
                );
                for ms_dst in &dst_hashset {
                    some_moves_hashset.insert(
                        Sasite {
                            src: ms_src,
                            dst: *ms_dst,
                            pro: true, // 成り
                            drop: KmSyurui::Kara,
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
    for dan_dst in 1..10 {
        for suji_dst in 1..10 {
            let ms_dst = suji_dan_to_ms(suji_dst, dan_dst);
            let km_dst = uchu.ky.get_km_by_ms(ms_dst);
            match km_dst {
                Piece::Kara => {
                    // 駒が無いところに打つ

                    let mut da_kms_hashset = HashSet::new();
                    for kms_motigoma in MGS_ARRAY.iter() {
                        let km_motigoma = sn_kms_to_km(&uchu.get_teban(&Jiai::Ji), kms_motigoma);
                        if 0 < uchu.ky.get_mg(&km_motigoma) {
                            // 駒を持っていれば
                            insert_da_kms_by_ms_km(
                                ms_dst,
                                &km_motigoma,
                                &uchu,
                                &mut da_kms_hashset,
                            );
                        }
                    }
                    for num_kms_da in da_kms_hashset {
                        let kms = num_to_kms(num_kms_da);
                        some_moves_hashset.insert(
                            Sasite {
                                src: SS_SRC_DA, // 駒大
                                dst: ms_dst,    // どの升へ行きたいか
                                pro: false,     // 打に成りは無し
                                drop: kms,      // 打った駒種類
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
/// 1. 移動先升指定  ms_dst
/// 2. 移動先駒指定  km_dst
///
/// 盤上の駒の移動の最初の１つ。打を除く
///
pub fn insert_ss_by_ms_km_on_banjo(
    uchu: &Uchu,
    ms_dst: Square,
    km_dst: &Piece,
    some_moves_hashset: &mut HashSet<u64>,
) {
    assert_banjo_ms(ms_dst, "Ｉnsert_ss_by_ms_km_on_banjo");

    // 手番の先後、駒種類
    let (sn, _kms_dst) = km_to_sn_kms(&km_dst);

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn(&uchu.ky.get_sn_by_ms(ms_dst), &sn) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Sasite::new();

    ss_hash_builder.dst = ms_dst;

    // 移動元の升
    let mut mv_src_hashset: HashSet<Square> = HashSet::new();

    // +----------------+
    // | 盤上（成らず） |
    // +----------------+
    insert_narazu_src_by_ms_km(ms_dst, &km_dst, &uchu, &mut mv_src_hashset);
    for ms_src in &mv_src_hashset {
        assert_banjo_ms(*ms_src, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成らず)");

        ss_hash_builder.src = *ms_src;
        // 成らず
        ss_hash_builder.pro = false;
        ss_hash_builder.drop = KmSyurui::Kara;
        some_moves_hashset.insert(ss_hash_builder.to_hash());
    }

    // +--------------+
    // | 盤上（成り） |
    // +--------------+
    mv_src_hashset.clear();
    insert_narumae_src_by_ms_km(ms_dst, &km_dst, &uchu, &mut mv_src_hashset);
    for ms_src in &mv_src_hashset {
        assert_banjo_ms(*ms_src, "Ｉnsert_ss_by_ms_km_on_banjo ms_src(成り)");

        ss_hash_builder.src = *ms_src;
        // 成り
        ss_hash_builder.pro = true;
        ss_hash_builder.drop = KmSyurui::Kara;
        some_moves_hashset.insert(ss_hash_builder.to_hash());
    }
}
///
/// 打
///
/// 1. 移動先升指定  ms_dst
/// 2. 移動先駒指定  km_dst
///
pub fn insert_ss_by_ms_km_on_da(
    uchu: &Uchu,
    ms_dst: Square,
    km_dst: &Piece,
    some_moves_hashset: &mut HashSet<u64>,
) {
    assert_banjo_ms(ms_dst, "Ｉnsert_ss_by_ms_km_on_da");

    // 手番の先後、駒種類
    let (sn, _kms_dst) = km_to_sn_kms(&km_dst);

    // 移動先に自駒があれば、指し手は何もない。終わり。
    if match_sn(&uchu.ky.get_sn_by_ms(ms_dst), &sn) {
        return;
    }

    // ハッシュを作るのに使う
    let mut ss_hash_builder = Sasite::new();

    ss_hash_builder.dst = ms_dst;

    // 移動元の升
    //let mut mv_src_hashset : HashSet<Square> = HashSet::new();

    // +----+
    // | 打 |
    // +----+

    let mut da_kms_hashset: HashSet<usize> = HashSet::new();
    insert_da_kms_by_ms_km(ms_dst, &km_dst, &uchu, &mut da_kms_hashset);
    // 打
    for num_kms_da in da_kms_hashset.iter() {
        let kms_da = num_to_kms(*num_kms_da);

        let hash_ss = Sasite {
            src: SS_SRC_DA,
            dst: ms_dst,
            pro: false,
            drop: kms_da,
        }
        .to_hash();
        some_moves_hashset.insert(hash_ss);
    }
}
