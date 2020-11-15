//!
//! 値チェック
//!
use super::super::teigi::shogi_syugo::*;

/// 打はテストできない
pub fn assert_banjo_ms(ms: umasu, hint: &str) {
    debug_assert!(
        (10 < ms && ms < 20)
            || (20 < ms && ms < 30)
            || (30 < ms && ms < 40)
            || (40 < ms && ms < 50)
            || (50 < ms && ms < 60)
            || (60 < ms && ms < 70)
            || (70 < ms && ms < 80)
            || (80 < ms && ms < 90)
            || (90 < ms && ms < 100),
        "ms={} {}",
        ms,
        hint
    );
}
