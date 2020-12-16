//!
//! 定数
//!

///
/// USIプロトコル表記: 最多合法手５９３手局面
/// https://ameblo.jp/professionalhearts/entry-10001031814.html
///
#[allow(dead_code)]
pub const KY593: &'static str =
  "position sfen R8/2K1S1SSk/4B4/9/9/9/9/9/1L1L1L3 w RBGSNLP3g3n17p 1";
///
/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
///
#[allow(dead_code)]
pub const KY1: &'static str = "position startpos";
///
/// USIプロトコル表記: 飛角落ち初期局面
/// http://www.geocities.jp/shogidokoro/usi.html
///
#[allow(dead_code)]
pub const KY2: &'static str =
  "position sfen lnsgkgsnl/9/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1 moves 5a6b 7g7f 3a3b";

///
/// USIプロトコル表記: 平手初期局面（の盤上の駒配置部分のみ）
///
pub const STARTPOS_LN: usize = 57;
pub const STARTPOS: &'static str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL";
