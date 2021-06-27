# KifuWarabe_WCSC28

2020年11月の 電竜戦から きふわらすと(Kifuwarust)にリネームして開発再開だぜ☆（＾～＾）  

|                         | ファイル                                                           |
| ----------------------- | ------------------------------------------------------------------ |
| ソース                  | `Kifuwarabe_WCSC28/Cargo.toml`                                     |
| 将棋エンジン ソース     | `Kifuwarabe_WCSC28/src/main.rs`                                    |
| GUI                     | なし                                                               |
| 将棋エンジン ランタイム | `Kifuwarabe_WCSC28/target/release/grayscale_kifuwarust_engine.exe` |
| 設定ファイル1           | `Kifuwarabe_WCSC28/grayscale_kifuwarust_engine.exe.config.toml`    |
| 設定ファイル2           | `Kifuwarabe_WCSC28/profile/Engine.toml`                            |

* `Kifuwarabe_WCSC28` のトップ・ディレクトリーに `Logs` ディレクトリーを作成してください。
* `cargo build --release` コマンドを打鍵して `将棋エンジン ランタイム` を生成してください。
* 設定ファイル1 を、 将棋エンジン ランタイムと同じディレクトリーに 置いてください。
* 設定ファイル1 の `profile = "./profile"` ファイルパスを 設定ファイル2のディレクトリーに合わせてください。

## Manual

Rust言語だぜ☆（＾～＾）　開発している最中のまま投げ込むぜ☆（＾～＾）

## Biuld

```shell
rustup update
cargo build --release
```

## Run

```shell
cargo run --release
# cargo run
```
