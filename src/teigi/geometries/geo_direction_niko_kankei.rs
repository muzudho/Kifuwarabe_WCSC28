//!
//! 幾何学の二項関係☆（＾～＾）
//! 要するに　将棋盤の上で　北側の升はどこかとか調べるやつだぜ☆（＾～＾）
//!
use super::super::super::teigi::geometries::geo_teigi::*;

/**
 * 升0は、升1の西にあるか
 */
pub fn match_a_west_of_b(a: &Point, b: &Point) -> bool {
    // 上1桁が筋
    a.x < b.x
}
/**
 * 升0は、升1の南にあるか
 */
pub fn match_a_south_of_b(a: &Point, b: &Point) -> bool {
    // 下1桁が段
    a.y < b.y
}
