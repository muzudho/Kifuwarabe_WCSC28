#![allow(dead_code)]
//!
//! 積☆（＾～＾）　要するに組み合わせ
//!

use super::super::teigi::shogi_syugo::*;
use crate::take1base::Piece;

/// 升 × 駒
pub struct SqPc {
    sq: Square,
    pc: Piece,
}

/// 升 × 駒種類
pub struct SqPt {
    sq: Square,
    pt: PieceType,
}
