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
pub fn jiai_to_num(jiai: &Person) -> usize {
    use super::super::teigi::shogi_syugo::Person::*;
    match *jiai {
        Friend => 0,
        Opponent => 1,
        Owari => 2,
    }
}
pub fn hanten_jiai(jiai: &Person) -> Person {
    use super::super::teigi::shogi_syugo::Person::*;
    match *jiai {
        Friend => Opponent,
        Opponent => Friend,
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
pub fn file_rank_to_sq(suji: i8, dan: i8) -> Square {
    debug_assert!(
        (FILE_0 < suji && suji < FILE_10) && (RANK_0 < dan && dan < RANK_10),
        "(204)file_rank_to_sq suji={},dan={}",
        suji,
        dan
    );

    (suji * 10 + dan) as Square
}
pub fn p_in_ban(p: &Point) -> bool {
    (FILE_0 < p.x && p.x < FILE_10) && (RANK_0 < p.y && p.y < RANK_10)
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
        First => SQ_MAX - sq + SQ_MIN,
        _ => sq,
    }
}

///
/// 先後付き駒の数値化
///
pub fn pc_to_num(pc: &Piece) -> usize {
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        K1 => 0,
        R1 => 1,
        B1 => 2,
        G1 => 3,
        S1 => 4,
        N1 => 5,
        L1 => 6,
        P1 => 7,
        PR1 => 8,
        PB1 => 9,
        PS1 => 10,
        PN1 => 11,
        PL1 => 12,
        PP1 => 13,
        K2 => 14,
        R2 => 15,
        B2 => 16,
        G2 => 17,
        S2 => 18,
        N2 => 19,
        L2 => 20,
        P2 => 21,
        PR2 => 22,
        PB2 => 23,
        PS2 => 24,
        PN2 => 25,
        PL2 => 26,
        PP2 => 27,
        Empty => 28,
        Owari => 29,
    }
}
pub fn num_to_pc(pc_num: usize) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match pc_num {
        0 => K1,
        1 => R1,
        2 => B1,
        3 => G1,
        4 => S1,
        5 => N1,
        6 => L1,
        7 => P1,
        8 => PR1,
        9 => PB1,
        10 => PS1,
        11 => PN1,
        12 => PL1,
        13 => PP1,
        14 => K2,
        15 => R2,
        16 => B2,
        17 => G2,
        18 => S2,
        19 => N2,
        20 => L2,
        21 => P2,
        22 => PR2,
        23 => PB2,
        24 => PS2,
        25 => PN2,
        26 => PL2,
        27 => PP2,
        28 => Empty,
        _ => Owari,
    }
}
///
/// ハッシュ値を作る
///
pub fn push_pc_to_hash(hash: u64, pc: &Piece) -> u64 {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    (hash << 5) + pc_to_num(pc) as u64
}
///
/// ハッシュ値から作る
///
pub fn pop_pc_from_hash(hash: u64) -> (u64, Piece) {
    // 使ってるのは30駒番号ぐらいなんで、32(=2^5) あれば十分
    let pc_num = num_to_pc((hash & 0b11111) as usize);
    (hash >> 5, pc_num)
}
///
/// 駒→成駒　（成れない駒は、そのまま）
///
pub fn pc_to_pro_pc(pc: &Piece) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        K1 => K1,
        R1 => PR1,
        B1 => PB1,
        G1 => G1,
        S1 => PS1,
        N1 => PN1,
        L1 => PL1,
        P1 => PP1,
        PR1 => PR1,
        PB1 => PB1,
        PS1 => PS1,
        PN1 => PN1,
        PL1 => PL1,
        PP1 => PP1,
        K2 => K2,
        R2 => PR2,
        B2 => PB2,
        G2 => G2,
        S2 => PS2,
        N2 => PN2,
        L2 => PL2,
        P2 => PP2,
        PR2 => PR2,
        PB2 => PB2,
        PS2 => PS2,
        PN2 => PN2,
        PL2 => PL2,
        PP2 => PP2,
        Empty => Empty,
        Owari => Owari,
    }
}
///
/// 成駒→駒
///
pub fn pro_pc_to_pc(pc: &Piece) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match *pc {
        K1 => K1,
        R1 => R1,
        B1 => B1,
        G1 => G1,
        S1 => S1,
        N1 => N1,
        L1 => L1,
        P1 => P1,
        PR1 => R1,
        PB1 => B1,
        PS1 => S1,
        PN1 => N1,
        PL1 => L1,
        PP1 => P1,
        K2 => K2,
        R2 => R2,
        B2 => B2,
        G2 => G2,
        S2 => S2,
        N2 => N2,
        L2 => L2,
        P2 => P2,
        PR2 => R2,
        PB2 => B2,
        PS2 => S2,
        PN2 => N2,
        PL2 => L2,
        PP2 => P2,
        Empty => Empty,
        Owari => Owari,
    }
}
///
/// 駒→長い利きの有無
///
pub fn pc_is_long_control(pc: &Piece) -> bool {
    pt_is_long_control(&pc_to_pt(pc))
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
        K1 => (First, K),
        R1 => (First, R),
        B1 => (First, B),
        G1 => (First, G),
        S1 => (First, S),
        N1 => (First, N),
        L1 => (First, L),
        P1 => (First, P),
        PR1 => (First, PR),
        PB1 => (First, PB),
        PS1 => (First, PS),
        PN1 => (First, PN),
        PL1 => (First, PL),
        PP1 => (First, PP),
        K2 => (Second, K),
        R2 => (Second, R),
        B2 => (Second, B),
        G2 => (Second, G),
        S2 => (Second, S),
        N2 => (Second, N),
        L2 => (Second, L),
        P2 => (Second, P),
        PR2 => (Second, PR),
        PB2 => (Second, PB),
        PS2 => (Second, PS),
        PN2 => (Second, PN),
        PL2 => (Second, PL),
        PP2 => (Second, PP),
        Piece::Empty => (Phase::Owari, PieceType::Empty),
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
        K1 => First,
        R1 => First,
        B1 => First,
        G1 => First,
        S1 => First,
        N1 => First,
        L1 => First,
        P1 => First,
        PR1 => First,
        PB1 => First,
        PS1 => First,
        PN1 => First,
        PL1 => First,
        PP1 => First,
        K2 => Second,
        R2 => Second,
        B2 => Second,
        G2 => Second,
        S2 => Second,
        N2 => Second,
        L2 => Second,
        P2 => Second,
        PR2 => Second,
        PB2 => Second,
        PS2 => Second,
        PN2 => Second,
        PL2 => Second,
        PP2 => Second,
        Empty => Phase::Owari,
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
        K1 => K,
        R1 => R,
        B1 => B,
        G1 => G,
        S1 => S,
        N1 => N,
        L1 => L,
        P1 => P,
        PR1 => PR,
        PB1 => PB,
        PS1 => PS,
        PN1 => PN,
        PL1 => PL,
        PP1 => PP,
        K2 => K,
        R2 => R,
        B2 => B,
        G2 => G,
        S2 => S,
        N2 => N,
        L2 => L,
        P2 => P,
        PR2 => PR,
        PB2 => PB,
        PS2 => PS,
        PN2 => PN,
        PL2 => PL,
        PP2 => PP,
        Piece::Empty => PieceType::Empty,
        Piece::Owari => PieceType::Owari,
    }
}
///
/// 先後付き駒　を　持ち駒種類　へ変換。
/// 持ち駒にするので、先後は反転するぜ☆（＾～＾）
///
pub fn pc_to_hand(pc_cap: Piece) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    match pc_cap {
        K1 => Owari,
        R1 => R2,
        B1 => B2,
        G1 => G2,
        S1 => S2,
        N1 => N2,
        L1 => L2,
        P1 => P2,
        PR1 => R2,
        PB1 => B2,
        PS1 => S2,
        PN1 => N2,
        PL1 => L2,
        PP1 => P2,
        K2 => Owari,
        R2 => R1,
        B2 => B1,
        G2 => G1,
        S2 => S1,
        N2 => N1,
        L2 => L1,
        P2 => P1,
        PR2 => R1,
        PB2 => B1,
        PS2 => S1,
        PN2 => N1,
        PL2 => L1,
        PP2 => P1,
        Empty => Owari,
        Owari => Owari,
    }
}

///
/// 駒種類の数値化
///
pub fn pt_to_num(pt: &PieceType) -> usize {
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
        Empty => 14,
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
        14 => Empty,
        _ => Owari,
    }
}
///
/// ハッシュ値を作る
///
pub fn push_pt_to_hash(hash: u64, pt: &PieceType) -> u64 {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    (hash << 4) + pt_to_num(pt) as u64
}
///
/// ハッシュ値から作る
///
pub fn pop_pt_from_hash(hash: u64) -> (u64, PieceType) {
    // 使ってるのは16駒種類番号ぐらいなんで、16(=2^4) あれば十分
    let pt_num = num_to_pt((hash & 0b1111) as usize);
    (hash >> 4, pt_num)
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
        Empty => false,
        Owari => false,
    }
}
// 成り駒種類→成る前の駒種類。成り駒でなければ、空に戻る。
pub fn pro_pt_to_pt(pt: &PieceType) -> PieceType {
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *pt {
        K => Empty,
        R => Empty,
        B => Empty,
        G => Empty,
        S => Empty,
        N => Empty,
        L => Empty,
        P => Empty,
        PR => R,
        PB => B,
        PS => S,
        PN => N,
        PL => L,
        PP => P,
        Empty => Empty,
        Owari => Owari,
    }
}
///
/// 駒種類→｛　長い利きの駒か否か　｝
/// 合い駒で防ぎえる可能性があれば真
///
pub fn pt_is_long_control(pt: &PieceType) -> bool {
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
        Empty => false,
        Owari => false,
    }
}
///
/// 成れる駒
///
pub fn pt_can_pro(pt: &PieceType) -> bool {
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
        Empty => false,
        Owari => false,
    }
}
///
/// 打てる駒
///
pub fn pt_can_drop(pt: &PieceType) -> bool {
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
        Empty => false,
        Owari => false,
    }
}
/// 先後＆駒種類→先後付き駒
pub fn ph_pt_to_pc(phase: &Phase, pt: &PieceType) -> Piece {
    use super::super::teigi::shogi_syugo::Piece::*;
    use super::super::teigi::shogi_syugo::PieceType::*;
    match *phase {
        Phase::First => match *pt {
            K => K1,
            R => R1,
            B => B1,
            G => G1,
            S => S1,
            N => N1,
            L => L1,
            P => P1,
            PR => PR1,
            PB => PB1,
            PS => PS1,
            PN => PN1,
            PL => PL1,
            PP => PP1,
            _ => Piece::Owari,
        },
        Phase::Second => match *pt {
            K => K2,
            R => R2,
            B => B2,
            G => G2,
            S => S2,
            N => N2,
            L => L2,
            P => P2,
            PR => PR2,
            PB => PB2,
            PS => PS2,
            PN => PN2,
            PL => PL2,
            PP => PP2,
            _ => Piece::Owari,
        },
        Phase::Owari => Piece::Owari,
    }
}

///
/// 上下反転
///
pub fn hanten_pc_dir_joge(pc_dir: &PcDir) -> PcDir {
    use super::super::teigi::shogi_syugo::PcDir::*;
    match *pc_dir {
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
pub fn pc_dir_id(pc_dir:&PcDir) -> usize{
    use teigi::shogi_syugo::PcDir::*;
    match *pc_dir {
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
