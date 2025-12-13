use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `uniq`
struct Args {
    /// Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    let file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;

    let out_file: Box<dyn Write> = match &args.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    do_uniqr(file, out_file, args.count)
}

// --------------------------------------------------
fn do_uniqr(
    mut file: Box<dyn BufRead>,
    mut out_file: Box<dyn Write>,
    count_mode: bool,
) -> Result<()> {
    let mut print = |num: u64, text: &str| -> Result<()> {
        if num > 0 {
            if count_mode {
                write!(out_file, "{num:>4} {text}")?;
            } else {
                write!(out_file, "{text}")?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    print(count, &previous)?;

    Ok(())
}

// --------------------------------------------------
fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    struct ErrorWriter;
    impl Write for ErrorWriter {
        fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_do_uniqr_error() {
        let input = Cursor::new("one\ntwo\n");
        let output = Box::new(ErrorWriter);
        let result = do_uniqr(Box::new(input), output, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_do_uniqr_count_error() {
        let input = Cursor::new("one\ntwo\n");
        let output = Box::new(ErrorWriter);
        let result = do_uniqr(Box::new(input), output, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_do_uniqr_last_line_error() {
        let input = Cursor::new("one\n");
        let output = Box::new(ErrorWriter);
        let result = do_uniqr(Box::new(input), output, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_error_writer() {
        let mut writer = ErrorWriter;
        assert!(writer.flush().is_ok());
    }
}