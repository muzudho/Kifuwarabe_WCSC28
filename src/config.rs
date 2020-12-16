//!
//! 設定
//!

use serde::{Deserialize, Serialize};
use std::fs::{self};
use std::path::Path;

/// ログ
pub const LOG_FILE_PATH: &'static str = "kifuwarabe_WCSC28.log";
pub const LOG_ENABLE: bool = true; //false;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExeConfigFile {
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub profile: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineFile {
    pub engine: Engine,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Engine {
    pub name: String,
    pub author: String,
}

impl EngineFile {
    pub fn read() -> Self {
        // 実行ファイル名
        let exe_name = std::env::current_exe()
            .expect("Can't get the exec path")
            .file_name()
            .expect("Can't get the exec name")
            .to_string_lossy()
            .into_owned();
        // エンジン・パス。 Example: "kifuwarabe_shogi2018.exe.config.toml"
        let engine_path = match fs::read_to_string(format!("{}.config.toml", exe_name)) {
            Ok(text) => {
                let config: Result<ExeConfigFile, toml::de::Error> = toml::from_str(&text);
                match config {
                    Ok(config) => Path::new(&config.app.profile).join("Engine.toml"),
                    Err(why) => {
                        panic!("{}", why);
                    }
                }
            }
            Err(why) => {
                panic!("{}", why);
            }
        };
        // エンジン設定ファイルへのパス。 Example: "./profile\Engine.toml"
        match fs::read_to_string(format!("{}", engine_path.to_string_lossy())) {
            Ok(text) => {
                let config: Result<EngineFile, toml::de::Error> = toml::from_str(&text);
                match config {
                    Ok(config) => {
                        return config;
                    }
                    Err(why) => {
                        panic!("{}", why);
                    }
                }
            }
            Err(why) => {
                panic!("{}", why);
            }
        }
    }
}
