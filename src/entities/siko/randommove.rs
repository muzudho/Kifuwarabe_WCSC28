//!
//! ランダムムーブ
//!
extern crate rand;
use rand::Rng;

use super::super::teigi::conv::*;
use super::super::teigi::shogi_syugo::*;

///
/// ランダムに真偽を返す。
///
#[allow(dead_code)]
pub fn rnd_bool() -> bool {
    rand::thread_rng().gen_range(0, 2) == 0
}

///
/// (筋1～9,段1～9)の範囲で、ランダムに マス座標を返す
///
pub fn rnd_sq() -> Square {
    file_rank_to_sq(
        rand::thread_rng().gen_range(1, 10),
        rand::thread_rng().gen_range(1, 10),
    )
}

///
/// ランダムに 駒の種類を返す
///
pub fn rnd_pt() -> &'static PieceType {
    &PT_ARRAY[rand::thread_rng().gen_range(0, PT_ARRAY_LEN)]
}
