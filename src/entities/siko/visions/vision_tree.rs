//!
//! 思考部だぜ☆（＾～＾）
//!
use super::super::super::super::genmove::sasite_element::*;
use super::super::super::jotai::uchu::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::shogi_syugo::*;
use super::super::super::tusin::usi::*;
use std::collections::HashSet;

///
/// 狙いは、この木にぶら下げていくぜ☆（*＾～＾*）
/// 思考で、内容はどんどん変わっていくぜ☆（＾～＾）
///
pub struct VisionTree {
    // 相手玉の位置
    pub ms_ai_r: Square,
    // 相手玉を取る楽観筋
    pub ss_tume_hashset: HashSet<u64>,
}
impl VisionTree {
    pub fn new() -> VisionTree {
        VisionTree {
            ms_ai_r: 0,
            ss_tume_hashset: HashSet::new(),
        }
    }
    pub fn clear(&mut self) {
        self.ss_tume_hashset.clear();
    }
    pub fn set_ai_r(&mut self, sq: Square) {
        self.ms_ai_r = sq;
    }
}

///
/// 楽観筋
///
pub fn insert_rakkansuji(uchu: &mut Uchu) {
    for phase in SN_ARRAY.iter() {
        let ai_sn = hanten_sn(phase);

        // 相手の　らいおん　の位置を覚える
        &uchu.vision_tree_by_sn[sn_to_num(phase)].set_ai_r(uchu.ky.ms_r[sn_to_num(&ai_sn)]);
        // 盤上に相手の　らいおん１枚　しかないと想定して、アタックする手
        let mut mv_src_hashset: HashSet<Square> = HashSet::new();
        //let mut drop_pt_hashset : HashSet<usize> = HashSet::new();

        for kms_dst in KMS_ARRAY.iter() {
            let to_pc = ph_pt_to_pc(&phase, &kms_dst);
            for x in SUJI_1..SUJI_10 {
                // 9..0 みたいに降順に書いても動かない？
                for y in DAN_1..DAN_10 {
                    let to = suji_dan_to_ms(x, y);

                    mv_src_hashset.clear();
                    //drop_pt_hashset.clear();
                    insert_nopromote_from_by_sq_pc(to, &to_pc, &uchu, &mut mv_src_hashset);
                    insert_beforepromote_from_by_sq_pc(to, &to_pc, &uchu, &mut mv_src_hashset);
                    // TODO 王手になるところに打ちたい
                    //insert_drop_pt_by_sq_pc      ( &to, &to_pc, &uchu, &mut drop_pt_hashset );

                    // 盤上
                    for ms_src in mv_src_hashset.iter() {
                        // 成り
                        let pro = &uchu.ky.is_natta(*ms_src, to);

                        let hash_ss = Sasite {
                            src: *ms_src,
                            dst: to,
                            pro: *pro,
                            drop: PieceType::Kara,
                        }
                        .to_hash();
                        &uchu.vision_tree_by_sn[sn_to_num(phase)]
                            .ss_tume_hashset
                            .insert(hash_ss);
                    }

                    /*
                    // 打
                    for kms_da in drop_pt_hashset.iter() {
                        let km_da = ph_pt_to_pc( &phase, &kms_da );
                        let hash_ss = Sasite{
                            src:SS_SRC_DA,
                            dst:to,
                            pro:false,
                            drop:km_da,
                        }.to_hash();
                        &uchu.vision_tree_by_sn[phase].ss_tume_hashset.insert( hash_ss );
                    }
                    */
                }
            }
        }
    } //phase
}
