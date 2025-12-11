use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

/// デフォルトの表示行数
pub const DEFAULT_LINES: u64 = 10;

#[derive(Parser, Debug)]
#[command(author, version, about)]
/// Rust version of `head`
pub struct Args {
    /// Input file(s)
    #[arg(default_value = "-", value_name = "FILE")]
    pub files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'),
        long,
        default_value_t = DEFAULT_LINES,
        value_name = "LINES",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    pub lines: u64,

    /// Number of bytes
    #[arg(
        short('c'),
        long,
        value_name = "BYTES",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    pub bytes: Option<u64>,
}

/// 指定された引数に基づいてファイルの先頭を表示します。
///
/// # Errors
///
/// ファイルの読み取り中にI/Oエラーが発生した場合、エラーを返します。
pub fn run(args: &Args) -> Result<()> {
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" },);
                }

                if let Some(num_bytes) = args.bytes {
                    let mut buffer = Vec::new();
                    let mut handle = file.take(num_bytes);
                    handle.read_to_end(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}

/// ファイルまたは標準入力を開いて`BufRead`を返します。
///
/// # Errors
///
/// ファイルが存在しない場合やアクセス権限がない場合、エラーを返します。
pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_nonexistent_file() {
        let result = open("nonexistent_file_12345.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_run_with_bad_file() {
        let args = Args {
            files: vec!["nonexistent_file_67890.txt".to_string()],
            lines: DEFAULT_LINES,
            bytes: None,
        };
        // run should succeed even with bad file (it prints error to stderr)
        let result = run(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_lines_constant() {
        assert_eq!(DEFAULT_LINES, 10);
    }
}
