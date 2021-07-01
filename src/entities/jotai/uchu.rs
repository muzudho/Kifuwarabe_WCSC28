//!
//! 宇宙
//!
//! グローバル変数の作り方が分からないので仕方なく多くの変数を詰め込んでいるぜ☆（＾～＾）
//! 引数で渡しまくれだぜ☆（＾～＾）
//!
extern crate rand;
use crate::entities::logging::{LOGFILE, LOG_ENABLED};
use rand::Rng;

use super::super::jotai::ky::*;
use super::super::jotai::number_board::*;
use super::super::siko::visions::vision_tree::*;
use super::super::teigi;
use super::super::teigi::conv::*;
use super::super::teigi::shogi_syugo::*;
use super::super::tusin::usi::*;
use std::io::Write;

#[allow(dead_code)]
pub fn g_write(s: &str) {
    println!("{}", s);
    if *LOG_ENABLED.lock().unwrap() {
        // write_allメソッドを使うには use std::io::Write; が必要
        match LOGFILE.lock().unwrap().write_all(s.as_bytes()) {
            // 大会向けに、ログ書き込み失敗は出力しないことにする
            Err(_why) => {} //panic!("couldn't write log. : {}",Error::description(&why)),
            Ok(_) => {}
        }
    }
}
#[allow(dead_code)]
pub fn g_writeln(s: &str) {
    println!("{}", s);
    if *LOG_ENABLED.lock().unwrap() {
        match LOGFILE
            .lock()
            .unwrap()
            .write_all(format!("{}\n", s).as_bytes())
        {
            Err(_why) => {}
            Ok(_) => {}
        }
    }
}

/// 局面ハッシュ種
/// ゾブリストハッシュを使って、局面の一致判定をするのに使う☆（＾～＾）
pub struct KyHashSeed {
    // 盤上の駒
    pub pc: [[u64; PC_LEN]; BAN_SIZE],
    // 持ち駒
    pub mg: [[u64; MG_MAX]; PC_LEN],
    // 先後
    pub phase: [u64; SN_LN],
}

/// グローバル変数の作り方が分からないので、
/// ここに全部入れてあるぜ☆（＾～＾）
pub struct Uchu {
    // 対話モード
    pub dialogue_mode: bool,
    // コマンドを溜めておくバッファー
    pub vec_command: Vec<String>,
    // 初期局面
    pub ky0: Kyokumen,
    // 現局面
    pub ky: Kyokumen,
    // 局面ハッシュ種
    pub ky_hash_seed: KyHashSeed,
    // 手目
    pub teme: usize,
    // 棋譜
    //#[derive(Copy, Clone)]
    pub kifu: [Sasite; TEME_LN],
    // 初期局面ハッシュ
    pub ky0_hash: u64,
    // 現局面ハッシュ
    pub ky_hash: [u64; TEME_LN],
    /// 取った駒
    pub cap: [Piece; TEME_LN],
    // 利きの数（先後別）
    pub kiki_su_by_sn: [NumberBoard; SN_LN],
    // 利きの数（先後付き駒別）
    pub kiki_su_by_pc: [NumberBoard; PC_LEN],
    // ビジョン・ツリー
    pub vision_tree_by_sn: [VisionTree; SN_LN],
}

impl Uchu {
    pub fn new() -> Uchu {
        Uchu {
            dialogue_mode: false,
            vec_command: Vec::new(),
            // 初期局面
            ky0: Kyokumen::new(),
            // 現局面
            ky: Kyokumen::new(),
            ky_hash_seed: KyHashSeed {
                // 盤上の駒
                pc: [[0; PC_LEN]; BAN_SIZE],
                // 持ち駒
                mg: [[0; MG_MAX]; PC_LEN],
                // 先後
                phase: [0; SN_LN],
            },
            teme: 0,
            kifu: [
                // 1行16要素で並べるぜ☆（＾～＾）
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(),
                Sasite::new(), //257要素
            ],
            ky0_hash: 0,
            ky_hash: [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, //257要素
            ],
            /// 取った駒
            cap: [
                // 1行16要素で並べるぜ☆（＾～＾）
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara,
                Piece::Kara, //257要素
            ],
            // 利き数（先後別）
            kiki_su_by_sn: [NumberBoard::new(), NumberBoard::new(), NumberBoard::new()],
            // 利き数（駒別なので３０個ある）
            kiki_su_by_pc: [
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
                NumberBoard::new(),
            ],
            vision_tree_by_sn: [VisionTree::new(), VisionTree::new(), VisionTree::new()],
        }
    }
    /// 宇宙誕生
    pub fn big_bang(&mut self) {
        // 局面ハッシュの種をリセット

        // 盤上の駒
        for i_ms in MASU_0..BAN_SIZE {
            for i_pc in 0..PC_LEN {
                // FIXME 18446744073709551615 が含まれないだろ、どうなってるんだぜ☆（＾～＾）！？
                self.ky_hash_seed.pc[i_ms][i_pc] =
                    rand::thread_rng().gen_range(0, 18446744073709551615);
            }
        }
        // 持ち駒
        for i_pc in 0..PC_LEN {
            for i_mg in 0..MG_MAX {
                self.ky_hash_seed.mg[i_pc][i_mg] =
                    rand::thread_rng().gen_range(0, 18446744073709551615);
            }
        }
        // 先後
        for i_sn in 0..SN_LN {
            self.ky_hash_seed.phase[i_sn] = rand::thread_rng().gen_range(0, 18446744073709551615);
        }
    }
    /// 初期局面、現局面ともにクリアーします。
    /// 手目も 0 に戻します。
    pub fn clear_ky01(&mut self) {
        self.ky0.clear();
        self.ky.clear();
        self.set_teme(0);
    }
    /// 初期局面を、現局面にコピーします
    pub fn copy_ky0_to_ky1(&mut self) {
        // 盤上
        for i_ms in 0..BAN_SIZE {
            self.ky.set_pc_by_sq(i_ms, self.ky0.get_pc_by_sq(i_ms));
        }
        // 持ち駒
        for i_mg in 0..PC_LEN {
            self.ky.mg[i_mg] = self.ky0.mg[i_mg];
        }
    }

    /// コマンド・バッファー
    pub fn is_empty_command(&mut self) -> bool {
        self.vec_command.len() == 0
    }
    pub fn push_command(&mut self, line: &String) {
        self.vec_command.push(format!("{}\n", line));
    }
    pub fn pop_command(&mut self) -> String {
        self.vec_command.pop().unwrap()
    }

    /// 初期局面の盤上に駒の位置を設定するもの
    pub fn set_pos0_board_pc(&mut self, suji: i8, dan: i8, pc: Piece) {
        self.ky0.set_pc_by_sq(suji_dan_to_ms(suji, dan), pc);
    }
    pub fn set_ky0_mg(&mut self, pc: Piece, maisu: i8) {
        self.ky0.mg[pc as usize] = maisu;
    }
    pub fn get_jiai_by_pc(&self, pc: &Piece) -> Jiai {
        let (phase, _pt) = pc_to_ph_pt(pc);

        if match_sn(&phase, &self.get_teban(&Jiai::Ji)) {
            Jiai::Ji
        } else {
            Jiai::Ai
        }
    }

    /// 手目
    pub fn set_teme(&mut self, teme: usize) {
        self.teme = teme
    }
    /// 手目
    pub fn get_teme(&self) -> usize {
        self.teme
    }
    /// 手番
    pub fn get_teban(&self, jiai: &Jiai) -> Phase {
        use teigi::shogi_syugo::Jiai::*;
        match *jiai {
            Ji => {
                // 手番
                if self.teme % 2 == 0 {
                    Phase::First
                } else {
                    Phase::Second
                }
            }
            Ai => {
                // 相手番
                if self.teme % 2 == 0 {
                    Phase::Second
                } else {
                    Phase::First
                }
            }
            _ => Phase::Owari,
        }
    }

    /// 棋譜の作成
    pub fn set_sasite_src(&mut self, src: Square) {
        self.kifu[self.teme].src = src
    }
    pub fn set_sasite_dst(&mut self, dst: Square) {
        self.kifu[self.teme].dst = dst
    }
    pub fn set_sasite_pro(&mut self, pro: bool) {
        self.kifu[self.teme].pro = pro
    }
    pub fn set_sasite_drop(&mut self, pt: PieceType) {
        self.kifu[self.teme].drop = pt
    }
    pub fn set_ky0_hash(&mut self, hash: u64) {
        self.ky0_hash = hash
    }
    pub fn set_ky1_hash(&mut self, hash: u64) {
        self.ky_hash[self.teme] = hash
    }
    #[allow(dead_code)]
    pub fn set_cap(&mut self, teme: usize, pc: Piece) {
        self.cap[teme] = pc
    }
    pub fn get_sasite(&self) -> Sasite {
        self.kifu[self.teme]
    }
    #[allow(dead_code)]
    pub fn get_ky_hash(&mut self) -> u64 {
        self.ky_hash[self.teme]
    }
    /// 使い方
    /// let s = uchu.kaku_kifu();
    /// g_writeln( &s );
    pub fn kaku_kifu(&self) -> String {
        let mut s = String::new();
        for teme in 0..self.teme {
            let ss = &self.kifu[teme];
            s.push_str(&format!("[{}] {}", teme, ss));
        }
        s
    }
    pub fn kaku_ky_hash(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("[ini] {:20}\n", &self.ky0_hash));

        for teme in 0..self.teme {
            let hash = &self.ky_hash[teme];
            // 64bitは10進数20桁。改行する
            s.push_str(&format!("[{:3}] {:20}\n", teme, hash));
        }
        s
    }

    /// 自陣
    #[allow(dead_code)]
    pub fn get_ji_jin(&self) -> Vec<Square> {
        if let Phase::First = self.get_teban(&Jiai::Ji) {
            teigi::shogi_syugo::SenteJin::to_elm()
        } else {
            teigi::shogi_syugo::GoteJin::to_elm()
        }
    }
    /// 相手陣
    #[allow(dead_code)]
    pub fn get_aite_jin(&self) -> Vec<Square> {
        if let Phase::First = self.get_teban(&Jiai::Ji) {
            teigi::shogi_syugo::GoteJin::to_elm()
        } else {
            teigi::shogi_syugo::SenteJin::to_elm()
        }
    }

    /// 表示
    ///
    /// 後手から見た盤を表示するぜ☆（＾～＾）
    /// デカルト座標の第一象限と x,y 方向が一致するメリットがあるぜ☆（＾～＾）
    pub fn kaku_ky(&self, num: &KyNums) -> String {
        let ky = match *num {
            KyNums::Current => &self.ky,
            KyNums::Start => &self.ky0,
        };

        // 局面表示
        format!(
            "\
表示 {95}手目 {96} 同一局面{97}回目

           +----+----+----+----+----+----+----+----+----+
        i9 |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|
           +----+----+----+----+----+----+----+----+----+
ひx{87:2}   h8 |{9}|{10}|{11}|{12}|{13}|{14}|{15}|{16}|{17}|   ヒx{94:2}
           +----+----+----+----+----+----+----+----+----+
しx{86:2}   g7 |{18}|{19}|{20}|{21}|{22}|{23}|{24}|{25}|{26}|   シx{93:2}
           +----+----+----+----+----+----+----+----+----+
うx{85:2}   f6 |{27}|{28}|{29}|{30}|{31}|{32}|{33}|{34}|{35}|   ウx{92:2}
           +----+----+----+----+----+----+----+----+----+
ねx{84:2}   e5 |{36}|{37}|{38}|{39}|{40}|{41}|{42}|{43}|{44}|   ネx{91:2}
           +----+----+----+----+----+----+----+----+----+
いx{83:2}   d4 |{45}|{46}|{47}|{48}|{49}|{50}|{51}|{52}|{53}|   イx{90:2}
           +----+----+----+----+----+----+----+----+----+
ぞx{82:2}   c3 |{54}|{55}|{56}|{57}|{58}|{59}|{60}|{61}|{62}|   ゾx{89:2}
           +----+----+----+----+----+----+----+----+----+
きx{81:2}   b2 |{63}|{64}|{65}|{66}|{67}|{68}|{69}|{70}|{71}|   キx{88:2}
           +----+----+----+----+----+----+----+----+----+
▼      a1 |{72}|{73}|{74}|{75}|{76}|{77}|{78}|{79}|{80}|   △
           +----+----+----+----+----+----+----+----+----+
              1    2    3    4    5    6    7    8    9\
",
            ky.get_pc_by_sq(19),
            ky.get_pc_by_sq(29),
            ky.get_pc_by_sq(39),
            ky.get_pc_by_sq(49),
            ky.get_pc_by_sq(59),
            ky.get_pc_by_sq(69),
            ky.get_pc_by_sq(79),
            ky.get_pc_by_sq(89),
            ky.get_pc_by_sq(99),
            ky.get_pc_by_sq(18),
            ky.get_pc_by_sq(28),
            ky.get_pc_by_sq(38),
            ky.get_pc_by_sq(48),
            ky.get_pc_by_sq(58),
            ky.get_pc_by_sq(68),
            ky.get_pc_by_sq(78),
            ky.get_pc_by_sq(88),
            ky.get_pc_by_sq(98),
            ky.get_pc_by_sq(17),
            ky.get_pc_by_sq(27),
            ky.get_pc_by_sq(37),
            ky.get_pc_by_sq(47),
            ky.get_pc_by_sq(57),
            ky.get_pc_by_sq(67),
            ky.get_pc_by_sq(77),
            ky.get_pc_by_sq(87),
            ky.get_pc_by_sq(97),
            ky.get_pc_by_sq(16),
            ky.get_pc_by_sq(26),
            ky.get_pc_by_sq(36),
            ky.get_pc_by_sq(46),
            ky.get_pc_by_sq(56),
            ky.get_pc_by_sq(66),
            ky.get_pc_by_sq(76),
            ky.get_pc_by_sq(86),
            ky.get_pc_by_sq(96),
            ky.get_pc_by_sq(15),
            ky.get_pc_by_sq(25),
            ky.get_pc_by_sq(35),
            ky.get_pc_by_sq(45),
            ky.get_pc_by_sq(55),
            ky.get_pc_by_sq(65),
            ky.get_pc_by_sq(75),
            ky.get_pc_by_sq(85),
            ky.get_pc_by_sq(95),
            ky.get_pc_by_sq(14),
            ky.get_pc_by_sq(24),
            ky.get_pc_by_sq(34),
            ky.get_pc_by_sq(44),
            ky.get_pc_by_sq(54),
            ky.get_pc_by_sq(64),
            ky.get_pc_by_sq(74),
            ky.get_pc_by_sq(84),
            ky.get_pc_by_sq(94),
            ky.get_pc_by_sq(13),
            ky.get_pc_by_sq(23),
            ky.get_pc_by_sq(33),
            ky.get_pc_by_sq(43),
            ky.get_pc_by_sq(53),
            ky.get_pc_by_sq(63),
            ky.get_pc_by_sq(73),
            ky.get_pc_by_sq(83),
            ky.get_pc_by_sq(93),
            ky.get_pc_by_sq(12),
            ky.get_pc_by_sq(22),
            ky.get_pc_by_sq(32),
            ky.get_pc_by_sq(42),
            ky.get_pc_by_sq(52),
            ky.get_pc_by_sq(62),
            ky.get_pc_by_sq(72),
            ky.get_pc_by_sq(82),
            ky.get_pc_by_sq(92),
            ky.get_pc_by_sq(11),
            ky.get_pc_by_sq(21),
            ky.get_pc_by_sq(31),
            ky.get_pc_by_sq(41),
            ky.get_pc_by_sq(51),
            ky.get_pc_by_sq(61),
            ky.get_pc_by_sq(71),
            ky.get_pc_by_sq(81),
            ky.get_pc_by_sq(91),
            //                   ▲き,　                   ▲ぞ,                     ▲い,                     ▲ね,                     ▲う,                     ▲し,                     ▲ひ,
            ky.mg[Piece::K0 as usize],
            ky.mg[Piece::Z0 as usize],
            ky.mg[Piece::I0 as usize],
            ky.mg[Piece::N0 as usize],
            ky.mg[Piece::U0 as usize],
            ky.mg[Piece::S0 as usize],
            ky.mg[Piece::H0 as usize],
            //                   ▽キ,                     ▽ゾ,                     ▽イ,                     ▽ネ,                     ▽ウ,                     ▽シ,                     ▽ヒ,
            ky.mg[Piece::K1 as usize],
            ky.mg[Piece::Z1 as usize],
            ky.mg[Piece::I1 as usize],
            ky.mg[Piece::N1 as usize],
            ky.mg[Piece::U1 as usize],
            ky.mg[Piece::S1 as usize],
            ky.mg[Piece::H1 as usize],
            self.get_teme(),
            self.get_teban(&Jiai::Ji),
            self.count_same_ky()
        )
    }

    /// 表示
    pub fn kaku_number_board(&self, phase: &Phase, pc: &Piece) -> String {
        let nb = match *phase {
            Phase::Owari => &self.kiki_su_by_pc[pc_to_num(&pc)],
            _ => &self.kiki_su_by_sn[sn_to_num(&phase)],
        };

        // 数盤表示
        format!(
            "    +----+----+----+----+----+----+----+----+----+
i9  |{0:4}|{1:4}|{2:4}|{3:4}|{4:4}|{5:4}|{6:4}|{7:4}|{8:4}|
    +----+----+----+----+----+----+----+----+----+
h8  |{9:4}|{10:4}|{11:4}|{12:4}|{13:4}|{14:4}|{15:4}|{16:4}|{17:4}|
    +----+----+----+----+----+----+----+----+----+
g7  |{18:4}|{19:4}|{20:4}|{21:4}|{22:4}|{23:4}|{24:4}|{25:4}|{26:4}|
    +----+----+----+----+----+----+----+----+----+
f6  |{27:4}|{28:4}|{29:4}|{30:4}|{31:4}|{32:4}|{33:4}|{34:4}|{35:4}|
    +----+----+----+----+----+----+----+----+----+
e5  |{36:4}|{37:4}|{38:4}|{39:4}|{40:4}|{41:4}|{42:4}|{43:4}|{44:4}|
    +----+----+----+----+----+----+----+----+----+
d4  |{45:4}|{46:4}|{47:4}|{48:4}|{49:4}|{50:4}|{51:4}|{52:4}|{53:4}|
    +----+----+----+----+----+----+----+----+----+
c3  |{54:4}|{55:4}|{56:4}|{57:4}|{58:4}|{59:4}|{60:4}|{61:4}|{62:4}|
    +----+----+----+----+----+----+----+----+----+
b2  |{63:4}|{64:4}|{65:4}|{66:4}|{67:4}|{68:4}|{69:4}|{70:4}|{71:4}|
    +----+----+----+----+----+----+----+----+----+
a1  |{72:4}|{73:4}|{74:4}|{75:4}|{76:4}|{77:4}|{78:4}|{79:4}|{80:4}|
    +----+----+----+----+----+----+----+----+----+
       1    2    3    4    5    6    7    8    9\
",
            nb.get_su_by_ms(19),
            nb.get_su_by_ms(29),
            nb.get_su_by_ms(39),
            nb.get_su_by_ms(49),
            nb.get_su_by_ms(59),
            nb.get_su_by_ms(69),
            nb.get_su_by_ms(79),
            nb.get_su_by_ms(89),
            nb.get_su_by_ms(99),
            nb.get_su_by_ms(18),
            nb.get_su_by_ms(28),
            nb.get_su_by_ms(38),
            nb.get_su_by_ms(48),
            nb.get_su_by_ms(58),
            nb.get_su_by_ms(68),
            nb.get_su_by_ms(78),
            nb.get_su_by_ms(88),
            nb.get_su_by_ms(98),
            nb.get_su_by_ms(17),
            nb.get_su_by_ms(27),
            nb.get_su_by_ms(37),
            nb.get_su_by_ms(47),
            nb.get_su_by_ms(57),
            nb.get_su_by_ms(67),
            nb.get_su_by_ms(77),
            nb.get_su_by_ms(87),
            nb.get_su_by_ms(97),
            nb.get_su_by_ms(16),
            nb.get_su_by_ms(26),
            nb.get_su_by_ms(36),
            nb.get_su_by_ms(46),
            nb.get_su_by_ms(56),
            nb.get_su_by_ms(66),
            nb.get_su_by_ms(76),
            nb.get_su_by_ms(86),
            nb.get_su_by_ms(96),
            nb.get_su_by_ms(15),
            nb.get_su_by_ms(25),
            nb.get_su_by_ms(35),
            nb.get_su_by_ms(45),
            nb.get_su_by_ms(55),
            nb.get_su_by_ms(65),
            nb.get_su_by_ms(75),
            nb.get_su_by_ms(85),
            nb.get_su_by_ms(95),
            nb.get_su_by_ms(14),
            nb.get_su_by_ms(24),
            nb.get_su_by_ms(34),
            nb.get_su_by_ms(44),
            nb.get_su_by_ms(54),
            nb.get_su_by_ms(64),
            nb.get_su_by_ms(74),
            nb.get_su_by_ms(84),
            nb.get_su_by_ms(94),
            nb.get_su_by_ms(13),
            nb.get_su_by_ms(23),
            nb.get_su_by_ms(33),
            nb.get_su_by_ms(43),
            nb.get_su_by_ms(53),
            nb.get_su_by_ms(63),
            nb.get_su_by_ms(73),
            nb.get_su_by_ms(83),
            nb.get_su_by_ms(93),
            nb.get_su_by_ms(12),
            nb.get_su_by_ms(22),
            nb.get_su_by_ms(32),
            nb.get_su_by_ms(42),
            nb.get_su_by_ms(52),
            nb.get_su_by_ms(62),
            nb.get_su_by_ms(72),
            nb.get_su_by_ms(82),
            nb.get_su_by_ms(92),
            nb.get_su_by_ms(11),
            nb.get_su_by_ms(21),
            nb.get_su_by_ms(31),
            nb.get_su_by_ms(41),
            nb.get_su_by_ms(51),
            nb.get_su_by_ms(61),
            nb.get_su_by_ms(71),
            nb.get_su_by_ms(81),
            nb.get_su_by_ms(91),
        )
    }

    /// 駒の動きを出力
    pub fn print_pcugoki(&self) {
        for pt in PT_ARRAY.iter() {
            g_write(&format!("{} ", pt));
            self.print_pcugoki_dir(&pt);
            g_writeln(""); //改行
        }
    }
    pub fn print_pcugoki_dir(&self, pt: &PieceType) {
        for pc_dir in PC_UGOKI.back[pt_to_num(pt)].iter() {
            match *pc_dir {
                PcDir::Owari => break,
                _ => g_write(&format!("{},", pc_dir)),
            }
        }
    }

    /// 入れた指し手の通り指すぜ☆（＾～＾）
    pub fn do_ss(&mut self, ss: &Sasite) {
        // もう入っているかも知れないが、棋譜に入れる☆
        let phase = self.get_teban(&Jiai::Ji);
        let cap = self.ky.do_sasite(&phase, ss);
        let teme = self.teme;
        self.kifu[teme] = *ss;
        self.set_cap(teme, cap);

        // 局面ハッシュを作り直す
        let ky_hash = self.create_ky1_hash();
        self.set_ky1_hash(ky_hash);

        self.teme += 1;
    }

    pub fn undo_ss(&mut self) -> bool {
        if 0 < self.teme {
            // 棋譜から読取、手目も減る
            self.teme -= 1;
            let phase = self.get_teban(&Jiai::Ji);
            let ss = &self.get_sasite();
            let cap = self.cap[self.teme];
            self.ky.undo_sasite(&phase, &ss, &cap);
            // 棋譜にアンドゥした指し手がまだ残っているが、とりあえず残しとく
            true
        } else {
            false
        }
    }

    pub fn remake_visions(&mut self) {
        for phase in SN_ARRAY.iter() {
            // 全部忘れる☆（＾～＾）
            self.vision_tree_by_sn[sn_to_num(phase)].clear();
        }
    }

    /// 初期局面ハッシュを作り直す
    pub fn create_ky0_hash(&self) -> u64 {
        let mut hash = self.ky0.create_hash(&self);

        // 手番ハッシュ（後手固定）
        hash ^= self.ky_hash_seed.phase[SN_GO];

        hash
    }

    /// 局面ハッシュを作り直す
    pub fn create_ky1_hash(&self) -> u64 {
        let mut hash = self.ky.create_hash(&self);

        // 手番ハッシュ
        use teigi::shogi_syugo::Phase::*;
        match self.get_teban(&Jiai::Ji) {
            First => hash ^= self.ky_hash_seed.phase[SN_SEN],
            Second => hash ^= self.ky_hash_seed.phase[SN_GO],
            _ => {}
        }

        hash
    }

    /// 千日手を調べるために、
    /// 現局面は、同一局面が何回目かを調べるぜ☆（＾～＾）
    pub fn count_same_ky(&self) -> i8 {
        if self.get_teme() < 1 {
            return 0;
        }

        let mut count = 0;
        let last_teme = self.get_teme() - 1;
        let new_teme = self.get_teme();
        // g_writeln( &format!( "Ｃount_same_ky last_teme={} new_teme={}", last_teme ,new_teme ) );
        for i_teme in 0..new_teme {
            let t = last_teme - i_teme;
            // g_writeln( &format!( "i_teme={} t={}", i_teme, t ) );
            if self.ky_hash[t] == self.ky_hash[last_teme] {
                count += 1;
            }
        }

        // 初期局面のハッシュ
        if self.ky0_hash == self.ky_hash[last_teme] {
            count += 1;
        }

        count
    }
}
