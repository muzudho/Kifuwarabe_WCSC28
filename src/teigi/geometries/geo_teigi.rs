#![allow(dead_code)]
//!
//! 数学
//!

use std::fmt;

pub struct Point {
    pub x: i8,
    pub y: i8,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

///
/// 一致
///
pub enum MatchingResult {
    /// 一致した
    Matched,
    /// 一致しなかった
    Unmatched,
    /// 比較は無効
    Owari,
}

/// 4偏角(argument angle)
///
/// 盤の方向は、後手から見た視点
pub enum ArgAngle4 {
    // 0° 段
    Arg0,
    // 45° 左下がり
    Arg45,
    // 90° 筋
    Arg90,
    // 135° 左上がり
    Arg135,
    // ２点が重なっていて角度を測れないとき
    Overlapped,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari,
}
impl fmt::Display for ArgAngle4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use super::super::super::teigi::geometries::geo_teigi::ArgAngle4::*;
        write!(
            f,
            "{}",
            match *self {
                Arg0 => "0°",
                Arg45 => "45°",
                Arg90 => "90°",
                Arg135 => "135°",
                Overlapped => "重",
                _ => "×",
            }
        )
    }
}
pub fn argangle4_to_num(angle: &ArgAngle4) -> usize {
    use super::super::super::teigi::geometries::geo_teigi::ArgAngle4::*;
    match *angle {
        Arg0 => 0,
        Arg45 => 1,
        Arg90 => 2,
        Arg135 => 3,
        Overlapped => 4,
        Owari => 5,
    }
}
pub fn num_to_argangle4(n: usize) -> ArgAngle4 {
    use super::super::super::teigi::geometries::geo_teigi::ArgAngle4::*;
    match n {
        0 => Arg0,
        1 => Arg45,
        2 => Arg90,
        3 => Arg135,
        4 => Overlapped,
        _ => Owari,
    }
}
///
/// ハッシュ値を作る
///
pub fn push_argangle4_to_hash(hash: u64, angle: &ArgAngle4) -> u64 {
    // エラー値含めて 6bit あるので 2^3
    (hash << 3) + argangle4_to_num(angle) as u64
}
///
/// ハッシュ値から作る
///
pub fn pop_argangle4_from_hash(hash: u64) -> (u64, ArgAngle4) {
    // エラー値含めて 6bit あるので 2^3
    let angle = num_to_argangle4((hash & 0b11111) as usize);
    (hash >> 3, angle)
}

///
/// 同じ位置か
///
pub fn match_p_p(a: &Point, b: &Point) -> bool {
    // 上1桁が同じか
    a.x == b.x && a.y == b.y
}

///
/// 同じ段に駒があるか
///
pub fn match_argangle0_p_p(a: &Point, b: &Point) -> bool {
    // 下1桁が同じか
    a.y == b.y
}
///
/// 同じ左下がり筋に駒があるか
///
pub fn match_argangle45_p_p(a: &Point, b: &Point) -> bool {
    // ( 一の位の数 - 1 ) の数を、十の位の数 から引くと、一筋目のどの段か分かる。
    // このとき、-7～9 段まで、17段階の斜めの段があることになる
    // ただし unsigned型では 負数 を表現できないので、
    // 8の下駄を履かせて 1～17段にずらす
    a.x + 8 - (a.y - 1) == b.x + 8 - (b.y - 1)
}
///
/// 同じ筋に駒があるか
///
pub fn match_argangle90_p_p(a: &Point, b: &Point) -> bool {
    // 上1桁が同じか
    a.x == b.x
}
///
/// 同じ左上がり筋に駒があるか
///
pub fn match_argangle135_p_p(a: &Point, b: &Point) -> bool {
    // ( 一の位の数 - 1 ) の数を、十の位の数 に足すと、一筋目のどの段か分かる。
    // このとき、1～17 段まで、17段階の斜めの段があることになる
    a.x + (a.y - 1) == b.x + (b.y - 1)
}

///
/// ２つの点が直線上に並んでいる場合の、角度（４方向）。
/// 盤の方向は、後手から見た視点。
/// ２つの点が（４つの角度の）直線上に並んでいない、または、同じ位置を指定した場合、Owari を返す。
///
pub fn get_argangle4_p_p(a: &Point, b: &Point) -> ArgAngle4 {
    // 同じ位置
    if match_p_p(a, b) {
        ArgAngle4::Overlapped
    // 段が同じ
    } else if match_argangle0_p_p(a, b) {
        ArgAngle4::Arg0
    // 左下がり筋が同じ
    } else if match_argangle45_p_p(a, b) {
        ArgAngle4::Arg45
    // 筋が同じ
    } else if match_argangle90_p_p(a, b) {
        ArgAngle4::Arg90
    // 左上がり筋が同じ
    } else if match_argangle135_p_p(a, b) {
        ArgAngle4::Arg135
    // 並んでいない
    } else {
        ArgAngle4::Owari
    }
}

/// 狭義の順序（＝同じを許容しない）
/// a < b < c または、 a > b > c なら真。
pub fn irreflexive_ordered3_i8(a: i8, b: i8, c: i8) -> bool {
    (a < b && b < c) || (a > b && b > c)
}

/// 広義の順序（＝同じを許容する）
/// a ≦ b ≦ c または、 a ≧ b ≧ c なら真。
pub fn reflexive_ordered3_i8(a: i8, b: i8, c: i8) -> bool {
    (a <= b && b <= c) || (a >= b && b >= c)
}

///
/// 角度の一致比較。
///
/// どちらか一方でも「角度なし」の場合、一致と判定。
/// ただし、Owari は何とも一致しないとする。
///
pub fn match_argangle4(a: &ArgAngle4, b: &ArgAngle4) -> MatchingResult {
    if argangle4_to_num(&ArgAngle4::Owari) == argangle4_to_num(a)
        || argangle4_to_num(&ArgAngle4::Owari) == argangle4_to_num(b)
    {
        MatchingResult::Owari
    } else if argangle4_to_num(&ArgAngle4::Overlapped) == argangle4_to_num(a)
        || argangle4_to_num(&ArgAngle4::Overlapped) == argangle4_to_num(b)
        || argangle4_to_num(a) == argangle4_to_num(b)
    {
        MatchingResult::Matched
    } else {
        MatchingResult::Unmatched
    }
}
