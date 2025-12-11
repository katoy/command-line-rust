use clap::Parser;

/// コマンドライン引数を保持する構造体
///
/// # フィールド
/// - `text`: 出力するテキスト（1つ以上必須）
/// - `omit_newline`: trueの場合、末尾の改行を省略する
#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `echo`
struct Args {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

/// echoコマンドのRust実装
///
/// 引数として渡されたテキストをスペース区切りで出力する。
/// `-n` オプションが指定された場合は末尾の改行を省略する。
fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
