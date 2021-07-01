//!
//! 指し手選択
//!

extern crate rand;
use rand::Rng;
use std::collections::HashSet;

use super::super::entities::consoles::asserts::*;
use super::super::entities::jotai::uchu::*;
use super::super::entities::siko::results::komatori_result::*;
use super::super::entities::teigi::conv::*;
use super::super::entities::teigi::shogi_syugo::*;
use super::super::entities::tusin::usi::*;
use super::super::genmove::sasite_element::*;

pub fn choice_1moveex_by_hashset(move_hashset: &HashSet<u64>) -> MoveEx {
    let index = if move_hashset.len() == 0 {
        0
    } else {
        rand::thread_rng().gen_range(0, move_hashset.len())
    };
    let mut i = 0;
    let mut choice_move_hash = 0;
    for move_hash in move_hashset.iter() {
        if i == index {
            choice_move_hash = *move_hash;
            break;
        }
        i += 1;
    }
    MoveEx::from_hash(choice_move_hash)
}

///
/// 王が取られる局面を除く手を選ぶぜ☆（＾～＾）
///
pub fn filtering_move_except_oute(move_hashset_input: &mut HashSet<u64>, uchu: &mut Uchu) {
    // 自玉の位置
    let ms_r = uchu.get_ms_r(&Person::Friend);
    g_writeln(&format!("info string My raion {}.", ms_r));

    // 王手の一覧を取得
    let komatori_result_hashset: HashSet<u64> =
        lookup_banjo_catch(uchu, &uchu.get_teban(&Person::Opponent), ms_r);
    if 0 < komatori_result_hashset.len() {
        // 王手されていれば

        // 表示
        g_writeln(&format!(
            "info string My raion is {} OUTED.",
            komatori_result_hashset.len()
        ));
        for komatori_result_hash0 in komatori_result_hashset.iter() {
            let komatori_result = KomatoriResult::from_hash(*komatori_result_hash0);
            // どんな王手か、出力
            g_writeln(&format!("info OUTE: {}.", komatori_result));
        }

        let mut move_hashset_pickup: HashSet<u64> = HashSet::new();

        // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
        'idea: for hash_potential_move in move_hashset_input.iter() {
            let potential_moveex = MoveEx::from_hash(*hash_potential_move);
            for komatori_result_hash in komatori_result_hashset.iter() {
                let komatori_result = KomatoriResult::from_hash(*komatori_result_hash);

                assert_onboard_sq(potential_moveex.dst, "(206)Ｓearch_gohoshu_hash");
                match komatori_result.get_result(&potential_moveex) {
                    KomatoriResultResult::NoneAttacker
                    | KomatoriResultResult::NoneAigoma
                    | KomatoriResultResult::NoneMoved => {
                        // 駒取りが起こらないものだけが解決
                    }
                    _ => {
                        // 解決しないのが１つでもあれば、次のアイデアへ☆（＾～＾）
                        continue 'idea;
                    }
                }
            }

            // 王手を回避している指し手
            move_hashset_pickup.insert(*hash_potential_move);
        }

        // 振り替え
        move_hashset_input.clear();
        for hash_move in move_hashset_pickup.iter() {
            move_hashset_input.insert(*hash_move);
        }
    } else {
        // 王手されていなければ
        g_writeln(&format!("info string My raion is not outed."));
    }
}

///
/// 王手されていれば、王手を解除しろだぜ☆（＾～＾）
/// 千日手には喜んで飛び込めだぜ☆（＾▽＾）ｗｗｗ
///
pub fn filtering_move_except_suicide(move_hashset_input: &mut HashSet<u64>, uchu: &mut Uchu) {
    // 残すのはここに退避する☆（＾～＾）
    let mut move_hashset_pickup: HashSet<u64> = HashSet::new();

    // 自玉の位置
    let ms_r = uchu.ky.ms_r[sn_to_num(&uchu.get_teban(&Person::Friend))];

    // 王手回避カードを発行する
    // TODO 王手が２か所から掛かっていたら、全部回避しないといけない☆

    // 指せる手から、王手が消えている手だけ、選び抜くぜ☆（＾～＾）
    'idea: for hash_potential_move in move_hashset_input.iter() {
        let potential_moveex = MoveEx::from_hash(*hash_potential_move);

        // その手を指してみる
        uchu.do_moveex(&potential_moveex);
        // // 現局面表示
        // let s1 = &uchu.kaku_ky( &PosNums::Current );
        // g_writeln( &s1 );

        // 狙われている方の玉の位置
        let ms_r_new = if potential_moveex.src == ms_r {
            potential_moveex.dst // 狙われていた方の玉が動いた先
        } else {
            ms_r // 動いていない、狙われていた方の玉の居場所
        };

        // 利きの再計算
        // 有り得る移動元が入る☆（＾～＾）
        let mut attackers: HashSet<Square> = HashSet::new();
        insert_narazu_src_by_sn_ms(
            &uchu.get_teban(&Person::Friend), // 指定の升に駒を動かそうとしている手番
            ms_r_new,                         // 指定の升
            &uchu,
            &mut attackers,
        );
        insert_narumae_src_by_sn_ms(
            &uchu.get_teban(&Person::Friend), // 指定の升に駒を動かそうとしている手番
            ms_r_new,                         // 指定の升
            &uchu,
            &mut attackers,
        );

        // 玉が利きに飛び込んでいるか？
        let jisatusyu = 0 < attackers.len();
        g_writeln(&format!(
            "info {} evaluated => {} attackers. offence={}->{}",
            potential_moveex,
            attackers.len(),
            uchu.get_teban(&Person::Friend),
            ms_r_new
        ));
        for sq_atk in attackers.iter() {
            g_writeln(&format!("info sq_atk={}.", sq_atk));
        }

        // 手を戻す
        uchu.undo_moveex();
        // // 現局面表示
        // let s2 = &uchu.kaku_ky( &PosNums::Current );
        // g_writeln( &s2 );

        if jisatusyu {
            continue 'idea;
        }

        g_writeln(&format!("info SOLUTED moveex={}.", potential_moveex));
        // 問題を全て解決していれば、入れる
        move_hashset_pickup.insert(potential_moveex.to_hash());
    }
    g_writeln(&format!("info {} solutions.", move_hashset_pickup.len()));

    // 空っぽにする
    move_hashset_input.clear();
    // 振り替える
    for hash_move in move_hashset_pickup.iter() {
        move_hashset_input.insert(*hash_move);
    }
}

///
/// 千日手の指し手を取り除いた集合を作るぜ☆（＾～＾）
///
/// ただし、千日手を取り除くと手がない場合は、千日手を選ぶぜ☆（＾～＾）
///
pub fn filtering_move_except_sennitite(move_hashset_input: &mut HashSet<u64>, uchu: &mut Uchu) {
    let mut move_hashset_pickup = HashSet::new();

    // 指せる手から、千日手が消えている手だけ選んで、集合を作るぜ☆（＾～＾）
    'idea: for hash_potential_move in move_hashset_input.iter() {
        let moveex = MoveEx::from_hash(*hash_potential_move);
        //move_hashset.insert( *hash_potential_move );

        // その手を指してみる
        uchu.do_moveex(&moveex);
        // 現局面表示
        // let s1 = &uchu.kaku_ky( &PosNums::Current );
        // g_writeln( &s1 );

        // 千日手かどうかを判定する☆（＾～＾）
        if uchu.count_same_ky() < SENNTITE_NUM {
            move_hashset_pickup.insert(*hash_potential_move);
        } else {
            // 千日手
        }

        // 手を戻す FIXME: 打った象が戻ってない？
        uchu.undo_moveex();
        // 現局面表示
        // let s2 = &uchu.kaku_ky( &PosNums::Current );
        // g_writeln( &s2 );
    }

    // ただし、千日手を取り除くと手がない場合は、千日手を選ぶぜ☆（＾～＾）
    if 0 == move_hashset_pickup.len() {
        return;
    }

    // 振り替え
    move_hashset_input.clear();
    for hash_move in move_hashset_pickup.iter() {
        move_hashset_input.insert(*hash_move);
    }
}
