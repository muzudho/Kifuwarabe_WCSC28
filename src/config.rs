//! 設定
use serde::{Deserialize, Serialize};

/// ログ
pub const LOG_FILE_PATH: &'static str = "kifuwarabe_WCSC28.log";
pub const LOG_ENABLE: bool = true; //false;

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineFile {
    pub engine: Engine,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    pub name: String,
    pub author: String,
}
