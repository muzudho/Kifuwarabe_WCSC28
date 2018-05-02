#![allow(dead_code)]
/**
 * いろんな値、定義☆（＾～＾）
 */
use std::collections::HashSet;

use jotai::uchu::*;
use teigi::conv::*;
use std::fmt;

/********
 * 手目 *
 ********/
/**
 * 手目数。何手目まで指せるか。
 * 棋譜を残す配列のサイズでもある。
 * 大会ルールが 256手として、終端子として投了を１個入れておけるようにする。
 */
pub const TEME_LN :usize = 257;
/**
 * 同一局面何回で千日手
 */
pub const SENNTITE_NUM :i8 = 4;


/********
 * 先後 *
 ********/
pub const SN_LN : usize = 3;
/**
 * 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
 * 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
 */
#[derive(Clone)]
pub enum Sengo{
    Sen,
    Go,
    // 空升の先後を調べようとした場合等
    Owari,
}
pub const SN_SEN : usize = 0;
pub const SN_GO : usize = 1;
/**
 * 後手（上手）を盤の下側に持ってきて表示するのを基本とするぜ☆（＾～＾）
 */
impl fmt::Display for Sengo{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use teigi::shogi_syugo::Sengo::*;
        match *self{
            Sen => { write!(f,"▼")},
            Go => { write!(f,"△")},
            Owari => { write!(f,"×")},
        }
    }
}
/**
 * 先後の一致比較
 */
pub fn match_sn(a:&Sengo, b:&Sengo)->bool{
    sn_to_num(a) == sn_to_num(b)
}

pub const SN_ARRAY_LN : usize = 2;
pub const SN_ARRAY : [Sengo;SN_ARRAY_LN] = [
    Sengo::Sen,
    Sengo::Go,
];


/************
 * 自分相手 *
 ************/
// 先後とは別物
pub const JIAI_LN : usize = 3;
/**
 * 先後。単純にプレイヤー１を先手、プレイヤー２を後手とする。
 * 駒落ち戦での通称　上手／下手　の場合、上手は先手、下手は後手とする。
 */
#[derive(Clone)]
pub enum Jiai{
    Ji,
    Ai,
    Owari,
}
pub const JIAI_JI : usize = 0;
pub const JIAI_AI : usize = 1;
impl fmt::Display for Jiai {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        use teigi::shogi_syugo::Jiai::*;
        match *self{
            Ji => { write!(f,"自")},
            Ai => { write!(f,"相")},
            Owari => { write!(f,"×")},
        }
    }
}
/**
 * 一致比較
 */
pub fn match_jiai(a:&Jiai, b:&Jiai)->bool{
    jiai_to_num(a) == jiai_to_num(b)
}

pub const JIAI_ARRAY_LN : usize = 2;
pub const JIAI_ARRAY : [Jiai;JIAI_ARRAY_LN] = [
    Jiai::Ji,
    Jiai::Ai,
];


/******************
 * 盤、升、筋、段 *
 ******************/
/*
 * 盤の符号は、後手番から見る
 *
 *
 * 19  29  39  49  59  69  79  89  99
 * 18  28  38  48  58  68  78  88  98
 * 17  27  37  47  57  67  77  87  97
 * 16  26  36  46  56  66  76  86  96
 * 15  25  35  45  55  65  75  85  95
 * 14  24  34  44  54  64  74  84  94
 * 13  23  33  43  53  63  73  83  93
 * 12  22  32  42  52  62  72  82  92
 * 11  21  31  41  51  61  71  81  91
 *
 */
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MIN :usize = 11;
/**
 * 盤を回転するのに使うぜ☆（＾～＾）
 */
pub const BAN_MAX :usize = 99;
/**
 * 盤のヨコ幅、タテ幅。
 * 筋と段は x,y とは逆方向なので、幅も左端、下端を指す。
 */
//pub const BAN_W :i8 = 9;
//pub const BAN_H :i8 = 9;
pub const BAN_SIZE :usize = 100;
// 1辺の長さ
//pub const BAN_LINE :usize = 10;
/**
 * 筋、段は 1 から始まる、という明示。
 * 増減はよく使うので u8 ではなく i8 にした。
 */
pub const SUJI_0 :i8 = 0;
pub const SUJI_1 :i8 = 1;
pub const SUJI_9 :i8 = 9;
pub const SUJI_10 :i8 = 10;
pub const DAN_0 :i8 = 0;
pub const DAN_1 :i8 = 1;
pub const DAN_2 :i8 = 2;
pub const DAN_3 :i8 = 3;
pub const DAN_4 :i8 = 4;
pub const DAN_5 :i8 = 5;
pub const DAN_6 :i8 = 6;
pub const DAN_7 :i8 = 7;
pub const DAN_8 :i8 = 8;//うさぎの打てる段の上限
pub const DAN_9 :i8 = 9;
pub const DAN_10 :i8 = 10;
/**
 * 升番号 0～99。
 * 10の位を筋、1の位を段とする。0筋、0段は未使用（番兵として使用）
 * 該当なしの場合 0 を使う
 */
 #[allow(non_camel_case_types)]
pub type umasu = usize;
/**
 * 升の検索等で、該当なしの場合
 */
pub const MASU_0 : umasu = 0;

/**
 * 指し手。打の場合のsrc
 */
pub const SS_SRC_DA : umasu = 0;



/******
 * 陣 *
 ******/

/**
 * 先手陣
 */
pub struct SenteJin{
}
impl SenteJin{
    pub fn to_elm()->Vec<umasu>{
        vec![
            91,81,71,61,51,41,31,21,11,
            92,82,72,62,52,42,32,22,12,
            93,83,73,63,53,43,33,23,13,
        ]
    }
}

/**
 * 後手陣
 */
pub struct GoteJin{
}
impl GoteJin{
    pub fn to_elm()->Vec<umasu>{
        vec![
            91,81,71,61,51,41,31,21,11,
            92,82,72,62,52,42,32,22,12,
            93,83,73,63,53,43,33,23,13,
        ]
    }
}


/******
 * 駒 *
 ******/
// 先後付き駒

// 持ち駒の駒のうち、最大の枚数は歩の 18。
pub const MG_MAX : usize = 18;
pub const KM_LN : usize = 30;
/**
 * 先後付きの駒と空白
 */
#[derive(Copy, Clone)]
pub enum Koma{
    // ▼らいおん
    R0,
    // ▼きりん
    K0,
    // ▼ぞう
    Z0,
    // ▼いぬ
    I0,
    // ▼ねこ
    N0,
    // ▼うさぎ
    U0,
    // ▼いのしし
    S0,
    // ▼ひよこ
    H0,
    // ▼ぱわーあっぷきりん
    PK0,
    // ▼ぱわーあっぷぞう
    PZ0,
    // ▼ぱわーあっぷねこ
    PN0,
    // ▼ぱわーあっぷうさぎ
    PU0,
    // ▼ぱわーあっぷいのしし
    PS0,
    // ▼ぱわーあっぷひよこ
    PH0,
    // △ライオン
    R1,
    // △キリン
    K1,
    // △ゾウ
    Z1,
    // △イヌ
    I1,
    // △ネコ
    N1,
    // △ウサギ
    U1,
    // △イノシシ
    S1,
    // △ヒヨコ
    H1,
    // △パワーアップキリン
    PK1,
    // △パワーアップゾウ
    PZ1,
    // △パワーアップネコ
    PN1,
    // △パワーアップウサギ
    PU1,
    // △パワーアップイノシシ
    PS1,
    // △パワーアップヒヨコ
    PH1,
    // 空マス
    Kara,
    // 要素数より1小さい数。該当なしや、エラー値用としても兼用する
    Owari
}
impl fmt::Display for Koma{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use teigi::shogi_syugo::Koma::*;
        match *self{
            R0 => { write!(f,"▼ら")},
            K0 => { write!(f,"▼き")},
            Z0 => { write!(f,"▼ぞ")},
            I0 => { write!(f,"▼い")},
            N0 => { write!(f,"▼ね")},
            U0 => { write!(f,"▼う")},
            S0 => { write!(f,"▼し")},
            H0 => { write!(f,"▼ひ")},
            PK0 => { write!(f,"▼PK")},
            PZ0 => { write!(f,"▼PZ")},
            PN0 => { write!(f,"▼PN")},
            PU0 => { write!(f,"▼PU")},
            PS0 => { write!(f,"▼PS")},
            PH0 => { write!(f,"▼PH")},
            R1 => { write!(f,"△ラ")},
            K1 => { write!(f,"△キ")},
            Z1 => { write!(f,"△ゾ")},
            I1 => { write!(f,"△イ")},
            N1 => { write!(f,"△ネ")},
            U1 => { write!(f,"△ウ")},
            S1 => { write!(f,"△シ")},
            H1 => { write!(f,"△ヒ")},
            PK1 => { write!(f,"△pk")},
            PZ1 => { write!(f,"△pz")},
            PN1 => { write!(f,"△pn")},
            PU1 => { write!(f,"△pu")},
            PS1 => { write!(f,"△ps")},
            PH1 => { write!(f,"△ph")},
            Kara => { write!(f,"　　")},
            Owari => { write!(f,"××")},
        }
    }
}
/**
 * 駒の一致比較
 */
pub fn match_km(a:&Koma, b:&Koma)->bool{
    km_to_num(a) == km_to_num(b)
}

pub const KM_ARRAY_HALF_LN : usize = 14;
pub const KM_ARRAY_LN : usize = 28;
pub const KM_ARRAY : [Koma;KM_ARRAY_LN] = [
    Koma::R0,// らいおん
    Koma::K0,// きりん
    Koma::Z0,// ぞう
    Koma::I0,// いぬ
    Koma::N0,// ねこ
    Koma::U0,// うさぎ
    Koma::S0,// いのしし
    Koma::H0,// ひよこ
    Koma::PK0,// ぱわーあっぷきりん
    Koma::PZ0,// ぱわーあっぷぞう
    Koma::PN0,// ぱわーあっぷねこ
    Koma::PU0,// ぱわーあっぷうさぎ
    Koma::PS0,// ぱわーあっぷいのしし
    Koma::PH0,// ぱわーあっぷひよこ
    Koma::R1,// らいおん
    Koma::K1,// きりん
    Koma::Z1,// ぞう
    Koma::I1,// いぬ
    Koma::N1,// ねこ
    Koma::U1,// うさぎ
    Koma::S1,// いのしし
    Koma::H1,// ひよこ
    Koma::PK1,// ぱわーあっぷきりん
    Koma::PZ1,// ぱわーあっぷぞう
    Koma::PN1,// ぱわーあっぷねこ
    Koma::PU1,// ぱわーあっぷうさぎ
    Koma::PS1,// ぱわーあっぷいのしし
    Koma::PH1,// ぱわーあっぷひよこ
];
pub const SN_KM_ARRAY : [[Koma;KM_ARRAY_HALF_LN];SN_LN] = [
    [
        Koma::R0,// らいおん
        Koma::K0,// きりん
        Koma::Z0,// ぞう
        Koma::I0,// いぬ
        Koma::N0,// ねこ
        Koma::U0,// うさぎ
        Koma::S0,// いのしし
        Koma::H0,// ひよこ
        Koma::PK0,// ぱわーあっぷきりん
        Koma::PZ0,// ぱわーあっぷぞう
        Koma::PN0,// ぱわーあっぷねこ
        Koma::PU0,// ぱわーあっぷうさぎ
        Koma::PS0,// ぱわーあっぷいのしし
        Koma::PH0,// ぱわーあっぷひよこ
    ],
    [
        Koma::R1,// らいおん
        Koma::K1,// きりん
        Koma::Z1,// ぞう
        Koma::I1,// いぬ
        Koma::N1,// ねこ
        Koma::U1,// うさぎ
        Koma::S1,// いのしし
        Koma::H1,// ひよこ
        Koma::PK1,// ぱわーあっぷきりん
        Koma::PZ1,// ぱわーあっぷぞう
        Koma::PN1,// ぱわーあっぷねこ
        Koma::PU1,// ぱわーあっぷうさぎ
        Koma::PS1,// ぱわーあっぷいのしし
        Koma::PH1,// ぱわーあっぷひよこ
    ],
    [
        Koma::Owari,// らいおん
        Koma::Owari,// きりん
        Koma::Owari,// ぞう
        Koma::Owari,// いぬ
        Koma::Owari,// ねこ
        Koma::Owari,// うさぎ
        Koma::Owari,// いのしし
        Koma::Owari,// ひよこ
        Koma::Owari,// ぱわーあっぷきりん
        Koma::Owari,// ぱわーあっぷぞう
        Koma::Owari,// ぱわーあっぷねこ
        Koma::Owari,// ぱわーあっぷうさぎ
        Koma::Owari,// ぱわーあっぷいのしし
        Koma::Owari,// ぱわーあっぷひよこ
    ],
];
/**
 * 駒集合
 */
pub struct KmSyugo {
    num_syugo : HashSet<usize>,
}
impl KmSyugo {
    /**
     * 全ての元を含む
     */
    pub fn new_all() -> KmSyugo {
        let mut num_syugo1 : HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            num_syugo1.insert( km_to_num(km) );
        }
        let km_syugo = KmSyugo {
            num_syugo : num_syugo1,
        };
        km_syugo
    }
    /**
     * 自分相手
     */
    pub fn new_jiai( &self, jiai:&Jiai, uchu:&Uchu ) -> KmSyugo {
        let sn0 = uchu.get_teban(&jiai);
        let mut num_syugo1 : HashSet<usize> = HashSet::new();
        for km in KM_ARRAY.iter() {
            let (sn1,_kms) = km_to_sn_kms( km );
            if match_sn( &sn0, &sn1 ) {
                num_syugo1.insert( km_to_num(km) );
            }
        }
        let km_syugo = KmSyugo {
            num_syugo : num_syugo1,
        };
        km_syugo
    }
    pub fn remove( &mut self, km:&Koma ) {
        self.num_syugo.remove( &km_to_num(km) );
    }
}

/**********
 * 駒種類 *
 **********/
pub const KMS_LN : usize = 16;
// 駒の動ける方向数、終端子込み
pub const KM_UGOKI_LN : usize = 9;
// 先後なしの駒と空白
#[derive(Copy, Clone)]
pub enum KmSyurui{
    // らいおん
    R,
    // きりん
    K,
    // ぞう
    Z,
    // いぬ
    I,
    // ねこ
    N,
    // うさぎ
    U,
    // いのしし
    S,
    // ひよこ
    H,
    // ぱわーあっぷきりん
    PK,
    // ぱわーあっぷぞう
    PZ,
    // ぱわーあっぷねこ
    PN,
    // ぱわーあっぷうさぎ
    PU,
    // ぱわーあっぷいのしし
    PS,
    // ぱわーあっぷひよこ
    PH,
    // 空マス
    Kara,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari
}
impl fmt::Display for KmSyurui{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use teigi::shogi_syugo::KmSyurui::*;
        match *self{
            R => { write!(f,"ら")},
            K => { write!(f,"き")},
            Z => { write!(f,"ぞ")},
            I => { write!(f,"い")},
            N => { write!(f,"ね")},
            U => { write!(f,"う")},
            S => { write!(f,"い")},
            H => { write!(f,"ひ")},
            PK => { write!(f,"PK")},
            PZ => { write!(f,"PZ")},
            PN => { write!(f,"PN")},
            PU => { write!(f,"PU")},
            PS => { write!(f,"PS")},
            PH => { write!(f,"PH")},
            Kara => { write!(f,"　")},
            Owari => { write!(f,"×")},
        }
    }
}
/**
 * 駒種類の一致比較
 */
pub fn match_kms(a:&KmSyurui, b:&KmSyurui)->bool{
    kms_to_num(a) == kms_to_num(b)
}

// 駒種類数
pub const KMS_ARRAY_LN : usize = 14;
// 駒種類
pub const KMS_ARRAY : [KmSyurui;KMS_ARRAY_LN] = [
    KmSyurui::R,// らいおん
    KmSyurui::K,// きりん
    KmSyurui::Z,// ぞう
    KmSyurui::I,// いぬ
    KmSyurui::N,// ねこ
    KmSyurui::U,// うさぎ
    KmSyurui::S,// いのしし
    KmSyurui::H,// ひよこ
    KmSyurui::PK,// ぱわーあっぷきりん
    KmSyurui::PZ,// ぱわーあっぷぞう
    KmSyurui::PN,// ぱわーあっぷねこ
    KmSyurui::PU,// ぱわーあっぷうさぎ
    KmSyurui::PS,// ぱわーあっぷいのしし
    KmSyurui::PH,// ぱわーあっぷひよこ
];

// 非成 駒種類数
pub const KMS_NPRO_ARRAY_LN : usize = 8;
// 非成 駒種類
pub const KMS_NPRO_ARRAY : [KmSyurui;KMS_NPRO_ARRAY_LN] = [
    KmSyurui::R,// らいおん
    KmSyurui::K,// きりん
    KmSyurui::Z,// ぞう
    KmSyurui::I,// いぬ
    KmSyurui::N,// ねこ
    KmSyurui::U,// うさぎ
    KmSyurui::S,// いのしし
    KmSyurui::H,// ひよこ
];

// 成 駒種類数
pub const KMS_PRO_ARRAY_LN : usize = 6;
// 成 駒種類
pub const KMS_PRO_ARRAY : [KmSyurui;KMS_PRO_ARRAY_LN] = [
    KmSyurui::PK,// ぱわーあっぷきりん
    KmSyurui::PZ,// ぱわーあっぷぞう
    KmSyurui::PN,// ぱわーあっぷねこ
    KmSyurui::PU,// ぱわーあっぷうさぎ
    KmSyurui::PS,// ぱわーあっぷいのしし
    KmSyurui::PH,// ぱわーあっぷひよこ
];

// 持駒種類数
pub const MGS_ARRAY_LN : usize = 7;
// 持駒種類
pub const MGS_ARRAY : [KmSyurui;MGS_ARRAY_LN] = [
    KmSyurui::K,
    KmSyurui::Z,
    KmSyurui::I,
    KmSyurui::N,
    KmSyurui::U,
    KmSyurui::S,
    KmSyurui::H,
];

/**
 * 駒種類集合
 */
pub struct KmsSyugo {
    num_syugo : HashSet<usize>,
}
impl KmsSyugo {
    /**
     * 全ての元を含む
     */
    pub fn new_all() -> KmsSyugo {
        let mut num_syugo1 : HashSet<usize> = HashSet::new();
        for kms in KMS_ARRAY.iter() {
            num_syugo1.insert( kms_to_num(kms) );
        }
        let kms_syugo = KmsSyugo {
            num_syugo : num_syugo1,
        };
        kms_syugo
    }
    pub fn remove( &mut self, kms:&KmSyurui ) {
        self.num_syugo.remove( &kms_to_num(kms) );
    }
}


/*********
 * 8方向 *
 *********/
// 盤の方向は、後手から見た視点
pub enum Dir8{
    // 東
    E,
    // 北東
    NE,
    // 北
    N,
    // 北西
    NW,
    // 西
    W,
    // 南西
    SW,
    // 南
    S,
    // 南東
    SE,
    // 要素数より1小さい数。エラー値用に使っても可
    Owari
}

/****************
 * 駒の動く方向 *
 ****************/
/**
 * 後手から見た盤を想像すること。筋、段を第一象限と同じ向きに合わせる。
 * 駒が戻る方向10方向。東から反時計回り。boolは長い利きなら真
 */
#[derive(Clone)]
pub enum KmDir{
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
    Owari
}
impl fmt::Display for KmDir{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // 文字列リテラルでないとダメみたいなんで、他に似たようなコードがあるのに、また書くことに☆（＾～＾）
        use teigi::shogi_syugo::KmDir::*;
        match *self{
            E(b)        => if b { write!(f,"長東")  }else{ write!(f,"東")  },
            NE(b)       => if b { write!(f,"長北東")}else{ write!(f,"北東")},
            NNE         => {write!(f,"北北東")},
            N(b)        => if b { write!(f,"長北")  }else{ write!(f,"北")  },
            NNW         => { write!(f,"北北西")},
            NW(b)       => if b { write!(f,"長北西")}else{ write!(f,"北西")},
            W(b)        => if b { write!(f,"長西")  }else{ write!(f,"西")  },
            SW(b)       => if b { write!(f,"長南西")}else{ write!(f,"南西")},
            SSW         => { write!(f,"南南西")},
            S(b)        => if b { write!(f,"長南")  }else{ write!(f,"南")  },
            SSE         => { write!(f,"南南東")},
            SE(b)       => if b { write!(f,"長南東")}else{ write!(f,"南東")},
            Owari       => { write!(f,"×")},
        }
    }
}
/************
 * 駒の動き *
 ************/
// 駒が戻る動き
#[allow(dead_code)]
pub struct KmUgoki{
    // 駒種類ごとに、駒の動きを保持。動ける方向は、駒ごとに可変長配列
    pub back:[[KmDir;KM_UGOKI_LN];KMS_LN]
}
/**
 * 駒が戻る動き。投了図から現局面へ逆向きに指す思想。
 * [駒種類][9]
 *
 * （１）この表は、後手から盤面を見たものを想像する。
 * （２）後手から見て、普通に駒の動きが　登録されている。
 *       先手から見たとき、back （後ろ向きの動き）となる。
 */
pub const KM_UGOKI : KmUgoki = KmUgoki{
    back:[
        // 東,北東,北,北西,西,南西,南南西,南,南南東,南東,終わり
        /*ら  */ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),KmDir::SW(false),KmDir::S(false),KmDir::SE(false),KmDir::Owari],
        /*き  */ [KmDir::E(true ),                            KmDir::N(true ),                            KmDir::W(true ),                 KmDir::S(true ),                 KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぞ  */ [                KmDir::NE(true ),                                      KmDir::NW(true ),                KmDir::SW(true ),                KmDir::SE(true ),KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*い  */ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ね  */ [                KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),                KmDir::SW(false),                KmDir::SE(false),KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*う  */ [                                 KmDir::NNE,                KmDir::NNW                 ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*し  */ [                                            KmDir::N(true )                            ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ひ  */ [                                            KmDir::N(false)                            ,                                                                  KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱき*/ [KmDir::E(true ),KmDir::NE(false),           KmDir::N(true ),           KmDir::NW(false),KmDir::W(true ),KmDir::SW(false),KmDir::S(true ),KmDir::SE(false),KmDir::Owari],
        /*ぱぞ*/ [KmDir::E(false),KmDir::NE(true ),           KmDir::N(false),           KmDir::NW(true ),KmDir::W(false),KmDir::SW(true ),KmDir::S(false),KmDir::SE(true ),KmDir::Owari],
        /*ぱね*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱう*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱし*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*ぱひ*/ [KmDir::E(false),KmDir::NE(false),           KmDir::N(false),           KmDir::NW(false),KmDir::W(false),                 KmDir::S(false),                 KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*空升*/ [                                                                                                                                                          KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
        /*終り*/ [                                                                                                                                                          KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari,KmDir::Owari],
    ]
};

/********
 * 局面 *
 ********/
pub enum KyNums {
    // 現局面
    Current,
    // 初期局面
    Start,
}

/**************
 * 予想の結果 *
 **************/
pub enum DoingResult {
    // 起こった
    Done,
    // 起こらなかった
    None,
    // それ以外
    Owari,
}
