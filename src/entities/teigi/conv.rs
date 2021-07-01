#![allow(dead_code)]
//!
//! 変換
//!
//! * 論理値
//!

use super::super::consoles::asserts::*;
use super::super::teigi::geometries::geo_teigi::*;
use super::super::teigi::shogi_syugo::*;

///
/// false => 0
/// true => 1
///
/// bool は i32 だが、_to_num 系は usize を返すように合わせるぜ☆（*＾～＾*）
///
#[allow(dead_code)]
pub fn bool_to_num(b: bool) -> usize {
    b as usize
}
///
/// 0 なら偽、それ以外は真☆（＾～＾）
///
#[allow(dead_code)]
pub fn num_to_bool(n: usize) -> bool {
    match n {
        0 => false,
        _ => true,
    }
}
///
/// ハッシュ値を作る
///
#[allow(dead_code)]
pub fn push_bool_to_hash(hash: u64, b: bool) -> u64 {
    // bool は i32 だが、hash は u64 なので u64 に合わせるぜ☆（*＾～＾*）
    (hash << 7) + b as u64
}
///
/// ハッシュ値から作る
///
#[allow(dead_code)]
pub fn pop_bool_from_hash(hash: u64) -> (u64, bool) {
    let b_num = num_to_bool((hash & 0b1) as usize);
    (hash >> 7, b_num)
}

///
/// 8方向
///
#[allow(dead_code)]
pub fn dir8_to_num(dir: &Dir8) -> usize {
    use super::super::teigi::shogi_syugo::Dir8::*;
    match *dir {
        E => 0,
        NE => 1,
        N => 2,
        NW => 3,
        W => 4,
        SW => 5,
        S => 6,
        SE => 7,
        Owari => 8,
    }
}
#[allow(dead_code)]
pub fn num_to_dir8(n: usize) -> Dir8 {
    use super::super::teigi::shogi_syugo::Dir8::*;
    match n {
        0 => E,
        1 => NE,
        2 => N,
        3 => NW,
        4 => W,
        5 => SW,
        6 => S,
        7 => SE,
        _ => Owari,
    }
}
///
/// ハッシュ値を作る
///
#[allow(dead_code)]
pub fn push_dir8_to_hash(hash: u64, dir: &Dir8) -> u64 {
    // エラー値含めて 9bit あるので 2^5
    (hash << 5) + dir8_to_num(dir) as u64
}
///
/// ハッシュ値から作る
///
#[allow(dead_code)]
pub fn pop_dir8_from_hash(hash: u64) -> (u64, Dir8) {
    // エラー値含めて 9bit あるので 2^5
    let dir = num_to_dir8((hash & 0b11111) as usize);
    (hash >> 5, dir)
}

///
/// 先後
///
pub fn sn_to_num(phase: &Phase) -> usize {
    use super::super::teigi::shogi_syugo::Phase::*;
    match *phase {
        First => 0,
        Second => 1,
        Owari => 2,
    }
}
pub fn hanten_sn(phase: &Phase) -> Phase {
    use super::super::teigi::shogi_syugo::Phase::*;
    match *phase {
        First => Second,
        Second => First,
        Owari => Owari,
    }
}

///
/// 自分相手
///
pub fn jiai_to_num(jiai: &Jiai) -> usize {
    use super::super::teigi::shogi_syugo::Jiai::*;
    match *jiai {
        Ji => 0,
        Ai => 1,
        Owari => 2,
    }
}
pub fn hanten_jiai(jiai: &Jiai) -> Jiai {
    use super::super::teigi::shogi_syugo::Jiai::*;
    match *jiai {
        Ji => Ai,
        Ai => Ji,
        Owari => Owari,
    }
}

///
/// Square は 将棋盤座標
///
/// 91 81 71 ...
/// 92 82 72
/// 93 83 73
/// ...
///
pub fn sq_to_file_rank(sq: Square) -> (i8, i8) {
    assert_banjo_ms(sq, "(203)Ｍs_to_suji_dan");
    ((sq / 10) as i8, (sq % 10) as i8)
}
pub fn ms_to_p(sq: Square) -> Point {
    assert_banjo_ms(sq, "(203b)ms_to_p");
    Point {
        x: (sq / 10) as i8,
        y: (sq % 10) as i8,
    }
}
pub fn suji_dan_to_ms(suji: i8, dan: i8) -> Square {
    debug_assert!(
        (SUJI_0 < suji && suji < SUJI_10) && (DAN_0 < dan && dan < DAN_10),
        "(204)suji_dan_to_ms suji={},dan={}",
        suji,
        dan
    );

    (suji * 10 + dan) as Square
}
pub fn p_in_ban(p: &Point) -> bool {
    (SUJI_0 < p.x && p.x < SUJI_10) && (DAN_0 < p.y && p.y < DAN_10)
}
pub fn p_to_ms(p: &Point) -> Square {
    debug_assert!(p_in_ban(&p), "(204b)p_to_ms x={},y={}", p.x, p.y);

    (p.x * 10 + p.y) as Square
}
///
/// ハッシュ値を作る
///
pub fn push_ms_to_hash(hash: u64, sq: Square) -> u64 {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    (hash << 7) + sq as u64
}
///
/// ハッシュ値から作る
///
pub fn pop_ms_from_hash(hash: u64) -> (u64, Square) {
    // 0筋とか 0段とか 使ってないが、そのまま足す。
    // 0～100の101升と、ちょいなんで、128(=2^7) あれば十分
    let ms_num = (hash & 0b1111111) as Square;
    (hash >> 7, ms_num)
}

///
/// 指し手のために、段をアルファベットにすることを想定
///
pub fn num_to_lower_case(num: i8) -> &'static str {
    match num {
        1 => "a",
        2 => "b",
        3 => "c",
        4 => "d",
        5 => "e",
        6 => "f",
        7 => "g",
        8 => "h",
        9 => "i",
        _ => "?", // 返却型が &'static str なので、エラー値を動的に作れない
    }
}
///
/// 先手であれば、後手のように番号を振った座標に変換
///
pub fn kaiten180_ms_by_ms_sn(sq: Square, phase: &Phase) -> Square {
    use super::super::teigi::shogi_syugo::Phase::*;
    match *phase {
        First => BAN_MAX - sq + BAN_MIN,
        _ => sq,
    }
}

///
/// 先後付き駒の数値化
///
pub fn pc_to_num(pc: &Piece) -> usize {
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        R0 => 0,
        K0 => 1,
        Z0 => 2,
        I0 => 3,
        N0 => 4,
        U0 => 5,
        S0 => 6,
        H0 => 7,
        PK0 => 8,
        PZ0 => 9,
        PN0 => 10,
        PU0 => 11,
        PS0 => 12,
        PH0 => 13,
        R1 => 14,
        K1 => 15,
        Z1 => 16,
        I1 => 17,
        N1 => 18,
        U1 => 19,
        S1 => 20,
        H1 => 21,
        PK1 => 22,
        PZ1 => 23,
        PN1 => 24,
        PU1 => 25,
        PS1 => 26,
        PH1 => 27,
        Kara => 28,
        Owari => 29,
    }
}
pub fn num_to_km(km_num: usize) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match km_num {
        0 => R0,
        1 => K0,
        2 => Z0,
        3 => I0,
        4 => N0,
        5 => U0,
        6 => S0,
        7 => H0,
        8 => PK0,
        9 => PZ0,
        10 => PN0,
        11 => PU0,
        12 => PS0,
        13 => PH0,
        14 => R1,
        15 => K1,
        16 => Z1,
        17 => I1,
        18 => N1,
        19 => U1,
        20 => S1,
        21 => H1,
        22 => PK1,
        23 => PZ1,
        24 => PN1,
        25 => PU1,
        26 => PS1,
        27 => PH1,
        28 => Kara,
        _ => Owari,
    }
}
///
/// ハッシュ値を作る
///
pub fn push_km_to_hash(hash: u64, pc: &Piece) -> u64 {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    (hash << 5) + pc_to_num(pc) as u64
}
///
/// ハッシュ値から作る
///
pub fn pop_km_from_hash(hash: u64) -> (u64, Piece) {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    let km_num = num_to_km((hash & 0b11111) as usize);
    (hash >> 5, km_num)
}
///
/// 駒→成駒　（成れない駒は、そのまま）
///
pub fn pc_to_pro_pc(pc: &Piece) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        R0 => R0,
        K0 => PK0,
        Z0 => PZ0,
        I0 => I0,
        N0 => PN0,
        U0 => PU0,
        S0 => PS0,
        H0 => PH0,
        PK0 => PK0,
        PZ0 => PZ0,
        PN0 => PN0,
        PU0 => PU0,
        PS0 => PS0,
        PH0 => PH0,
        R1 => R1,
        K1 => PK1,
        Z1 => PZ1,
        I1 => I1,
        N1 => PN1,
        U1 => PU1,
        S1 => PS1,
        H1 => PH1,
        PK1 => PK1,
        PZ1 => PZ1,
        PN1 => PN1,
        PU1 => PU1,
        PS1 => PS1,
        PH1 => PH1,
        Kara => Kara,
        Owari => Owari,
    }
}
///
/// 成駒→駒
///
pub fn pro_pc_to_pc(pc: &Piece) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        R0 => R0,
        K0 => K0,
        Z0 => Z0,
        I0 => I0,
        N0 => N0,
        U0 => U0,
        S0 => S0,
        H0 => H0,
        PK0 => K0,
        PZ0 => Z0,
        PN0 => N0,
        PU0 => U0,
        PS0 => S0,
        PH0 => H0,
        R1 => R1,
        K1 => K1,
        Z1 => Z1,
        I1 => I1,
        N1 => N1,
        U1 => U1,
        S1 => S1,
        H1 => H1,
        PK1 => K1,
        PZ1 => Z1,
        PN1 => N1,
        PU1 => U1,
        PS1 => S1,
        PH1 => H1,
        Kara => Kara,
        Owari => Owari,
    }
}
///
/// 駒→長い利きの有無
///
pub fn km_is_nagaikiki(pc: &Piece) -> bool {
    kms_is_nagaikiki(&pc_to_pt(pc))
}
///
/// 先後付き駒→駒種類
///
pub fn pc_to_ph_pt(pc: &Piece) -> (Phase, PieceType) {
    // use super::super::teigi::shogi_syugo::PieceType;
    use super::super::teigi::shogi_syugo::PieceType::*;
    // use super::super::teigi::shogi_syugo::Piece;
    use super::super::teigi::shogi_syugo::Phase::*;
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        R0 => (First, K),
        K0 => (First, R),
        Z0 => (First, B),
        I0 => (First, G),
        N0 => (First, S),
        U0 => (First, N),
        S0 => (First, L),
        H0 => (First, P),
        PK0 => (First, PR),
        PZ0 => (First, PB),
        PN0 => (First, PS),
        PU0 => (First, PN),
        PS0 => (First, PL),
        PH0 => (First, PP),
        R1 => (Second, K),
        K1 => (Second, R),
        Z1 => (Second, B),
        I1 => (Second, G),
        N1 => (Second, S),
        U1 => (Second, N),
        S1 => (Second, L),
        H1 => (Second, P),
        PK1 => (Second, PR),
        PZ1 => (Second, PB),
        PN1 => (Second, PS),
        PU1 => (Second, PN),
        PS1 => (Second, PL),
        PH1 => (Second, PP),
        Piece::Kara => (Phase::Owari, PieceType::Kara),
        Piece::Owari => (Phase::Owari, PieceType::Owari),
    }
}
///
/// 先後付き駒　を　先後　へ変換。
///
#[allow(dead_code)]
pub fn pc_to_ph(pc: &Piece) -> Phase {
    use super::super::teigi::shogi_syugo::Phase::*;
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        R0 => First,
        K0 => First,
        Z0 => First,
        I0 => First,
        N0 => First,
        U0 => First,
        S0 => First,
        H0 => First,
        PK0 => First,
        PZ0 => First,
        PN0 => First,
        PU0 => First,
        PS0 => First,
        PH0 => First,
        R1 => Second,
        K1 => Second,
        Z1 => Second,
        I1 => Second,
        N1 => Second,
        U1 => Second,
        S1 => Second,
        H1 => Second,
        PK1 => Second,
        PZ1 => Second,
        PN1 => Second,
        PU1 => Second,
        PS1 => Second,
        PH1 => Second,
        Kara => Phase::Owari,
        Piece::Owari => Phase::Owari,
    }
}
///
/// 先後付き駒→駒種類
///
pub fn pc_to_pt(pc: &Piece) -> PieceType {
    // use super::super::teigi::shogi_syugo::PieceType;
    use super::super::teigi::shogi_syugo::PieceType::*;
    // use super::super::teigi::shogi_syugo::Piece;
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        R0 => K,
        K0 => R,
        Z0 => B,
        I0 => G,
        N0 => S,
        U0 => N,
        S0 => L,
        H0 => P,
        PK0 => PR,
        PZ0 => PB,
        PN0 => PS,
        PU0 => PN,
        PS0 => PL,
        PH0 => PP,
        R1 => K,
        K1 => R,
        Z1 => B,
        I1 => G,
        N1 => S,
        U1 => N,
        S1 => L,
        H1 => P,
        PK1 => PR,
        PZ1 => PB,
        PN1 => PS,
        PU1 => PN,
        PS1 => PL,
        PH1 => PP,
        Piece::Kara => PieceType::Kara,
        Piece::Owari => PieceType::Owari,
    }
}
///
/// 先後付き駒　を　持ち駒種類　へ変換。
/// 持ち駒にするので、先後は反転するぜ☆（＾～＾）
///
pub fn pc_to_hand(km_cap: Piece) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match km_cap {
        R0 => Owari,
        K0 => K1,
        Z0 => Z1,
        I0 => I1,
        N0 => N1,
        U0 => U1,
        S0 => S1,
        H0 => H1,
        PK0 => K1,
        PZ0 => Z1,
        PN0 => N1,
        PU0 => U1,
        PS0 => S1,
        PH0 => H1,
        R1 => Owari,
        K1 => K0,
        Z1 => Z0,
        I1 => I0,
        N1 => N0,
        U1 => U0,
        S1 => S0,
        H1 => H0,
        PK1 => K0,
        PZ1 => Z0,
        PN1 => N0,
        PU1 => U0,
        PS1 => S0,
        PH1 => H0,
        Kara => Owari,
        Owari => Owari,
    }
}

///
/// 駒種類の数値化
///
pub fn kms_to_num(pt: &PieceType) -> usize {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *pt {
        K => 0,
        R => 1,
        B => 2,
        G => 3,
        S => 4,
        N => 5,
        L => 6,
        P => 7,
        PR => 8,
        PB => 9,
        PS => 10,
        PN => 11,
        PL => 12,
        PP => 13,
        Kara => 14,
        Owari => 15,
    }
}
///
/// 数値の駒種類化
///
pub fn num_to_pt(n: usize) -> PieceType {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match n {
        0 => K,
        1 => R,
        2 => B,
        3 => G,
        4 => S,
        5 => N,
        6 => L,
        7 => P,
        8 => PR,
        9 => PB,
        10 => PS,
        11 => PN,
        12 => PL,
        13 => PP,
        14 => Kara,
        _ => Owari,
    }
}
///
/// ハッシュ値を作る
///
pub fn push_kms_to_hash(hash: u64, pt: &PieceType) -> u64 {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    (hash << 4) + kms_to_num(pt) as u64
}
///
/// ハッシュ値から作る
///
pub fn pop_kms_from_hash(hash: u64) -> (u64, PieceType) {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    let kms_num = num_to_pt((hash & 0b1111) as usize);
    (hash >> 4, kms_num)
}
// 駒種類→｛　成駒,（不成駒、それ以外）　｝
pub fn pt_is_pro(pt: &PieceType) -> bool {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *pt {
        K => false,
        R => false,
        B => false,
        G => false,
        S => false,
        N => false,
        L => false,
        P => false,
        PR => true,
        PB => true,
        PS => true,
        PN => true,
        PL => true,
        PP => true,
        Kara => false,
        Owari => false,
    }
}
// 成り駒種類→成る前の駒種類。成り駒でなければ、空に戻る。
pub fn prokms_to_kms(pt: &PieceType) -> PieceType {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *pt {
        K => Kara,
        R => Kara,
        B => Kara,
        G => Kara,
        S => Kara,
        N => Kara,
        L => Kara,
        P => Kara,
        PR => R,
        PB => B,
        PS => S,
        PN => N,
        PL => L,
        PP => P,
        Kara => Kara,
        Owari => Owari,
    }
}
///
/// 駒種類→｛　長い利きの駒か否か　｝
/// 合い駒で防ぎえる可能性があれば真
///
pub fn kms_is_nagaikiki(pt: &PieceType) -> bool {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *pt {
        K => false,
        R => true,
        B => true,
        G => false,
        S => false,
        N => false,
        L => true,
        P => false,
        PR => true,
        PB => true,
        PS => false,
        PN => false,
        PL => false,
        PP => false,
        Kara => false,
        Owari => false,
    }
}
///
/// 成れる駒
///
pub fn kms_can_pro(pt: &PieceType) -> bool {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *pt {
        K => false,
        R => true,
        B => true,
        G => false,
        S => true,
        N => true,
        L => true,
        P => true,
        PR => false,
        PB => false,
        PS => false,
        PN => false,
        PL => false,
        PP => false,
        Kara => false,
        Owari => false,
    }
}
///
/// 打てる駒
///
pub fn kms_can_da(pt: &PieceType) -> bool {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *pt {
        K => false,
        R => true,
        B => true,
        G => true,
        S => true,
        N => true,
        L => true,
        P => true,
        PR => false,
        PB => false,
        PS => false,
        PN => false,
        PL => false,
        PP => false,
        Kara => false,
        Owari => false,
    }
}
/// 先後＆駒種類→先後付き駒
pub fn ph_pt_to_pc(phase: &Phase, pt: &PieceType) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *phase {
        Phase::First => match *pt {
            K => R0,
            R => K0,
            B => Z0,
            G => I0,
            S => N0,
            N => U0,
            L => S0,
            P => H0,
            PR => PK0,
            PB => PZ0,
            PS => PN0,
            PN => PU0,
            PL => PS0,
            PP => PH0,
            _ => Piece::Owari,
        },
        Phase::Second => match *pt {
            K => R1,
            R => K1,
            B => Z1,
            G => I1,
            S => N1,
            N => U1,
            L => S1,
            P => H1,
            PR => PK1,
            PB => PZ1,
            PS => PN1,
            PN => PU1,
            PL => PS1,
            PP => PH1,
            _ => Piece::Owari,
        },
        Phase::Owari => Piece::Owari,
    }
}

///
/// 上下反転
///
pub fn hanten_kmdir_joge(kmdir: &KmDir) -> KmDir {
    use super::super::teigi::shogi_syugo::KmDir::*;
    match *kmdir {
        // 東
        E(b) => E(b),
        // 北東
        NE(b) => SE(b),
        // 北北東（桂馬が戻る動き）
        NNE => SSE,
        // 北
        N(b) => S(b),
        // 北北西（桂馬が戻る動き）
        NNW => SSW,
        // 北西
        NW(b) => SW(b),
        // 西
        W(b) => W(b),
        // 南西
        SW(b) => NW(b),
        // 南南西（桂馬の動き）
        SSW => NNW,
        // 南
        S(b) => N(b),
        // 南南東（桂馬の動き）
        SSE => NNE,
        // 南東
        SE(b) => NE(b),
        // 要素数より1小さい数。エラー値用に使っても可
        Owari => Owari,
    }
}
/*
pub fn kmdir_id(kmdir:&KmDir) -> usize{
    use teigi::shogi_syugo::KmDir::*;
    match *kmdir {
        E  (b)=>if b { 0}else{ 1},
        NE (b)=>if b { 2}else{ 3},
        N  (b)=>if b { 4}else{ 5},
        NW (b)=>if b { 6}else{ 7},
        W  (b)=>if b { 8}else{ 9},
        SW (b)=>if b {10}else{11},
        SSW   =>12,
        S  (b)=>if b {13}else{14},
        SSE   =>15,
        SE (b)=>if b {16}else{17},
        Owari =>18,
    }
}
*/
