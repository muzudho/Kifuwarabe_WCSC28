#![allow(dead_code)]
//!
//! いろんな値、定義☆（＾～＾）
//!
use std::collections::HashSet;

use super::super::jotai::uchu::*;
use super::super::teigi::conv::*;
use std::fmt;

/// 手目数。何手目まで指せるか。
/// 棋譜を残す配列のサイズでもある。
/// 大会ルールが 256手として、終端子として投了を１個入れておけるようにする。
pub const TEME_LN: usize = 257;
///
/// 同一局面何回で千日手
///
pub const SENNTITE_NUM: i8 = 4;

pub const SN_LN: usize = 3;
///
/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
///
#[derive(Clone)]
pub enum Phase {
    First,
    Second,
    // 空升の先後を調べようとした場合等
    Owari,
}
pub const SN_SEN: usize = 0;
pub const SN_GO: usize = 1;
///
/// 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
///
impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use super::super::teigi::shogi_syugo::Phase::*;
        match *self {
            First => write!(f, "▼"),
            Second => write!(f, "△"),
            Owari => write!(f, "×"),
        }
    }
}
///
/// 先後の一致比較
///
pub fn match_ph(a: &Phase, b: &Phase) -> bool {
    sn_to_num(a) == sn_to_num(b)
}

pub const SN_ARRAY_LN: usize = 2;
pub const SN_ARRAY: [Phase; SN_ARRAY_LN] = [Phase::First, Phase::Second];

/// 先後とは別物
pub const JIAI_LN: usize = 3;

/// 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
/// 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
#[derive(Clone)]
pub enum Jiai {
    Ji,
    Ai,
    Owari,
}
pub const JIAI_JI: usize = 0;
pub const JIAI_AI: usize = 1;
impl fmt::Display for Jiai {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use super::super::teigi::shogi_syugo::Jiai::*;
        match *self {
            Ji => write!(f, "自"),
            Ai => write!(f, "相"),
            Owari => write!(f, "×"),
        }
    }
}

/// 一致比較
pub fn match_jiai(a: &Jiai, b: &Jiai) -> bool {
    jiai_to_num(a) == jiai_to_num(b)
}

pub const JIAI_ARRAY_LN: usize = 2;
pub const JIAI_ARRAY: [Jiai; JIAI_ARRAY_LN] = [Jiai::Ji, Jiai::Ai];

/// 盤の符号は、後手番から見る
///
///
/// 19  29  39  49  59  69  79  89  99
/// 18  28  38  48  58  68  78  88  98
/// 17  27  37  47  57  67  77  87  97
/// 16  26  36  46  56  66  76  86  96
/// 15  25  35  45  55  65  75  85  95
/// 14  24  34  44  54  64  74  84  94
/// 13  23  33  43  53  63  73  83  93
/// 12  22  32  42  52  62  72  82  92
/// 11  21  31  41  51  61  71  81  91
///
///
///
/// 盤を回転するのに使うぜ☆（＾～＾）
pub const BAN_MIN: usize = 11;
///
/// 盤を回転するのに使うぜ☆（＾～＾）
///
pub const BAN_MAX: usize = 99;

//
// 盤のヨコ幅、タテ幅。
// 筋と段は x,y とは逆方向なので、幅も左端、下端を指す。
//
//pub const BAN_W :i8 = 9;
//pub const BAN_H :i8 = 9;

pub const BAN_SIZE: usize = 100;

// 1辺の長さ
//pub const BAN_LINE :usize = 10;

/// 筋、段は 1 から始まる、という明示。
/// 増減はよく使うので u8 ではなく i8 にした。
pub const SUJI_0: i8 = 0;
pub const SUJI_1: i8 = 1;
pub const SUJI_9: i8 = 9;
pub const SUJI_10: i8 = 10;
pub const DAN_0: i8 = 0;
pub const DAN_1: i8 = 1;
pub const DAN_2: i8 = 2;
pub const DAN_3: i8 = 3;
pub const DAN_4: i8 = 4;
pub const DAN_5: i8 = 5;
pub const DAN_6: i8 = 6;
pub const DAN_7: i8 = 7;
pub const DAN_8: i8 = 8; //うさぎの打てる段の上限
pub const DAN_9: i8 = 9;
pub const DAN_10: i8 = 10;
///
/// 升番号 0～99。
/// 10の位を筋、1の位を段とする。0筋、0段は未使用（番兵として使用）
/// 該当なしの場合 0 を使う
///
#[allow(non_camel_case_types)]
pub type Square = usize;
///
/// 升の検索等で、該当なしの場合
///
pub const MASU_0: Square = 0;

///
/// 指し手。打の場合のsrc
///
pub const SS_SRC_DA: Square = 0;

///
/// 先手陣
///
pub struct SenteJin {}
impl SenteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            91, 81, 71, 61, 51, 41, 31, 21, 11, 92, 82, 72, 62, 52, 42, 32, 22, 12, 93, 83, 73, 63,
            53, 43, 33, 23, 13,
        ]
    }
}

///
/// 後手陣
///
pub struct GoteJin {}
impl GoteJin {
    pub fn to_elm() -> Vec<Square> {
        vec![
            91, 81, 71, 61, 51, 41, 31, 21, 11, 92, 82, 72, 62, 52, 42, 32, 22, 12, 93, 83, 73, 63,
            53, 43, 33, 23, 13,
        ]
    }
}

/// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX: usize = 18;
pub const PC_LEN: usize = 30;
///
/// 先後付きの駒と空白
///
#[derive(Copy, Clone)]
pub enum Piece {
    /// ▲玉 King
    K1,
    /// ▲飛 Rook
    R1,
    /// ▲角 Bishop
    B1,
    /// ▲金 Gold
    G1,
    /// ▲銀 Sliver
    S1,
    /// ▲桂 Knight
    N1,
    /// ▲香 Lance
    L1,
    /// ▲歩 Pawn
    P1,
    /// ▲竜 Promoted Rook (Dragon)
    PR1,
    /// ▲馬 Promoted Bishop (Horse)
    PB1,
    /// ▲全 Promoted Silver
    PS1,
    /// ▲圭 Promoted Knight
    PN1,
    /// ▲杏 Promoted Lance
    PL1,
    /// ▲と Promoted Pawn
    PP1,
    /// ▽玉
    K2,
    /// ▽飛
    R2,
    /// ▽角
    B2,
    /// ▽金
    G2,
    /// ▽銀
    S2,
    /// ▽桂
    N2,
    /// ▽香
    L2,
    /// ▽歩
    P2,
    /// ▽竜
    PR2,
    /// ▽馬
    PB2,
    /// ▽全
    PS2,
    /// ▽圭
    PN2,
    /// ▽杏
    PL2,
    /// ▽と
    PP2,
    /// 空マス
    Empty,
    /// 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Owari,
}
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use super::super::teigi::shogi_syugo::Piece::*;
        match *self {
            K1 => write!(f, "▼ら"),
            R1 => write!(f, "▼き"),
            B1 => write!(f, "▼ぞ"),
            G1 => write!(f, "▼い"),
            S1 => write!(f, "▼ね"),
            N1 => write!(f, "▼う"),
            L1 => write!(f, "▼し"),
            P1 => write!(f, "▼ひ"),
            PR1 => write!(f, "▼PK"),
            PB1 => write!(f, "▼PZ"),
            PS1 => write!(f, "▼PN"),
            PN1 => write!(f, "▼PU"),
            PL1 => write!(f, "▼PS"),
            PP1 => write!(f, "▼PH"),
            K2 => write!(f, "△ラ"),
            R2 => write!(f, "△キ"),
            B2 => write!(f, "△ゾ"),
            G2 => write!(f, "△イ"),
            S2 => write!(f, "△ネ"),
            N2 => write!(f, "△ウ"),
            L2 => write!(f, "△シ"),
            P2 => write!(f, "△ヒ"),
            PR2 => write!(f, "△pk"),
            PB2 => write!(f, "△pz"),
            PS2 => write!(f, "△pn"),
            PN2 => write!(f, "△pu"),
            PL2 => write!(f, "△ps"),
            PP2 => write!(f, "△ph"),
            Empty => write!(f, "　　"),
            Owari => write!(f, "××"),
        }
    }
}
///
/// 駒の一致比較
///
pub fn match_pc(a: &Piece, b: &Piece) -> bool {
    pc_to_num(a) == pc_to_num(b)
}

pub const PC_ARRAY_HALF_LEN: usize = 14;
pub const PC_ARRAY_LEN: usize = 28;
pub const PC_ARRAY: [Piece; PC_ARRAY_LEN] = [
    Piece::K1,  // らいおん
    Piece::R1,  // きりん
    Piece::B1,  // ぞう
    Piece::G1,  // いぬ
    Piece::S1,  // ねこ
    Piece::N1,  // うさぎ
    Piece::L1,  // いのしし
    Piece::P1,  // ひよこ
    Piece::PR1, // ぱわーあっぷきりん
    Piece::PB1, // ぱわーあっぷぞう
    Piece::PS1, // ぱわーあっぷねこ
    Piece::PN1, // ぱわーあっぷうさぎ
    Piece::PL1, // ぱわーあっぷいのしし
    Piece::PP1, // ぱわーあっぷひよこ
    Piece::K2,  // らいおん
    Piece::R2,  // きりん
    Piece::B2,  // ぞう
    Piece::G2,  // いぬ
    Piece::S2,  // ねこ
    Piece::N2,  // うさぎ
    Piece::L2,  // いのしし
    Piece::P2,  // ひよこ
    Piece::PR2, // ぱわーあっぷきりん
    Piece::PB2, // ぱわーあっぷぞう
    Piece::PS2, // ぱわーあっぷねこ
    Piece::PN2, // ぱわーあっぷうさぎ
    Piece::PL2, // ぱわーあっぷいのしし
    Piece::PP2, // ぱわーあっぷひよこ
];
pub const PH_PC_ARRAY: [[Piece; PC_ARRAY_HALF_LEN]; SN_LN] = [
    [
        Piece::K1,  // らいおん
        Piece::R1,  // きりん
        Piece::B1,  // ぞう
        Piece::G1,  // いぬ
        Piece::S1,  // ねこ
        Piece::N1,  // うさぎ
        Piece::L1,  // いのしし
        Piece::P1,  // ひよこ
        Piece::PR1, // ぱわーあっぷきりん
        Piece::PB1, // ぱわーあっぷぞう
        Piece::PS1, // ぱわーあっぷねこ
        Piece::PN1, // ぱわーあっぷうさぎ
        Piece::PL1, // ぱわーあっぷいのしし
        Piece::PP1, // ぱわーあっぷひよこ
    ],
    [
        Piece::K2,  // らいおん
        Piece::R2,  // きりん
        Piece::B2,  // ぞう
        Piece::G2,  // いぬ
        Piece::S2,  // ねこ
        Piece::N2,  // うさぎ
        Piece::L2,  // いのしし
        Piece::P2,  // ひよこ
        Piece::PR2, // ぱわーあっぷきりん
        Piece::PB2, // ぱわーあっぷぞう
        Piece::PS2, // ぱわーあっぷねこ
        Piece::PN2, // ぱわーあっぷうさぎ
        Piece::PL2, // ぱわーあっぷいのしし
        Piece::PP2, // ぱわーあっぷひよこ
    ],
    [
        Piece::Owari, // らいおん
        Piece::Owari, // きりん
        Piece::Owari, // ぞう
        Piece::Owari, // いぬ
        Piece::Owari, // ねこ
        Piece::Owari, // うさぎ
        Piece::Owari, // いのしし
        Piece::Owari, // ひよこ
        Piece::Owari, // ぱわーあっぷきりん
        Piece::Owari, // ぱわーあっぷぞう
        Piece::Owari, // ぱわーあっぷねこ
        Piece::Owari, // ぱわーあっぷうさぎ
        Piece::Owari, // ぱわーあっぷいのしし
        Piece::Owari, // ぱわーあっぷひよこ
    ],
];
///
/// 駒集合
///
pub struct PcSyugo {
    num_syugo: HashSet<usize>,
}
impl PcSyugo {
    ///
    /// 全ての元を含む
    ///
    pub fn new_all() -> PcSyugo {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for pc in PC_ARRAY.iter() {
            num_syugo1.insert(pc_to_num(pc));
        }
        let pc_syugo = PcSyugo {
            num_syugo: num_syugo1,
        };
        pc_syugo
    }
    ///
    /// 自分相手
    ///
    pub fn new_jiai(&self, jiai: &Jiai, uchu: &Uchu) -> PcSyugo {
        let sn0 = uchu.get_teban(&jiai);
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for pc in PC_ARRAY.iter() {
            let (sn1, _pt) = pc_to_ph_pt(pc);
            if match_ph(&sn0, &sn1) {
                num_syugo1.insert(pc_to_num(pc));
            }
        }
        let pc_syugo = PcSyugo {
            num_syugo: num_syugo1,
        };
        pc_syugo
    }
    pub fn remove(&mut self, pc: &Piece) {
        self.num_syugo.remove(&pc_to_num(pc));
    }
}

///
/// 駒種類
///
pub const PT_LEN: usize = 16;
/// 駒の動ける方向数、終端子込み
pub const PC_UGOKI_LEN: usize = 9;
/// 先後なしの駒と空白
#[derive(Copy, Clone)]
pub enum PieceType {
    // 玉 King
    K,
    // 飛車 Rook
    R,
    // 角 Bishop
    B,
    // 金 Gold
    G,
    // 銀 Silver
    S,
    // 桂 Knight
    N,
    // 香 Lance
    L,
    // 歩 Pawn
    P,
    // 竜 Promoted Rook (Dragon)
    PR,
    // 馬 Promoted Bishop (Horse)
    PB,
    // 全 Promoted Silver
    PS,
    // 圭 Promoted Knight
    PN,
    // 杏 Promoted Lance
    PL,
    // と Promoted Pawn
    PP,
    // 空マス
    Empty,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari,
}
impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use super::super::teigi::shogi_syugo::PieceType::*;
        match *self {
            K => write!(f, "ら"),
            R => write!(f, "き"),
            B => write!(f, "ぞ"),
            G => write!(f, "い"),
            S => write!(f, "ね"),
            N => write!(f, "う"),
            L => write!(f, "い"),
            P => write!(f, "ひ"),
            PR => write!(f, "PR"),
            PB => write!(f, "PB"),
            PS => write!(f, "PS"),
            PN => write!(f, "PN"),
            PL => write!(f, "PL"),
            PP => write!(f, "PP"),
            Empty => write!(f, "　"),
            Owari => write!(f, "×"),
        }
    }
}
///
/// 駒種類の一致比較
///
pub fn match_pt(a: &PieceType, b: &PieceType) -> bool {
    pt_to_num(a) == pt_to_num(b)
}

/// 駒種類数
pub const PT_ARRAY_LEN: usize = 14;
/// 駒種類
pub const PT_ARRAY: [PieceType; PT_ARRAY_LEN] = [
    PieceType::K,  // らいおん
    PieceType::R,  // きりん
    PieceType::B,  // ぞう
    PieceType::G,  // いぬ
    PieceType::S,  // ねこ
    PieceType::N,  // うさぎ
    PieceType::L,  // いのしし
    PieceType::P,  // ひよこ
    PieceType::PR, // ぱわーあっぷきりん
    PieceType::PB, // ぱわーあっぷぞう
    PieceType::PS, // ぱわーあっぷねこ
    PieceType::PN, // ぱわーあっぷうさぎ
    PieceType::PL, // ぱわーあっぷいのしし
    PieceType::PP, // ぱわーあっぷひよこ
];

/// 非成 駒種類数
pub const PT_NPRO_ARRAY_LEN: usize = 8;
/// 非成 駒種類
pub const PT_NPRO_ARRAY: [PieceType; PT_NPRO_ARRAY_LEN] = [
    PieceType::K, // らいおん
    PieceType::R, // きりん
    PieceType::B, // ぞう
    PieceType::G, // いぬ
    PieceType::S, // ねこ
    PieceType::N, // うさぎ
    PieceType::L, // いのしし
    PieceType::P, // ひよこ
];

/// 成 駒種類数
pub const PT_PRO_ARRAY_LEN: usize = 6;
/// 成 駒種類
pub const PT_PRO_ARRAY: [PieceType; PT_PRO_ARRAY_LEN] = [
    PieceType::PR, // ぱわーあっぷきりん
    PieceType::PB, // ぱわーあっぷぞう
    PieceType::PS, // ぱわーあっぷねこ
    PieceType::PN, // ぱわーあっぷうさぎ
    PieceType::PL, // ぱわーあっぷいのしし
    PieceType::PP, // ぱわーあっぷひよこ
];

/// 持駒種類数
pub const MGS_ARRAY_LN: usize = 7;
/// 持駒種類
pub const MGS_ARRAY: [PieceType; MGS_ARRAY_LN] = [
    PieceType::R,
    PieceType::B,
    PieceType::G,
    PieceType::S,
    PieceType::N,
    PieceType::L,
    PieceType::P,
];

///
/// 駒種類集合
///
pub struct PtSyugo {
    num_syugo: HashSet<usize>,
}
impl PtSyugo {
    ///
    /// 全ての元を含む
    ///
    pub fn new_all() -> PtSyugo {
        let mut num_syugo1: HashSet<usize> = HashSet::new();
        for pt in PT_ARRAY.iter() {
            num_syugo1.insert(pt_to_num(pt));
        }
        let pt_syugo = PtSyugo {
            num_syugo: num_syugo1,
        };
        pt_syugo
    }
    pub fn remove(&mut self, pt: &PieceType) {
        self.num_syugo.remove(&pt_to_num(pt));
    }
}

///
/// 8方向
///
/// 盤の方向は、後手から見た視点
pub enum Dir8 {
    /// 東
    E,
    /// 北東
    NE,
    /// 北
    N,
    /// 北西
    NW,
    /// 西
    W,
    /// 南西
    SW,
    /// 南
    S,
    /// 南東
    SE,
    /// 要素数より1小さい数。エラー値用に使っても可
    Owari,
}

/// 駒の動く方向
///
/// 後手から見た盤を想像すること。筋、段を第一象限と同じ向きに合わせる。
/// 駒が戻る方向10方向。東から反時計回り。boolは長い利きなら真
#[derive(Clone)]
pub enum PcDir {
    // 東
    E(bool),
    // 北東
    NE(bool),
    // 北北東（桂馬が戻る動き）
    NNE,
    // 北
    N(bool),
    // 北北西（桂馬が戻る動き）
    NNW,
    // 北西
    NW(bool),
    // 西
    W(bool),
    // 南西
    SW(bool),
    // 南南西（桂馬の動き）
    SSW,
    // 南
    S(bool),
    // 南南東（桂馬の動き）
    SSE,
    // 南東
    SE(bool),
    // 要素数より1小さい数。エラー値用に使っても可
    Owari,
}
impl fmt::Display for PcDir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use super::super::teigi::shogi_syugo::PcDir::*;
        match *self {
            E(b) => {
                if b {
                    write!(f, "長東")
                } else {
                    write!(f, "東")
                }
            }
            NE(b) => {
                if b {
                    write!(f, "長北東")
                } else {
                    write!(f, "北東")
                }
            }
            NNE => write!(f, "北北東"),
            N(b) => {
                if b {
                    write!(f, "長北")
                } else {
                    write!(f, "北")
                }
            }
            NNW => write!(f, "北北西"),
            NW(b) => {
                if b {
                    write!(f, "長北西")
                } else {
                    write!(f, "北西")
                }
            }
            W(b) => {
                if b {
                    write!(f, "長西")
                } else {
                    write!(f, "西")
                }
            }
            SW(b) => {
                if b {
                    write!(f, "長南西")
                } else {
                    write!(f, "南西")
                }
            }
            SSW => write!(f, "南南西"),
            S(b) => {
                if b {
                    write!(f, "長南")
                } else {
                    write!(f, "南")
                }
            }
            SSE => write!(f, "南南東"),
            SE(b) => {
                if b {
                    write!(f, "長南東")
                } else {
                    write!(f, "南東")
                }
            }
            Owari => write!(f, "×"),
        }
    }
}
/// 駒が戻る動き
#[allow(dead_code)]
pub struct PcUgoki {
    // 駒種類ごとに、駒の動きを保持。動ける方向は、駒ごとに可変長配列
    pub back: [[PcDir; PC_UGOKI_LEN]; PT_LEN],
}
///
/// 駒が戻る動き。投了図から現局面へ逆向きに指す思想。
/// [駒種類][9]
///
/// （１）この表は、後手から盤面を見たものを想像する。
/// （２）後手から見て、普通に駒の動きが　登録されている。
///       先手から見たとき、back （後ろ向きの動き）となる。
///
pub const PC_UGOKI: PcUgoki = PcUgoki {
    back: [
        // 東,北東,北,北西,西,南西,南南西,南,南南東,南東,終わり
        /*ら  */
        [
            PcDir::E(false),
            PcDir::NE(false),
            PcDir::N(false),
            PcDir::NW(false),
            PcDir::W(false),
            PcDir::SW(false),
            PcDir::S(false),
            PcDir::SE(false),
            PcDir::Owari,
        ],
        /*き  */
        [
            PcDir::E(true),
            PcDir::N(true),
            PcDir::W(true),
            PcDir::S(true),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*ぞ  */
        [
            PcDir::NE(true),
            PcDir::NW(true),
            PcDir::SW(true),
            PcDir::SE(true),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*い  */
        [
            PcDir::E(false),
            PcDir::NE(false),
            PcDir::N(false),
            PcDir::NW(false),
            PcDir::W(false),
            PcDir::S(false),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*ね  */
        [
            PcDir::NE(false),
            PcDir::N(false),
            PcDir::NW(false),
            PcDir::SW(false),
            PcDir::SE(false),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*う  */
        [
            PcDir::NNE,
            PcDir::NNW,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*し  */
        [
            PcDir::N(true),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*ひ  */
        [
            PcDir::N(false),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*ぱき*/
        [
            PcDir::E(true),
            PcDir::NE(false),
            PcDir::N(true),
            PcDir::NW(false),
            PcDir::W(true),
            PcDir::SW(false),
            PcDir::S(true),
            PcDir::SE(false),
            PcDir::Owari,
        ],
        /*ぱぞ*/
        [
            PcDir::E(false),
            PcDir::NE(true),
            PcDir::N(false),
            PcDir::NW(true),
            PcDir::W(false),
            PcDir::SW(true),
            PcDir::S(false),
            PcDir::SE(true),
            PcDir::Owari,
        ],
        /*ぱね*/
        [
            PcDir::E(false),
            PcDir::NE(false),
            PcDir::N(false),
            PcDir::NW(false),
            PcDir::W(false),
            PcDir::S(false),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*ぱう*/
        [
            PcDir::E(false),
            PcDir::NE(false),
            PcDir::N(false),
            PcDir::NW(false),
            PcDir::W(false),
            PcDir::S(false),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*ぱし*/
        [
            PcDir::E(false),
            PcDir::NE(false),
            PcDir::N(false),
            PcDir::NW(false),
            PcDir::W(false),
            PcDir::S(false),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*ぱひ*/
        [
            PcDir::E(false),
            PcDir::NE(false),
            PcDir::N(false),
            PcDir::NW(false),
            PcDir::W(false),
            PcDir::S(false),
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*空升*/
        [
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
        /*終り*/
        [
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
            PcDir::Owari,
        ],
    ],
};

///
/// 局面
///
pub enum KyNums {
    /// 現局面
    Current,
    /// 初期局面
    Start,
}

///
/// 予想の結果
///
pub enum DoingResult {
    /// 起こった
    Done,
    /// 起こらなかった
    None,
    /// それ以外
    Owari,
}
