#![allow(dead_code)]
//!
//! 頭金仮説
//!

use super::super::jotai::uchu::*;
use super::super::teigi::banjometries::*;
use super::super::teigi::conv::*;
use super::super::teigi::shogi_syugo::*;
use super::super::teigi::shogi_syugo_seki::*;

/// 後手視点で、相手らいおんの南側１升に、頭が丸い自駒がない？
pub fn is_s(uchu: &Uchu) -> bool {
    // 相手玉の位置
    let ms_r = uchu.get_ms_r(&Jiai::Ai);

    let p_r = ms_to_p(ms_r);
    let p_south_r = p_r.to_south();
    if !p_in_ban(&p_south_r) {
        return true;
    }

    let ms_south_r = p_to_ms(&p_south_r);
    let pc = uchu.ky.get_pc_by_sq(ms_south_r);
    let jiai_km = uchu.get_jiai_by_km(&pc);
    if !match_jiai(&jiai_km, &Jiai::Ji) {
        return true;
    }

    g_writeln(&format!(
        "info string south of My raion {} = {}. jiai_km={}.",
        ms_r, pc, jiai_km
    ));

    let pt = pc_to_pt(&pc);

    use super::super::teigi::shogi_syugo::PieceType::*;
    match pt {
        B | L => {
            return false;
        }
        _ => {}
    }

    return true;
}

/// 頭金か？
///
/// 三駒定形｛　ms_ai_r、 kms_set_ai_c_r、 ms_atama、 ms_kin、ms_behind、Ｔ　｝において、
/// 　｛　升　×　相手玉　　　　　｝∈ ms_aiR　かつ　、
/// 　｛　升　×　玉以外の相手駒　｝∈ kms_set_ai_c_r　かつ　、
/// 　｛　升　×　駒無し含む玉以外の相手駒　｝∈ ms_atama、
/// 　升　×　｛金、と、杏、圭、全、馬、竜｝　∈ ms_kin、
/// 　　　　　　　　　　　　　　升　×　自駒　∈ ms_behind、
/// 　Ｔは　１手　固定とし、
///
/// ms_ai_r の下段左中右＝移動不可升
/// かつ、
/// ms_ai_r（北０）ms_atama
/// かつ、
/// ms_behind （利き→）ms_atama、
/// かつ、
/// （
/// 　　（　ms_atama は空升　かつ　｛金｝∈自持駒　）
/// 　　または
/// 　　（　ms_kin （利き→）ms_atama　）
/// ）
/// かつ、
/// kms_set_ai_c_r（利き→） ms_atama でない、
///
/// のとき、
///
/// 一手詰め逃さない公理から、
///
/// ms_ai_r の９近傍非空升　⊂　ms_atama に指す（ ms_ai_r、kms_set_ai_c_r，ms_atama、 ms_kin， ms_behind，Ｔ）
///
/// が成り立つ
/// FIXME Aが動いたときの、逆王手が未考慮
pub fn is_atamakin(
    _mskms_l: &MsKms,
    _mskms_s: &MsKms,
    _mskms_a: &MsKms,
    _mskms_b: &MsKms,
    uchu: &Uchu,
) -> bool {
    // 相手らいおんのマス
    let ms_ai_r = uchu.get_ms_r(&Jiai::Ai);

    // らいおん以外の相手の駒種類
    let mut kms_set_ai_c_r = KmsSyugo::new_all();
    kms_set_ai_c_r.remove(&PieceType::K);

    // kの下段左中右＝移動不可升　※現局面２手先の動き？
    // A が移動することで、利きは変わるか？　玉の下３つは変わらない
    // 単に下３つに移動できるか調べられたらいい。８１升別　利きを作るか？
    // 駒、相手の利き
    let p_k = ms_to_p(ms_ai_r);
    if banjo_metrics::is_ji_km_by_ms(p_to_ms(&p_k.to_south_west()), &uchu) {}

    if banjo_metrics::is_ai_kiki_by_ms(p_to_ms(&p_k.to_south_west()), &uchu) {}

    // ms_ai_r （北０） ms_atama
    // if ms_north_of_ms( ms_ai_r, 0, ms_atama ) { }

    // ms_behind （利き→）ms_atama、
    // mskms_attack_to_ms( b, ms_atama ) { }

    // （　ms_atama は空升　かつ　｛金｝∈自持駒　）
    // ( ms_is_empty( ms_atama ) && has_mg( I ) )
    // または
    // ||
    // （　ms_kin （利き→）ms_atama　）
    // mskms_attack_to_ms( ms_kin, ms_atama ) { }

    false
}
