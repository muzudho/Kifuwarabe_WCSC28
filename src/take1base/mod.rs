use std::fmt;

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
        use crate::take1base::Piece::*;
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
///
/// 先後付き駒の数値化
///
pub fn pc_to_num(pc: &Piece) -> usize {
    use crate::take1base::Piece::*;
    match *pc {
        K1 => 0,
        R1 => 1,
        B1 => 2,
        G1 => 3,
        S1 => 4,
        N1 => 5,
        L1 => 6,
        P1 => 7,
        PR1 => 8,
        PB1 => 9,
        PS1 => 10,
        PN1 => 11,
        PL1 => 12,
        PP1 => 13,
        K2 => 14,
        R2 => 15,
        B2 => 16,
        G2 => 17,
        S2 => 18,
        N2 => 19,
        L2 => 20,
        P2 => 21,
        PR2 => 22,
        PB2 => 23,
        PS2 => 24,
        PN2 => 25,
        PL2 => 26,
        PP2 => 27,
        Empty => 28,
        Owari => 29,
    }
}
pub fn num_to_pc(pc_num: usize) -> Piece {
    use crate::take1base::Piece::*;
    match pc_num {
        0 => K1,
        1 => R1,
        2 => B1,
        3 => G1,
        4 => S1,
        5 => N1,
        6 => L1,
        7 => P1,
        8 => PR1,
        9 => PB1,
        10 => PS1,
        11 => PN1,
        12 => PL1,
        13 => PP1,
        14 => K2,
        15 => R2,
        16 => B2,
        17 => G2,
        18 => S2,
        19 => N2,
        20 => L2,
        21 => P2,
        22 => PR2,
        23 => PB2,
        24 => PS2,
        25 => PN2,
        26 => PL2,
        27 => PP2,
        28 => Empty,
        _ => Owari,
    }
}
