#![allow(dead_code)]
//!
//! コレクションの内容をダンプ（全部見る）とかだぜ☆（＾～＾）
//!
//! * 升
//! * 駒種類
//!
use super::super::super::jotai::uchu::*;
use super::super::super::teigi::conv::*;
use super::super::super::teigi::shogi_syugo::*;
use super::super::super::tusin::usi::*;
use std::collections::HashSet;

/// 升を表示
pub fn hyoji_ms_hashset(ms_hashset: &HashSet<umasu>) {
    g_writeln(&format!("ms_hashset.len()={}", ms_hashset.len()));
    for ms in ms_hashset {
        match *ms {
            MASU_0 => break,
            _ => g_writeln(&format!("ms({})", ms)),
        }
    }
}

/// 升を表示
pub fn hyoji_ms_vec(ms_vec: &Vec<umasu>) {
    g_writeln(&format!("ms_vec.len()={}", ms_vec.len()));
    for ms in ms_vec {
        match *ms {
            MASU_0 => break,
            _ => g_writeln(&format!("ms({})", ms)),
        }
    }
}

/// 駒種類
pub fn hyoji_kms_hashset(num_kms_hashset: &HashSet<usize>) {
    g_writeln(&format!("num_kms_hashset.len()={}", num_kms_hashset.len()));
    for num_kms in num_kms_hashset {
        g_writeln(&format!("kms({})", num_to_kms(*num_kms)));
    }
}

/// 指し手
pub fn hyoji_ss_hashset(ss_hashset: &HashSet<u64>) {
    g_writeln(&format!("ss_hashset.len()={}", ss_hashset.len()));
    // 辞書順ソート
    let mut vec_ss_str = Vec::new();
    for ss_hash in ss_hashset {
        let ss = Sasite::from_hash(*ss_hash);
        let ss_str = format!("{}", ss);
        vec_ss_str.push(ss_str);
    }
    //vec_ss_str.sort();
    vec_ss_str.sort_by(|a_str, b_str| {
        let a_arr: Vec<_> = a_str.chars().collect();
        let b_arr: Vec<_> = b_str.chars().collect();
        use std::cmp::min;
        let len = min(a_arr.len(), b_arr.len());

        use std::cmp::Ordering;
        for i in 0..len {
            if a_arr[i] < b_arr[i] {
                //g_writeln(&format!( "[{}] a_arr {} < b_arr {}", i, a_arr[i], b_arr[i] ));
                return Ordering::Greater;
            } else if b_arr[i] < a_arr[i] {
                //g_writeln(&format!( "[{}] a_arr {} > b_arr {}", i, a_arr[i], b_arr[i] ));
                return Ordering::Less;
            }
        }

        if a_arr.len() < b_arr.len() {
            //g_writeln(&format!( "len a_arr {} < b_arr {}", a_arr.len(), b_arr.len() ));
            Ordering::Greater
        } else if b_arr.len() < a_arr.len() {
            //g_writeln(&format!( "len a_arr {} > b_arr {}", a_arr.len(), b_arr.len() ));
            Ordering::Less
        } else {
            //g_writeln(&format!( "len a_arr {} = b_arr {}", a_arr.len(), b_arr.len() ));
            Ordering::Equal
        }
    });
    vec_ss_str.reverse();

    let mut i = 0;
    for ss_str in vec_ss_str {
        g_writeln(&format!("[{}] {}", i, ss_str));
        i += 1;
    }
}
