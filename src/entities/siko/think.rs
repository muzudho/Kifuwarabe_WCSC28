//!
//! 深い考えだぜ☆（＾～＾）
//!

extern crate rand;
use rand::Rng;
use std::collections::HashSet;

use super::super::super::genmove::sasite_seisei::*;
use super::super::super::genmove::sasite_sentaku::*;
use super::super::jotai::mediators::med_kikisu::*;
use super::super::jotai::uchu::*;
use super::super::kasetu;
use super::super::siko::visions::vision_tree::*;
use super::super::tusin::usi::*;

///
/// 現局面での最善手を返すぜ☆（*＾～＾*）
///
pub fn think(mut uchu: &mut Uchu) -> Sasite {
    // TODO 王手放置漏れ回避　を最優先させたいぜ☆（＾～＾）

    // +----------------------+
    // | 王手放置漏れ回避対策 |
    // +----------------------+

    // 相手の利き升調べ（自殺手防止のため）
    read_kikisu(&mut uchu);

    g_writeln(&format!("info test is_s={}", kasetu::atamakin::is_s(&uchu)));

    // let を 先に記述した変数の方が、後に記述した変数より　寿命が長いので注意☆（＾～＾）
    let mut some_moves_hashset: HashSet<u64> = HashSet::new();
    insert_potential_move(&uchu, &mut some_moves_hashset);
    // g_writeln("テスト ポテンシャルムーブ.");
    // use consoles::visuals::dumps::*;
    // hyoji_ss_hashset( &some_moves_hashset );

    filtering_ss_except_oute(&mut some_moves_hashset, &mut uchu);

    // 現局面を見て、ビジョンを作り直せだぜ☆（＾～＾）
    &uchu.remake_visions();
    insert_rakkansuji(&mut uchu);
    // TODO 楽観筋はまだ使ってない☆（＾～＾）

    // 楽観王手の一覧はできているはず。

    // FIXME 負けてても、千日手は除くぜ☆（＾～＾）ただし、千日手を取り除くと手がなくなる場合は取り除かないぜ☆（＾～＾）
    filtering_ss_except_sennitite(&mut some_moves_hashset, &mut uchu);

    // 自殺手は省くぜ☆（＾～＾）
    filtering_ss_except_jisatusyu(&mut some_moves_hashset, uchu);

    if some_moves_hashset.len() == 0 {
        // 投了
        return Sasite::new();
    } else {
        let index = rand::thread_rng().gen_range(0, some_moves_hashset.len());
        let mut i = 0;
        for ss_hash in some_moves_hashset {
            if i == index {
                //let result : Sasite = ss.clone();
                let ss = Sasite::from_hash(ss_hash);
                g_writeln(&format!("info solution:{}.", ss));
                return ss;
            }
            i += 1;
        }

        // 投了
        return Sasite::new();
    }
}
