//!
//! 命題
//!
use super::super::teigi::geometries::geo_teigi::*;

/// 升Pは、点ABで作る平面上にあるか？
pub fn intersect_point_on_plane(p: &Point, a: &Point, b: &Point) -> bool {
    // まず、x,y軸共に 点P は 線分AB の内側にあるよう、絞り込む
    if reflexive_ordered3_i8(a.x, p.x, b.x) && reflexive_ordered3_i8(a.y, p.y, b.y) {
        return true;
    }
    false
}

/// 升Pは、線分AB上にあるか？
pub fn intersect_point_on_line_segment(p: &Point, a: &Point, b: &Point) -> bool {
    // 升Pは、点ABで作る平面上にあるか？
    if intersect_point_on_plane(&p, &a, &b) {
        // ２本の線分が同じ角度なら、同じ線上にいると判定
        // 角度を持たない点は、一致と判定
        let angle4ab = get_argangle4_p_p(&a, &b);
        let angle4pa = get_argangle4_p_p(&p, &a);
        match match_argangle4(&angle4pa, &angle4ab) {
            MatchingResult::Matched => return true,
            _ => {}
        }
    }
    false
}
