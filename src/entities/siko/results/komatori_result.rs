//!
//! 結果：駒を取られる手
//!
use std::collections::HashSet;
use std::fmt;

use super::super::super::super::genmove::sasite_seisei::*;
use super::super::super::super::genmove::sasite_sentaku::*;
use super::super::super::consoles::asserts::*;
use super::super::super::jotai::uchu::*;
use super::super::super::meidai::math_meidai::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::geometries::geo_teigi::*;
use super::super::super::teigi::shogi_syugo::*;
use super::super::super::tusin::usi::*;

///
/// 駒取り結果の結果
///
pub enum KomatoriResultResult {
    // 駒は取られる
    Done,
    // アタッカーを除去したことにより、不発
    NoneAttacker,
    // 合い駒をしたことにより、不発
    NoneAigoma,
    // 移動したことにより、不発
    NoneMoved,
    // それ以外
    #[allow(dead_code)]
    Owari,
}

///
/// 結果：駒取り
///
pub struct KomatoriResult {
    // 要因：王手をしてきている駒（１つ）
    pc_attacker: Piece,
    // 要因：アタッカーが居る升
    ms_attacker: Square,
    // 要因：狙われている駒が居る升
    ms_target: Square,
}
impl fmt::Display for KomatoriResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PCTori:{}{}{}{}",
            self.ms_attacker,
            self.pc_attacker,
            if pc_is_long_control(&self.pc_attacker) {
                "-->"
            } else {
                "->"
            },
            self.ms_target
        )
    }
}
impl KomatoriResult {
    #[allow(dead_code)]
    pub fn get_ms_attacker(&self) -> Square {
        self.ms_attacker
    }
    pub fn to_hash(&self) -> u64 {
        let mut hash = 0;
        // 正順で取り出すことを考えて、逆順で押し込む☆（＾～＾）
        hash = push_ms_to_hash(hash, self.ms_target);
        hash = push_ms_to_hash(hash, self.ms_attacker);
        push_pc_to_hash(hash, &self.pc_attacker)
    }
    pub fn from_hash(hash: u64) -> KomatoriResult {
        // 逆順で押し込んであるんで、正順に引き出す☆（＾～＾）
        let (hash, pc_atk) = pop_pc_from_hash(hash);
        let (hash, ms_atk) = pop_ms_from_hash(hash);
        let (_hash, ms_tgt) = pop_ms_from_hash(hash);
        KomatoriResult {
            pc_attacker: pc_atk,
            ms_attacker: ms_atk,
            ms_target: ms_tgt,
        }
    }
    ///
    /// この結果を起こさないのに十分かどうか判断
    ///
    /// 解消十分方法：
    ///     (1) アタッカー升に駒を動かす（取ってしまえば解決）
    ///     (2-1) アタッカーがスライダーの場合
    ///         (2-1-1) 狙われている駒以外の駒（合い駒）を、間の升に置く
    ///     (2-2) アタッカーがスライダーではない場合
    ///         (2-2-1) 狙われている駒を、動かせば解決
    ///
    /// ss : 現局面での、駒の動き手の１つ
    pub fn get_result(&self, ss: &Sasite) -> KomatoriResultResult {
        // (1)
        if self.ms_attacker == ss.dst {
            return KomatoriResultResult::NoneAttacker;
        }

        // (2-1)
        if pc_is_long_control(&self.pc_attacker) {
            assert_banjo_ms(ss.dst, "(205b2)Ｇet_result");
            assert_banjo_ms(self.ms_attacker, "(205b3)Ｇet_result");
            assert_banjo_ms(self.ms_target, "(205b4)Ｇet_result");

            let p_dst = ms_to_p(ss.dst);
            let p_atk = ms_to_p(self.ms_attacker);
            let p_tgt = ms_to_p(self.ms_target);

            // 合い駒判定
            if
            // これから動かす駒は、狙われている駒ではないとする
            ss.src != self.ms_target
                // あるいは打か
                || ss.src == SS_SRC_DA
            {
                // 利きの線分上に、駒を置いたか？
                if intersect_point_on_line_segment(&p_dst, &p_atk, &p_tgt) {
                    // 合い駒を置いて、指定の駒取りを不成功にした
                    return KomatoriResultResult::NoneAigoma;
                }
            } else {
                // 狙われている駒を動かす場合

                assert_banjo_ms(ss.src, "(205b1)Ｇet_result");
                let p_src = ms_to_p(ss.src);

                // スライダー駒との角度
                let argangle4a = get_argangle4_p_p(&p_atk, &p_tgt);
                // これから動かす駒の、逃げた先と、いた場所との角度
                let argangle4b = get_argangle4_p_p(&p_dst, &p_src);

                // スライダーのいる筋の上で動いても、逃げたことにはならないぜ☆（＾～＾）
                match match_argangle4(&argangle4a, &argangle4b) {
                    MatchingResult::Unmatched => {
                        g_writeln(&format!("info ss={} evaluated in slider.", ss));
                        // スライダーから逃げても、ひよこの利きに飛び込むことはあるが……☆
                        return KomatoriResultResult::NoneMoved;
                    }
                    _ => {
                        g_writeln(&format!("info ss={} in slider attack.", ss));
                    }
                }
            }
        } else {
            // (3-2) 狙われている駒を、とりあえず動かす
            if self.ms_target == ss.src {
                return KomatoriResultResult::NoneMoved;
            }
        }

        // TODO 逃げた先の自殺手判定

        // 駒が取られてしまう場合
        KomatoriResultResult::Done
    }
}

///
/// 王手という原因を作っている関係を、（確率的洗いざらい）調べるぜ☆（＾～＾）
///
/// phase        : 駒を「動かす」方はどっちだぜ☆（＾～＾）
/// ms_target : 取りたい駒がいる升
///
/// return u64 : KomatoriResult のハッシュ
///
pub fn lookup_banjo_catch(uchu: &Uchu, phase: &Phase, ms_target: Square) -> HashSet<u64> {
    assert_banjo_ms(
        ms_target,
        &format!(
            "(119)Ｌookup_banjo_catch phase={} ms_target={}",
            phase, ms_target
        ),
    );

    let mut hash = HashSet::new();

    if ms_target == MASU_0 {
        return hash;
    }

    let mut ss_hashset = HashSet::new();

    for to_pt in PT_ARRAY.iter() {
        // 移動した後の相手の駒
        let to_pc = ph_pt_to_pc(&phase, to_pt);
        //let to_pc = ph_pt_to_pc( &phase, rnd_pt() );
        // 指定マスに移動できるか
        // 打は除く

        ss_hashset.clear();
        insert_move_by_sq_pc_on_board(&uchu, ms_target, &to_pc, &mut ss_hashset);

        // g_writeln( &format!("テスト lookup_banjo_catch insert_move_by_sq_pc_on_board to_pt={}.",to_pt) );
        // use consoles::visuals::dumps::*;
        // hyoji_ss_hashset( &ss_hashset );

        let ss = choice_1ss_by_hashset(&ss_hashset);
        if ss.exists() {
            assert_banjo_ms(
                ss.src,
                &format!(
                    "(123)Ｌookup_banjo_catch ss.src /  ms_target={} to_pc={} ss={}",
                    ms_target, to_pc, ss
                ),
            );

            let oute_result = KomatoriResult {
                pc_attacker: to_pc,
                ms_attacker: ss.src, // FIXME 打だと 0 になるのでは
                ms_target: ms_target,
            };

            // 重複がいっぱい
            hash.insert(oute_result.to_hash());
        }
    }
    hash
}
