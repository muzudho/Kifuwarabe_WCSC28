//!
//! 値チェック
//!
use super::super::teigi::shogi_syugo::*;

/// 打はテストできない
pub fn assert_onboard_sq(sq: Square, hint: &str) {
    debug_assert!(
        (10 < sq && sq < 20)
            || (20 < sq && sq < 30)
            || (30 < sq && sq < 40)
            || (40 < sq && sq < 50)
            || (50 < sq && sq < 60)
            || (60 < sq && sq < 70)
            || (70 < sq && sq < 80)
            || (80 < sq && sq < 90)
            || (90 < sq && sq < 100),
        "sq={} {}",
        sq,
        hint
    );
}
