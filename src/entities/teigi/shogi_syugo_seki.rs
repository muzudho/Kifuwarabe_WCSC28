#![allow(dead_code)]
//!
//! 積☆（＾～＾）　要するに組み合わせ
//!

use super::super::teigi::shogi_syugo::*;

/// 升 × 駒
pub struct MsKm {
    sq: Square,
    km: Koma,
}

/// 升 × 駒種類
pub struct MsKms {
    sq: Square,
    kms: KmSyurui,
}
