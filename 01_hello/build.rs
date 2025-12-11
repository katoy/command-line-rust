//! ビルドスクリプト - memory.x をリンカに認識させる

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // memory.x をビルド出力ディレクトリにコピー
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // リンカスクリプトの検索パスに追加
    println!("cargo:rustc-link-search={}", out.display());

    // 再ビルドトリガー
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
