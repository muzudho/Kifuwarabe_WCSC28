//!
//! 盤上いろいろ☆（＾～＾）
//!

use super::super::super::jotai::uchu::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::shogi_syugo::*;

pub fn is_ji_km_by_ms(sq: Square, uchu: &Uchu) -> bool {
    let pc = uchu.ky.get_pc_by_sq(sq);
    let (phase, _kms) = pc_to_ph_pt(&pc);
    match_sn(&phase, &uchu.get_teban(&Jiai::Ji))
}

// TODO
pub fn is_ai_kiki_by_ms(_ms: Square, _uchu: &Uchu) -> bool {
    false
}
