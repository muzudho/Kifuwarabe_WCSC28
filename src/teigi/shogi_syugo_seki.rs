#![allow(dead_code)]
/**
 * 積☆（＾～＾）　要するに組み合わせ
 */

use teigi::shogi_syugo::*;

/************
 * 升 × 駒 *
 ************/

pub struct MsKm {
    ms : umasu,
    km : Koma,
}

/****************
 * 升 × 駒種類 *
 ****************/

pub struct MsKms {
    ms : umasu,
    kms: KmSyurui,
}
