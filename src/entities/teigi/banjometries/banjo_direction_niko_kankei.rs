#![allow(dead_code)]
//!
//! 盤上の二項関係☆（＾～＾）
//!
use super::super::super::consoles::asserts::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::geometries::geo_direction_niko_kankei::*;
use super::super::super::teigi::geometries::geo_teigi::*;
use super::super::super::teigi::shogi_syugo::*;

///
/// 狙われている駒から見た、長い利きの駒の居る方向（８方向）
///
/// 盤の方向は、後手から見た視点
/// 引数には、同じ升を指定しないものとする
///
pub fn get_dir8_to_slider_from_target(
    ms_slider: Square,
    km_slider: &Piece,
    ms_target: Square,
) -> Dir8 {
    debug_assert!(
        ms_slider != ms_target,
        "dosn't sq{}!={}",
        ms_slider,
        ms_target
    );

    assert_banjo_ms(ms_slider, "(205a1)get_dir8_to_slider_from_target");
    assert_banjo_ms(ms_target, "(205a2)get_dir8_to_slider_from_target");
    let p_slider = ms_to_p(ms_slider);
    let p_target = ms_to_p(ms_target);

    let (sn_slider, pt) = pc_to_ph_pt(&km_slider);
    use super::super::super::teigi::shogi_syugo::Phase::*;
    use super::super::super::teigi::shogi_syugo::PieceType::*;
    match pt {
        R => {
            // 筋か、段かのどちらかが同じ
            if match_argangle0_p_p(&p_slider, &p_target) {
                if match_a_south_of_b(&p_slider, &p_target) {
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle90_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else {
                Dir8::Owari
            }
        }
        B => {
            // 左上がり筋か、左下がり筋かのどちらかが同じ
            if match_argangle45_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::SW
                } else {
                    Dir8::NE
                }
            } else if match_argangle135_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                Dir8::Owari
            }
        }
        L => {
            // 先後
            match sn_slider {
                First => Dir8::N,
                Second => Dir8::S,
                _ => Dir8::Owari,
            }
        }
        PR => {
            // 筋か、段か、
            // 左上がり筋か、左下がり筋かの　いずれかが同じ
            if match_argangle0_p_p(&p_slider, &p_target) {
                if match_a_south_of_b(&p_slider, &p_target) {
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle45_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else if match_argangle90_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else if match_argangle135_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                Dir8::Owari
            }
        }
        PB => {
            // 筋か、段か、
            // 左上がり筋か、左下がり筋かの　いずれかが同じ
            if match_argangle0_p_p(&p_slider, &p_target) {
                if match_a_south_of_b(&p_slider, &p_target) {
                    Dir8::S
                } else {
                    Dir8::N
                }
            } else if match_argangle45_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else if match_argangle90_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::W
                } else {
                    Dir8::E
                }
            } else if match_argangle135_p_p(&p_slider, &p_target) {
                if match_a_west_of_b(&p_slider, &p_target) {
                    Dir8::NW
                } else {
                    Dir8::SE
                }
            } else {
                Dir8::Owari
            }
        }
        _ => Dir8::Owari,
    }
}
