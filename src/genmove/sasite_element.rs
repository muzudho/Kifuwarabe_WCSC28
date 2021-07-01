//!
//! 指し手の要素☆（＾～＾）
//!

use super::super::entities::consoles::asserts::*;
use super::super::entities::jotai::uchu::*;
use super::super::entities::teigi::conv::*;
use super::super::entities::teigi::shogi_syugo::*;
use std::collections::HashSet;

///
/// 成る前を含めない、移動元升生成
///
/// 1. 移動先を指定          to
/// 2. 移動先にある駒を指定  to_pc
///
/// その願いが叶う移動元の一覧を返す。
/// 最大２０升。合法手生成の逆の動き☆（＾～＾）
///
/// 「成る前」を調べるのは別関数でやるぜ☆（＾～＾）
///
/// TODO 先手１段目の香車とか、必ず成らないといけないぜ☆（＾～＾）
///
pub fn insert_nopromote_from_by_sq_pc(
    to: Square,
    to_pc: &Piece,
    uchu: &Uchu,
    result: &mut HashSet<Square>,
) {
    assert_banjo_ms(to, "ｉnsert_narazu_src_by_ms_km");

    /*
     * Square は 将棋盤座標
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     *
     * x,y を使うと混乱するので、s,d を使う
     */
    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx, dy) = sq_to_file_rank(to);
    let phase = pc_to_ph(to_pc);
    let to_pt = pc_to_pt(&to_pc);
    let kms_num = kms_to_num(&to_pt);

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use super::super::entities::teigi::shogi_syugo::Piece::*;
    match *to_pc {
        U0 => {
            // ▼うさぎ　は１、２段目には進めない
            if dy < DAN_3 {
                return;
            }
        }
        S0 | H0 => {
            // ▼しし、▼ひよこ　は１段目には進めない
            if dy < DAN_2 {
                return;
            }
        }
        U1 => {
            // △うさぎ　は８、９段目には進めない
            if DAN_7 < dy {
                return;
            }
        }
        S1 | H1 => {
            // △しし、△ひよこ　は９段目には進めない
            if DAN_8 < dy {
                return;
            }
        }
        _ => {}
    }

    for i_dir in 0..KM_UGOKI_LN {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &KmDir;
        if match_sn(&Phase::First, &phase) {
            p_kmdir = &KM_UGOKI.back[kms_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        use super::super::entities::teigi::shogi_syugo::KmDir::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    for i_east in 1..9 {
                        if dx + i_east < SUJI_10 {
                            let from = suji_dan_to_ms(dx + i_east, dy);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 西東
                    if dx + 1 < SUJI_10 {
                        let from = suji_dan_to_ms(dx + 1, dy);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    for i_ne in 1..9 {
                        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                            let from = suji_dan_to_ms(dx + i_ne, dy + i_ne);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北東
                    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                        let from = suji_dan_to_ms(dx + 1, dy + 1);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北北東
            NNE => {
                if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                    let from = suji_dan_to_ms(dx + 1, dy + 2);
                    if uchu.ky.has_sq_pc(from, to_pc) {
                        result.insert(from);
                    }
                }
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    for i_south in 1..9 {
                        if dy + i_south < DAN_10 {
                            let from = suji_dan_to_ms(dx, dy + i_south);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北
                    if dy + 1 < DAN_10 {
                        let from = suji_dan_to_ms(dx, dy + 1);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北北西
            NNW => {
                if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                    let from = suji_dan_to_ms(dx - 1, dy + 2);
                    if uchu.ky.has_sq_pc(from, to_pc) {
                        result.insert(from);
                    }
                }
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    for i_se in 1..9 {
                        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                            let from = suji_dan_to_ms(dx - i_se, dy + i_se);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北西
                    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                        let from = suji_dan_to_ms(dx - 1, dy + 1);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    for i_east in 1..9 {
                        if SUJI_0 < dx - i_east {
                            // 進みたいマスから戻ったマス
                            let from = suji_dan_to_ms(dx - i_east, dy);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                // 指定の駒があれば、その升は移動元。続行
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                // なんか他の駒があれば終わり
                                break;
                            }
                        }
                    }
                } else {
                    // 西
                    if SUJI_0 < dx - 1 {
                        let from = suji_dan_to_ms(dx - 1, dy);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    for i_ne in 1..9 {
                        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                            let from = suji_dan_to_ms(dx - i_ne, dy - i_ne);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南西
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx - 1, dy - 1);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南南西
            SSW => {
                if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                    let from = suji_dan_to_ms(dx - 1, dy - 2);
                    if uchu.ky.has_sq_pc(from, to_pc) {
                        result.insert(from);
                    }
                }
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    for i_north in 1..9 {
                        if DAN_0 < dy - i_north {
                            let from = suji_dan_to_ms(dx, dy - i_north);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南
                    if DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx, dy - 1);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南南東
            SSE => {
                if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                    let from = suji_dan_to_ms(dx + 1, dy - 2);
                    if uchu.ky.has_sq_pc(from, to_pc) {
                        result.insert(from);
                    }
                }
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    for i_nw in 1..9 {
                        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                            let from = suji_dan_to_ms(dx + i_nw, dy - i_nw);
                            if uchu.ky.has_sq_pc(from, to_pc) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南東
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx + 1, dy - 1);
                        if uchu.ky.has_sq_pc(from, to_pc) {
                            result.insert(from);
                        }
                    }
                }
            }
            Owari => break,
        }
    }
}
///
/// 成る前の移動元升生成
///
/// 1. 移動先の升        to
/// 2. 移動先にある駒    to_pc
///
/// 成り　の動きでその結果になるような、元の升を返す☆（＾～＾）
///
pub fn insert_beforepromote_from_by_sq_pc(
    to: Square,
    to_pc: &Piece,
    uchu: &Uchu,
    result: &mut HashSet<Square>,
) {
    assert_banjo_ms(to, "Ｉnsert_narumae_src_by_ms_km");

    // +--------------------+
    // | 移動後は成り駒か？ |
    // +--------------------+
    let to_pt = pc_to_pt(&to_pc);
    if !pt_is_pro(&to_pt) {
        return; // 成り駒でないなら、成りの動きをしていない
    }

    // +--------------------+
    // | 移動前は成る前の駒 |
    // +--------------------+
    let phase = pc_to_ph(to_pc);
    let from_pt = prokms_to_kms(&to_pt);
    let pc_from = ph_pt_to_pc(&phase, &from_pt);

    /*
     * Square は 将棋盤座標
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     *
     * x,y を使うと混乱するので、s,d を使う
     */
    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx, dy) = sq_to_file_rank(to);

    // 例えば移動先の駒種類が「ぱひ」なら、「ぱひ」が動いた可能性の他に、
    // 「ひ」が動いたのかもしれない。
    // 「ぱひ」は、敵陣の１～３段目にいて、動きが北だった場合、元が「ひ」の可能性がある。
    let kms_src_narumae = prokms_to_kms(&to_pt);

    use super::super::entities::teigi::shogi_syugo::PieceType::*;
    match kms_src_narumae {
        Kara => {
            return;
        } // 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
        _ => {} // 成れる駒は、成る前の駒の動きも調べる
    }

    let kms_narumae_num = kms_to_num(&kms_src_narumae);

    for i_dir in 0..KM_UGOKI_LN {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &KmDir;
        if match_sn(&Phase::First, &phase) {
            p_kmdir = &KM_UGOKI.back[kms_narumae_num][i_dir]
        } else {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_narumae_num][i_dir]);
            p_kmdir = &_kmdir;
        };

        // 移動先を開始地点にして、駒の位置を終了地点にする
        use super::super::entities::teigi::shogi_syugo::KmDir::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    for i_east in 1..9 {
                        if dx + i_east < SUJI_10 {
                            let from = suji_dan_to_ms(dx + i_east, dy);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 西東
                    if dx + 1 < SUJI_10 {
                        let from = suji_dan_to_ms(dx + 1, dy);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    for i_ne in 1..9 {
                        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                            let from = suji_dan_to_ms(dx + i_ne, dy + i_ne);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北東
                    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                        let from = suji_dan_to_ms(dx + 1, dy + 1);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北北東
            NNE => {
                if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                    let from = suji_dan_to_ms(dx + 1, dy + 2);
                    if uchu.ky.has_sq_pc(from, &pc_from) {
                        result.insert(from);
                    }
                }
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    for i_south in 1..9 {
                        if dy + i_south < DAN_10 {
                            let from = suji_dan_to_ms(dx, dy + i_south);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北
                    if dy + 1 < DAN_10 {
                        let from = suji_dan_to_ms(dx, dy + 1);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北北西
            NNW => {
                if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                    let from = suji_dan_to_ms(dx - 1, dy + 2);
                    if uchu.ky.has_sq_pc(from, &pc_from) {
                        result.insert(from);
                    }
                }
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    for i_se in 1..9 {
                        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                            let from = suji_dan_to_ms(dx - i_se, dy + i_se);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北西
                    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                        let from = suji_dan_to_ms(dx - 1, dy + 1);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    for i_east in 1..9 {
                        if SUJI_0 < dx - i_east {
                            // 進みたいマスから戻ったマス
                            let from = suji_dan_to_ms(dx - i_east, dy);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                // 指定の駒があれば、その升は移動元。続行
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                // なんか他の駒があれば終わり
                                break;
                            }
                        }
                    }
                } else {
                    // 西
                    if SUJI_0 < dx - 1 {
                        let from = suji_dan_to_ms(dx - 1, dy);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    for i_ne in 1..9 {
                        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                            let from = suji_dan_to_ms(dx - i_ne, dy - i_ne);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南西
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx - 1, dy - 1);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南南西
            SSW => {
                if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                    let from = suji_dan_to_ms(dx - 1, dy - 2);
                    if uchu.ky.has_sq_pc(from, &pc_from) {
                        result.insert(from);
                    }
                }
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    for i_north in 1..9 {
                        if DAN_0 < dy - i_north {
                            let from = suji_dan_to_ms(dx, dy - i_north);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南
                    if DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx, dy - 1);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南南東
            SSE => {
                if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                    let from = suji_dan_to_ms(dx + 1, dy - 2);
                    if uchu.ky.has_sq_pc(from, &pc_from) {
                        result.insert(from);
                    }
                }
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    for i_nw in 1..9 {
                        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                            let from = suji_dan_to_ms(dx + i_nw, dy - i_nw);
                            if uchu.ky.has_sq_pc(from, &pc_from) {
                                result.insert(from);
                            } else if uchu.ky.exists_pc(from) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南東
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx + 1, dy - 1);
                        if uchu.ky.has_sq_pc(from, &pc_from) {
                            result.insert(from);
                        }
                    }
                }
            }
            Owari => break,
        }
    }
}
///
/// 打の駒種類生成
///
/// 1. 移動先の升    to
/// 2. 移動先の駒    to_pc  ※先後が要るので、kmsではなくkm。
///
/// そこに打てる駒種類を返す。
///
pub fn insert_drop_pt_by_sq_pc(
    to: Square,
    to_pc: &Piece,
    uchu: &Uchu,
    result_kms: &mut HashSet<usize>,
) {
    assert_banjo_ms(to, "Ｉnsert_da_kms_by_ms_km");

    let to_pt = pc_to_pt(&to_pc);
    if !kms_can_da(&to_pt) {
        return; // 打って出てくることがない駒なら終了
    }

    // +------------------------+
    // | 打ちたいところは空升か |
    // +------------------------+
    let km_banjo = uchu.ky.get_pc_by_sq(to);
    match km_banjo {
        Piece::Kara => {}
        _ => {
            return;
        } // 駒があるところに打つ手は終了
    }
    // 駒が無いところに打つ

    // +------------------+
    // | 持っている駒か？ |
    // +------------------+
    if uchu.ky.get_mg(&to_pc) < 1 {
        return; // 持っていない駒は打てない
    }

    // 回転していない将棋盤から見た筋番号
    let (suji, dy) = sq_to_file_rank(to);
    /*
     * Square は 将棋盤座標
     *
     * 考えることを打に限れば、先手も、後手も、後手から見た座標を使えば十分だぜ☆（＾～＾）
     *
     * ...
     * 13 23 33
     * 12 22 32
     * 11 21 31 ...
     */
    let phase = pc_to_ph(to_pc);
    let sq = kaiten180_ms_by_ms_sn(to, &phase);

    assert_banjo_ms(sq, "Ｉnsert_da_kms_by_ms_km＜その２＞");
    //let (_x,y) = sq_to_file_rank(sq);

    // 行先の無いところに駒を進めることの禁止☆（＾～＾）
    use super::super::entities::teigi::shogi_syugo::Piece::*;
    match *to_pc {
        U0 => {
            // ▼うさぎ　は１、２段目には進めない
            if dy < DAN_3 {
                return;
            }
        }
        // ▼しし、▼ひよこ　は１段目には進めない
        S0 => {
            if dy < DAN_2 {
                return;
            }
        }
        H0 => {
            // ▼ひよこ　は２歩できない
            if dy < DAN_2 || uchu.ky.exists_fu_by_sn_suji(&phase, suji) {
                return;
            }
        }
        U1 => {
            // △うさぎ　は８、９段目には進めない
            if DAN_7 < dy {
                return;
            }
        }
        // △しし、△ひよこ　は９段目には進めない
        S1 => {
            if DAN_8 < dy {
                return;
            }
        }
        H1 => {
            // △ひよこ　は２歩できない
            if DAN_8 < dy || uchu.ky.exists_fu_by_sn_suji(&phase, suji) {
                return;
            }
        }
        _ => {}
    }
    result_kms.insert(kms_to_num(&to_pt));
}
///
/// 移動先升生成
///
/// 1. 移動元升
/// 2. 移動したい駒
///
/// 駒の移動先を取得。合法手生成の動き☆（＾～＾）
///
/// pc_from   : 移動元の駒
/// from   : 移動元の升
/// to_nari  : 成りの手を生成するなら真
/// ky       : 現局面
///
pub fn insert_dst_by_ms_km(
    from: Square,
    pc_from: &Piece,
    to_nari: bool,
    uchu: &Uchu,
    result: &mut HashSet<Square>,
) {
    assert_banjo_ms(from, "Ｉnsert_dst_by_ms_km");

    // 移動先の筋、段、駒種類、駒種類インデックス
    let (dx, dy) = sq_to_file_rank(from);
    let phase = pc_to_ph(&pc_from);
    let from_pt = pc_to_pt(&pc_from);

    // +--------------+
    // | 成れる駒か？ |
    // +--------------+
    if to_nari && !kms_can_pro(&from_pt) {
        return; // 成れる駒でないなら、成りの動きはしない
    }
    let kms_num = kms_to_num(&from_pt);

    for i_dir in 0..KM_UGOKI_LN {
        // 指定の駒種類の、全ての逆向きに動ける方向
        let _kmdir;
        let p_kmdir: &KmDir;
        if match_sn(&Phase::First, &phase) {
            _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
            p_kmdir = &_kmdir;
        } else {
            p_kmdir = &KM_UGOKI.back[kms_num][i_dir]
        };

        // 駒の位置を開始地点に、離れていくように調べていく
        use super::super::entities::teigi::shogi_syugo::KmDir::*;
        match *p_kmdir {
            // 東
            E(b) => {
                if b {
                    // 長東
                    for i_east in 1..9 {
                        if dx + i_east < SUJI_10 {
                            let from = suji_dan_to_ms(dx + i_east, dy);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 西東
                    if dx + 1 < SUJI_10 {
                        let from = suji_dan_to_ms(dx + 1, dy);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北東
            NE(b) => {
                if b {
                    // 長北東
                    for i_ne in 1..9 {
                        if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                            let from = suji_dan_to_ms(dx + i_ne, dy + i_ne);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北東
                    if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                        let from = suji_dan_to_ms(dx + 1, dy + 1);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北北東
            NNE => {
                if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                    let from = suji_dan_to_ms(dx + 1, dy + 2);
                    let sn_ms = uchu.ky.get_sn_by_ms(from);
                    if !match_sn(&sn_ms, &phase) {
                        result.insert(from);
                    }
                }
            }
            // 北
            N(b) => {
                if b {
                    // 長北
                    for i_south in 1..9 {
                        if dy + i_south < DAN_10 {
                            let from = suji_dan_to_ms(dx, dy + i_south);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北
                    if dy + 1 < DAN_10 {
                        let from = suji_dan_to_ms(dx, dy + 1);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 北北西
            NNW => {
                if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                    let from = suji_dan_to_ms(dx - 1, dy + 2);
                    let sn_ms = uchu.ky.get_sn_by_ms(from);
                    if !match_sn(&sn_ms, &phase) {
                        result.insert(from);
                    }
                }
            }
            // 北西
            NW(b) => {
                if b {
                    // 長北西
                    for i_se in 1..9 {
                        if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                            let from = suji_dan_to_ms(dx - i_se, dy + i_se);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 北西
                    if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                        let from = suji_dan_to_ms(dx - 1, dy + 1);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 西
            W(b) => {
                if b {
                    // 長西
                    for i_east in 1..9 {
                        if SUJI_0 < dx - i_east {
                            let from = suji_dan_to_ms(dx - i_east, dy);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 西
                    if SUJI_0 < dx - 1 {
                        let from = suji_dan_to_ms(dx - 1, dy);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南西
            SW(b) => {
                if b {
                    // 長南西
                    for i_ne in 1..9 {
                        if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                            let from = suji_dan_to_ms(dx - i_ne, dy - i_ne);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南西
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx - 1, dy - 1);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南南西
            SSW => {
                if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                    let from = suji_dan_to_ms(dx - 1, dy - 2);
                    let sn_ms = uchu.ky.get_sn_by_ms(from);
                    if !match_sn(&sn_ms, &phase) {
                        result.insert(from);
                    }
                }
            }
            // 南
            S(b) => {
                if b {
                    // 長南
                    for i_north in 1..9 {
                        if DAN_0 < dy - i_north {
                            let from = suji_dan_to_ms(dx, dy - i_north);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南
                    if DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx, dy - 1);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            // 南南東
            SSE => {
                if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                    let from = suji_dan_to_ms(dx + 1, dy - 2);
                    let sn_ms = uchu.ky.get_sn_by_ms(from);
                    if !match_sn(&sn_ms, &phase) {
                        result.insert(from);
                    }
                }
            }
            // 南東
            SE(b) => {
                if b {
                    // 長南東
                    for i_nw in 1..9 {
                        if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                            let from = suji_dan_to_ms(dx + i_nw, dy - i_nw);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            if !match_sn(&sn_ms, &phase) {
                                result.insert(from);
                            }
                            if !match_sn(&sn_ms, &Phase::Owari) {
                                break;
                            }
                        }
                    }
                } else {
                    // 南東
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                        let from = suji_dan_to_ms(dx + 1, dy - 1);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        if !match_sn(&sn_ms, &phase) {
                            result.insert(from);
                        }
                    }
                }
            }
            Owari => break,
        }
    }

    if to_nari {
        // +------------------------------+
        // | 成れる動き以外での成りの禁止 |
        // +------------------------------+
        use super::super::entities::teigi::shogi_syugo::Piece::*;
        match *pc_from {
            K0 | Z0 | N0 => {
                // ▼きりん、▼ぞう、▼ねこ　は
                // 移動元または移動先が　１～３段目なら成れる
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_sx2, sy2) = sq_to_file_rank(from);
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if sy2 < DAN_4 && dy2 < DAN_4 {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            U0 | S0 | H0 => {
                // ▼うさぎ、▼しし、▼ひよこ　は
                // 移動先が　１～３段目なら成れる
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if dy2 < DAN_4 {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            K1 | Z1 | N1 => {
                // △きりん、△ぞう、△ねこ　は
                // 移動元または移動先が　７～９段目なら成れる
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_sx2, sy2) = sq_to_file_rank(from);
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if DAN_6 < sy2 && DAN_6 < dy2 {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            U1 | S1 | H1 => {
                // △うさぎ、△しし、△ひよこ　は
                // 移動先が　７～９段目なら成れる
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if DAN_6 < dy2 {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            _ => {}
        }
    } else {
        // +----------------------------------------+
        // | 行先の無いところに駒を進めることの禁止 |
        // +----------------------------------------+
        use super::super::entities::teigi::shogi_syugo::Piece::*;
        match *pc_from {
            U0 => {
                // ▼うさぎ　は１、２段目には進めない
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if dy2 < DAN_3 {
                    } else {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            S0 | H0 => {
                // ▼しし、▼ひよこ　は１段目には進めない
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if dy2 < DAN_2 {
                    } else {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            U1 => {
                // △うさぎ　は８、９段目には進めない
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if DAN_7 < dy2 {
                    } else {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            S1 | H1 => {
                // △しし、△ひよこ　は９段目には進めない
                let mut result2: HashSet<Square> = HashSet::new();
                for to in result.iter() {
                    let (_dx2, dy2) = sq_to_file_rank(*to);
                    if DAN_8 < dy2 {
                    } else {
                        result2.insert(*to);
                    }
                }
                // 入れ直し
                result.clear();
                for to in result2.iter() {
                    result.insert(*to);
                }
            }
            _ => {}
        }
    }
}
///
/// 移動元升生成
///
/// 1. 手番の先後    phase
/// 2. 移動先升      to
///
/// その升に到達できる駒が居る升を取得☆（＾～＾）
/// TODO 成りの動きも考えたい。升だけではなく、成りの有無☆（＾～＾）
///
pub fn insert_narazu_src_by_sn_ms(
    phase: &Phase,
    to: Square,
    uchu: &Uchu,
    result: &mut HashSet<Square>,
) {
    assert_banjo_ms(to, "Ｉnsert_narazu_src_by_sn_ms");

    // 移動先の筋、段
    let (dx, dy) = sq_to_file_rank(to);

    // 駒種類
    for pt in KMS_ARRAY.iter() {
        // 行先の無いところに駒を進めることの禁止☆（＾～＾）
        let pc = ph_pt_to_pc(&phase, &pt);
        use super::super::entities::teigi::shogi_syugo::Piece::*;
        match pc {
            U0 => {
                // ▼うさぎ　は１、２段目には進めない
                if dy < DAN_3 {
                    continue;
                }
            }
            S0 | H0 => {
                // ▼しし、▼ひよこ　は１段目には進めない
                if dy < DAN_2 {
                    continue;
                }
            }
            U1 => {
                // △うさぎ　は８、９段目には進めない
                if DAN_7 < dy {
                    continue;
                }
            }
            S1 | H1 => {
                // △しし、△ひよこ　は９段目には進めない
                if DAN_8 < dy {
                    continue;
                }
            }
            _ => {}
        }

        let kms_num = kms_to_num(&pt);
        for i_dir in 0..KM_UGOKI_LN {
            // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            let p_kmdir: &KmDir;
            if match_sn(&Phase::First, &phase) {
                p_kmdir = &KM_UGOKI.back[kms_num][i_dir];
            // g_writeln(&format!("get_src_by_sn_ms 先手なら pt={} kms_num={} p_kmdir={}",
            //     pt, kms_num, p_kmdir
            // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
                p_kmdir = &_kmdir;
                // g_writeln(&format!("get_src_by_sn_ms 後手なら pt={} kms_num={} p_kmdir={}",
                //     pt, kms_num, p_kmdir
                // ));
            }

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use super::super::entities::teigi::shogi_syugo::KmDir::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        for i_east in 1..9 {
                            if dx + i_east < SUJI_10 {
                                let from = suji_dan_to_ms(dx + i_east, dy);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 東
                        if dx + 1 < SUJI_10 {
                            let from = suji_dan_to_ms(dx + 1, dy);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        for i_ne in 1..9 {
                            if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                                let from = suji_dan_to_ms(dx + i_ne, dy + i_ne);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北東
                        if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                            let from = suji_dan_to_ms(dx + 1, dy + 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 北北東
                NNE => {
                    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                        let from = suji_dan_to_ms(dx + 1, dy + 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        for i_south in 1..9 {
                            if dy + i_south < DAN_10 {
                                let from = suji_dan_to_ms(dx, dy + i_south);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北
                        if dy + 1 < DAN_10 {
                            let from = suji_dan_to_ms(dx, dy + 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            // g_writeln(&format!("get_src_by_sn_ms 北 from={} sn_ms=>{} kms_ms={} match_sn={} match_pt={}",
                            //     from, sn_ms, kms_ms, match_sn( &sn_ms, &phase ), match_pt( &kms_ms, &pt )
                            // ));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 北北西
                NNW => {
                    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                        let from = suji_dan_to_ms(dx - 1, dy + 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        for i_se in 1..9 {
                            if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                                let from = suji_dan_to_ms(dx - i_se, dy + i_se);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北西
                        if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                            let from = suji_dan_to_ms(dx - 1, dy + 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        for i_east in 1..9 {
                            if SUJI_0 < dx - i_east {
                                let from = suji_dan_to_ms(dx - i_east, dy);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 西
                        if SUJI_0 < dx - 1 {
                            let from = suji_dan_to_ms(dx - 1, dy);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        for i_ne in 1..9 {
                            if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                                let from = suji_dan_to_ms(dx - i_ne, dy - i_ne);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南西
                        if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                            let from = suji_dan_to_ms(dx - 1, dy - 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 南南西
                SSW => {
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                        let from = suji_dan_to_ms(dx - 1, dy - 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        for i_north in 1..9 {
                            if DAN_0 < dy - i_north {
                                let from = suji_dan_to_ms(dx, dy - i_north);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南
                        if DAN_0 < dy - 1 {
                            let from = suji_dan_to_ms(dx, dy - 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            // g_writeln(&format!("get_src_by_sn_ms 南 pt={} kms_num={} from={} sn_ms=>{} kms_ms={} match_sn={} match_pt={}",
                            //     pt, kms_num, from, sn_ms, kms_ms, match_sn( &sn_ms, &phase ), match_pt( &kms_ms, &pt )
                            // ));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 南南東
                SSE => {
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                        let from = suji_dan_to_ms(dx + 1, dy - 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        for i_nw in 1..9 {
                            if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                                let from = suji_dan_to_ms(dx + i_nw, dy - i_nw);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南東
                        if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                            let from = suji_dan_to_ms(dx + 1, dy - 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                Owari => break,
            }
        }
    }
}
///
/// 移動元升生成（成る前）
///
pub fn insert_narumae_src_by_sn_ms(
    phase: &Phase,
    to: Square,
    uchu: &Uchu,
    result: &mut HashSet<Square>,
) {
    assert_banjo_ms(to, "Ｉnsert_narumae_src_by_sn_ms");

    // 移動先の筋、段
    let (dx, dy) = sq_to_file_rank(to);

    // 駒種類
    for pt in KMS_ARRAY.iter() {
        let pc_from = ph_pt_to_pc(&phase, &pt);

        // +--------------------+
        // | 移動前は非成駒か？ |
        // +--------------------+
        let from_pt = pc_to_pt(&pc_from);
        if pt_is_pro(&from_pt) {
            continue; // 成る前に成駒なら、成りの動きをしていない
        }

        let prokm_src = pc_to_pro_pc(&pc_from);
        match prokm_src {
            Piece::Kara => {
                continue;
            } // 成れない駒は、成る動きを考えなくていいぜ☆（＾～＾）
            _ => {} // 成れる駒は、成る前の駒の動きも調べる
        }

        // 成り駒に、行先の無いところは無いぜ☆

        let kms_num = kms_to_num(&pt);
        for i_dir in 0..KM_UGOKI_LN {
            // 指定の駒種類の、全ての逆向きに動ける方向
            let _kmdir;
            let p_kmdir: &KmDir;
            if match_sn(&Phase::First, &phase) {
                p_kmdir = &KM_UGOKI.back[kms_num][i_dir];
            // g_writeln(&format!("get_src_by_sn_ms 先手なら pt={} kms_num={} p_kmdir={}",
            //     pt, kms_num, p_kmdir
            // ));
            } else {
                _kmdir = hanten_kmdir_joge(&KM_UGOKI.back[kms_num][i_dir]);
                p_kmdir = &_kmdir;
                // g_writeln(&format!("get_src_by_sn_ms 後手なら pt={} kms_num={} p_kmdir={}",
                //     pt, kms_num, p_kmdir
                // ));
            }

            // 指定升を開始地点に、離れていくように調べていく
            // 指定先後の駒があれば追加
            use super::super::entities::teigi::shogi_syugo::KmDir::*;
            match *p_kmdir {
                // 東
                E(b) => {
                    if b {
                        // 長東
                        for i_east in 1..9 {
                            if dx + i_east < SUJI_10 {
                                let from = suji_dan_to_ms(dx + i_east, dy);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 東
                        if dx + 1 < SUJI_10 {
                            let from = suji_dan_to_ms(dx + 1, dy);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 北東
                NE(b) => {
                    if b {
                        // 長北東
                        for i_ne in 1..9 {
                            if dx + i_ne < SUJI_10 && dy + i_ne < DAN_10 {
                                let from = suji_dan_to_ms(dx + i_ne, dy + i_ne);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北東
                        if dx + 1 < SUJI_10 && dy + 1 < DAN_10 {
                            let from = suji_dan_to_ms(dx + 1, dy + 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 北北東
                NNE => {
                    if dx + 1 < SUJI_10 && dy + 2 < DAN_10 {
                        let from = suji_dan_to_ms(dx + 1, dy + 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 北
                N(b) => {
                    if b {
                        // 長北
                        for i_south in 1..9 {
                            if dy + i_south < DAN_10 {
                                let from = suji_dan_to_ms(dx, dy + i_south);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北
                        if dy + 1 < DAN_10 {
                            let from = suji_dan_to_ms(dx, dy + 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            // g_writeln(&format!("get_src_by_sn_ms 北 from={} sn_ms=>{} kms_ms={} match_sn={} match_pt={}",
                            //     from, sn_ms, kms_ms, match_sn( &sn_ms, &phase ), match_pt( &kms_ms, &pt )
                            // ));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 北北西
                NNW => {
                    if SUJI_0 < dx - 1 && dy + 2 < DAN_10 {
                        let from = suji_dan_to_ms(dx - 1, dy + 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 北西
                NW(b) => {
                    if b {
                        // 長北西
                        for i_se in 1..9 {
                            if SUJI_0 < dx - i_se && dy + i_se < DAN_10 {
                                let from = suji_dan_to_ms(dx - i_se, dy + i_se);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 北西
                        if dx - 1 > SUJI_0 && DAN_10 > dy + 1 {
                            let from = suji_dan_to_ms(dx - 1, dy + 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 西
                W(b) => {
                    if b {
                        // 長西
                        for i_east in 1..9 {
                            if SUJI_0 < dx - i_east {
                                let from = suji_dan_to_ms(dx - i_east, dy);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 西
                        if SUJI_0 < dx - 1 {
                            let from = suji_dan_to_ms(dx - 1, dy);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 南西
                SW(b) => {
                    if b {
                        // 長南西
                        for i_ne in 1..9 {
                            if SUJI_0 < dx - i_ne && DAN_0 < dy - i_ne {
                                let from = suji_dan_to_ms(dx - i_ne, dy - i_ne);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南西
                        if SUJI_0 < dx - 1 && DAN_0 < dy - 1 {
                            let from = suji_dan_to_ms(dx - 1, dy - 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 南南西
                SSW => {
                    if SUJI_0 < dx - 1 && DAN_0 < dy - 2 {
                        let from = suji_dan_to_ms(dx - 1, dy - 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 南
                S(b) => {
                    if b {
                        // 長南
                        for i_north in 1..9 {
                            if DAN_0 < dy - i_north {
                                let from = suji_dan_to_ms(dx, dy - i_north);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南
                        if DAN_0 < dy - 1 {
                            let from = suji_dan_to_ms(dx, dy - 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            // g_writeln(&format!("get_src_by_sn_ms 南 pt={} kms_num={} from={} sn_ms=>{} kms_ms={} match_sn={} match_pt={}",
                            //     pt, kms_num, from, sn_ms, kms_ms, match_sn( &sn_ms, &phase ), match_pt( &kms_ms, &pt )
                            // ));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                // 南南東
                SSE => {
                    if dx + 1 < SUJI_10 && DAN_0 < dy - 2 {
                        let from = suji_dan_to_ms(dx + 1, dy - 2);
                        let sn_ms = uchu.ky.get_sn_by_ms(from);
                        let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                        if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                            result.insert(from);
                        }
                    }
                }
                // 南東
                SE(b) => {
                    if b {
                        // 長南東
                        for i_nw in 1..9 {
                            if dx + i_nw < SUJI_10 && DAN_0 < dy - i_nw {
                                let from = suji_dan_to_ms(dx + i_nw, dy - i_nw);
                                let sn_ms = uchu.ky.get_sn_by_ms(from);
                                let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                                if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                    result.insert(from);
                                }
                                if !match_sn(&sn_ms, &Phase::Owari) {
                                    break;
                                }
                            }
                        }
                    } else {
                        // 南東
                        if dx + 1 < SUJI_10 && DAN_0 < dy - 1 {
                            let from = suji_dan_to_ms(dx + 1, dy - 1);
                            let sn_ms = uchu.ky.get_sn_by_ms(from);
                            let kms_ms = pc_to_pt(&uchu.ky.get_pc_by_sq(from));
                            if match_sn(&sn_ms, &phase) && match_pt(&kms_ms, &pt) {
                                result.insert(from);
                            }
                        }
                    }
                }
                Owari => break,
            }
        }
    }
}

/*
 * 合い駒スペースを算出
 *
 * phase_atk  : 攻めている方の先後
 * sq_atk  : 攻め駒の居る升
 * sq_tgt  : 狙われている駒の居る升
 * kms_atk : 攻め駒の駒種類
 */
/*
#[allow(dead_code)]
pub fn get_ms_vec_as_aigoma(
    phase_atk:&Phase,
    sq_atk:Square,
    sq_tgt:Square,
    kms_atk:&PieceType
    )->Vec<Square> {
    let vec = Vec::new();

    use teigi::shogi_syugo::PieceType::*;
    match *kms_atk {
        K => {
            // 北方向
            // 西方向
            // 南方向
            // 東方向
        },
        Z => {
            // 北東方向
            // 北西方向
            // 南西方向
            // 南東方向
        },
        S => {
            if match_sn(&Phase::First, &phase_atk) {
                // 北方向

            } else {
                // 南方向

            }
        },
        PK => {
            // 北方向
            // 西方向
            // 南方向
            // 東方向
        },
        PZ => {
            // 北東方向
            // 北西方向
            // 南西方向
            // 南東方向
        },
        _ => {}
    }
    vec
}
*/
