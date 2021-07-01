//!
//! コマンド一覧
//!
use super::super::jotai::uchu::*;
use super::super::teigi::shogi_syugo::*;
use crate::take1base::Piece;

/// 利き数表示
pub fn cmd_kikisu(uchu: &Uchu) {
    for pc in PC_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", pc));
        let s = uchu.kaku_number_board(&Phase::Owari, &pc);
        g_writeln(&s);
    }

    for phase in PH_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", phase));
        let s = uchu.kaku_number_board(&phase, &Piece::Owari);
        g_writeln(&s);
    }
}
