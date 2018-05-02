/**
 * 盤上いろいろ☆（＾～＾）
 */

use jotai::uchu::*;
use teigi::conv::*;
use teigi::shogi_syugo::*;

pub fn is_ji_km_by_ms( ms:umasu, uchu:&Uchu ) -> bool {
    let km = uchu.ky.get_km_by_ms( ms );
    let (sn,_kms) = km_to_sn_kms( &km );
    match_sn( &sn, &uchu.get_teban(&Jiai::Ji) )
}

// TODO
pub fn is_ai_kiki_by_ms( _ms:umasu, _uchu:&Uchu ) -> bool {
    false
}
