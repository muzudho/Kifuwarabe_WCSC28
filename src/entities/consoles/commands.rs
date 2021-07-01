//!
//! コマンド一覧
//!
use super::super::jotai::uchu::*;
use super::super::teigi::shogi_syugo::*;

/// 利き数表示
pub fn cmd_kikisu(uchu: &Uchu) {
    for pc in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", pc));
        let s = uchu.kaku_number_board(&Phase::Owari, &pc);
        g_writeln(&s);
    }

    for sn in SN_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", sn));
        let s = uchu.kaku_number_board(&sn, &Piece::Owari);
        g_writeln(&s);
    }
}
