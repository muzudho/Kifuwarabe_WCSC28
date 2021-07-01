//!
//! 盤上いろいろ☆（＾～＾）
//!

use super::super::super::jotai::uchu::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::shogi_syugo::*;

pub fn is_ji_km_by_ms(ms: Square, uchu: &Uchu) -> bool {
    let km = uchu.ky.get_km_by_ms(ms);
    let (sn, _kms) = km_to_sn_kms(&km);
    match_sn(&sn, &uchu.get_teban(&Jiai::Ji))
}

// TODO
pub fn is_ai_kiki_by_ms(_ms: Square, _uchu: &Uchu) -> bool {
    false
}
