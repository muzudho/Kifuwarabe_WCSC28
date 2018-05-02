/**
 * コマンド一覧
 */

use teigi::shogi_syugo::*;
use jotai::uchu::*;

/**
 * 利き数表示
 */
pub fn cmd_kikisu(uchu:&Uchu){
    for km in KM_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", km));
        let s = uchu.kaku_number_board( &Sengo::Owari, &km );
        g_writeln( &s );
    }

    for sn in SN_ARRAY.iter() {
        g_writeln(&format!("利き数：{}", sn));
        let s = uchu.kaku_number_board( &sn, &Koma::Owari );
        g_writeln( &s );        
    }
}