//!
//! ロギング。
//!

use super::super::config::*;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;

// グローバル定数
//
// 使い方（lazy_static!マクロ）
// ============================
// 定数の値を実行時に決めることができる。
//
// Cargo.toml に１行追記
// > [dependencies]
// > lazy_static = "1.0.0"
//
// main.rs の冒頭あたりに次の２行を記述
// > #[macro_use]
// > extern crate lazy_static;
//
// 「How can I use mutable lazy_static?」
// https://users.rust-lang.org/t/how-can-i-use-mutable-lazy-static/3751/3
lazy_static! {
    /// ログ・ファイルのミューテックス（排他制御）
    pub static ref LOGFILE: Mutex<File> = {
        let engine_file = EngineFile::read();

        // File::createの返り値は`io::Result<File>` なので .unwrap() で中身を取り出す
        Mutex::new(File::create(Path::new(&engine_file.resources.log_file)).unwrap())
    };

    pub static ref LOG_ENABLED: Mutex<bool> = {
        let engine_file = EngineFile::read();
        Mutex::new(engine_file.resources.log_enabled)
    };
}
