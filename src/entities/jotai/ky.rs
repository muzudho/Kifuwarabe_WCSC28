//!
//! 局面
//!
//! 後手（上手）から見た盤にすると、
//! 筋と段の方向は　数学のデカルト座標の第一象限のＸ軸、Ｙ軸方向と一致する☆（＾～＾）
//!
//! プログラム上に違いは無いが、ソースコードを読むときは　後手（上手）から
//! 盤を想像すること☆（＾～＾）！
//!

use super::super::jotai::uchu::*;
use super::super::teigi::conv::*;
use super::super::teigi::shogi_syugo::Piece::*;
use super::super::teigi::shogi_syugo::*;
use super::super::tusin::usi::*;

/// 局面
pub struct Kyokumen {
    /// 10の位を筋、1の位を段とする。
    /// 0筋、0段は未使用
    ban: [Piece; BAN_SIZE],
    /// 持ち駒数。持ち駒に使える、成らずの駒の部分だけ使用。
    /// 増減させたいので、u8 ではなく i8。
    pub mg: [i8; PC_LEN],
    /// らいおんの位置
    /// [先後]
    pub ms_r: [Square; SN_LN],
}
impl Kyokumen {
    pub fn new() -> Kyokumen {
        Kyokumen {
            // 盤上
            ban: [
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
                Empty, Empty, Empty, Empty,
            ],
            // 持ち駒数
            mg: [
                // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
                0, 0,
            ],
            ms_r: [0, 0, 0],
        }
    }
    pub fn clear(&mut self) {
        // use super::super::teigi::shogi_syugo::Piece::Empty;
        self.ban = [
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty, Empty,
        ];
        self.mg = [
            // ▲ら,▲き,▲ぞ,▲い,▲ね,▲う,▲し,▲ひ,▲ぱき,▲ぱぞ,▲ぱね,▲ぱう,▲ぱし,▲ぱひ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            // ▽ラ,▽キ,▽ゾ,▽イ,▽ネ,▽ウ,▽シ,▽ヒ,▽パキ,▽パゾ,▽パネ,▽パウ,▽パシ,▽パピ,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 空マス, 終わり,
            0, 0,
        ];
    }
    /// 歩が置いてあるか確認
    pub fn exists_fu_by_sn_suji(&self, phase: &Phase, suji: i8) -> bool {
        for dan in DAN_1..DAN_10 {
            let sq = file_rank_to_sq(suji, dan);
            let pc = self.get_pc_by_sq(sq);
            let (ph_pc, pt) = pc_to_ph_pt(&pc);
            if match_ph(&ph_pc, phase) && match_pt(&pt, &PieceType::P) {
                return true;
            }
        }
        false
    }
    /// 升で指定して駒を取る
    pub fn get_pc_by_sq(&self, sq: Square) -> Piece {
        self.ban[sq]
    }
    /// 升で指定して駒を置く
    pub fn set_pc_by_sq(&mut self, sq: Square, pc: Piece) {
        self.ban[sq] = pc;
        use super::super::teigi::shogi_syugo::Phase::*;
        match pc {
            Piece::R0 => self.ms_r[First as usize] = sq,
            Piece::R1 => self.ms_r[Second as usize] = sq,
            _ => {}
        }
    }
    /// 持ち駒の枚数を加算
    pub fn add_mg(&mut self, mg: Piece, maisu: i8) {
        self.mg[pc_to_num(&mg)] += maisu;
    }
    pub fn get_mg(&self, mg: &Piece) -> i8 {
        self.mg[pc_to_num(mg)]
    }

    /// 指し手の通りに、盤上の駒配置を動かすぜ☆（＾～＾）
    /// 手目のカウントが増えたりはしないぜ☆（＾～＾）
    ///
    /// return : 取った駒
    pub fn do_sasite(&mut self, phase: &Phase, ss: &Sasite) -> Piece {
        // 動かす駒
        let pc;
        // 取った駒
        let cap;

        // 打かどうか
        if ss.src == SS_SRC_DA {
            pc = ph_pt_to_pc(&phase, &ss.drop);
            // 自分の持ち駒を減らす
            self.add_mg(pc, -1);
        } else {
            // 打で無ければ、元の升の駒を消す。
            if ss.pro {
                // 成りなら
                pc = pc_to_pro_pc(&self.get_pc_by_sq(ss.src));
            } else {
                pc = self.get_pc_by_sq(ss.src);
            }
            self.set_pc_by_sq(ss.src, Piece::Empty);
        }

        // 移動先升に駒があるかどうか
        if let Piece::Empty = self.get_pc_by_sq(ss.dst) {
            cap = Piece::Empty;
        } else {
            // 移動先升の駒を盤上から消し、自分の持ち駒に増やす
            cap = self.get_pc_by_sq(ss.dst);
            let mg = pc_to_hand(cap);
            self.add_mg(mg, 1);
        }

        // 移動先升に駒を置く
        self.set_pc_by_sq(ss.dst, pc);

        cap
    }

    /// 指し手の　進む戻る　を逆さにして、盤上の駒配置を動かすぜ☆（＾～＾）
    /// 手目のカウントが増えたりはしないぜ☆（＾～＾）
    pub fn undo_sasite(&mut self, phase: &Phase, ss: &Sasite, cap: &Piece) {
        // 移動先の駒
        let pc;

        // 打かどうか
        if ss.src == SS_SRC_DA {
            pc = ph_pt_to_pc(phase, &ss.drop);
            // 自分の持ち駒を増やす
            //let mg = pc_to_hand(pc);
            //self.add_mg(mg,1);
            self.add_mg(pc, 1);
        } else {
            // 打で無ければ
            if ss.pro {
                // 成ったなら、成る前へ
                pc = pro_pc_to_pc(&self.get_pc_by_sq(ss.dst));
            } else {
                pc = self.get_pc_by_sq(ss.dst);
            }
        }

        // 移動先の駒を、取った駒（あるいは空）に戻す
        self.set_pc_by_sq(ss.dst, *cap);
        match *cap {
            Piece::Empty => {}
            _ => {
                // 自分の持ち駒を減らす
                let mg = pc_to_hand(*cap);
                self.add_mg(mg, -1);
            }
        }

        // 移動元升に、動かした駒を置く
        self.set_pc_by_sq(ss.src, pc);
    }

    /// 指定の升に駒があれば真
    pub fn exists_pc(&self, sq: Square) -> bool {
        !match_pc(&self.get_pc_by_sq(sq), &Piece::Empty)
    }

    /// 指定の升に指定の駒があれば真
    pub fn has_sq_pc(&self, sq: Square, pc: &Piece) -> bool {
        match_pc(&self.get_pc_by_sq(sq), pc)
    }

    /// 指定の升にある駒の先後、または空升
    pub fn get_sn_by_ms(&self, sq: Square) -> Phase {
        pc_to_ph(&self.get_pc_by_sq(sq))
    }

    /// 移動先と移動元を比較し、違う駒があれば、成ったと判定するぜ☆（＾～＾）
    pub fn is_natta(&self, ms_src: Square, to: Square) -> bool {
        let pc_from = &self.get_pc_by_sq(ms_src);
        let from_pt = pc_to_pt(&pc_from);
        let to_pc = &self.get_pc_by_sq(to);
        let to_pt = pc_to_pt(&to_pc);
        // 移動先の駒が成り駒で、 移動元の駒が不成駒なら、成る
        let pro_dst = pt_is_pro(&to_pt);
        let pro_src = pt_is_pro(&from_pt);

        // 成り
        pro_dst && !pro_src
    }

    /// 局面ハッシュを作り直す
    pub fn create_hash(&self, uchu: &Uchu) -> u64 {
        let mut hash: u64 = 0;

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            let pc = self.get_pc_by_sq(i_ms as Square);
            let num_pc = pc_to_num(&pc);
            hash ^= uchu.ky_hash_seed.pc[i_ms][num_pc];
        }

        // 持ち駒ハッシュ
        for i_pc in 0..PC_ARRAY_LEN {
            let pc = PC_ARRAY[i_pc];
            let num_pc = pc_to_num(&pc);

            let maisu = self.get_mg(&pc);
            debug_assert!(
                -1 < maisu && maisu <= MG_MAX as i8,
                "持ち駒 {} の枚数 {} <= {}",
                pc,
                maisu,
                MG_MAX
            );

            hash ^= uchu.ky_hash_seed.mg[num_pc][maisu as usize];
        }

        // 手番ハッシュ はここでは算出しないぜ☆（＾～＾）

        hash
    }
}
