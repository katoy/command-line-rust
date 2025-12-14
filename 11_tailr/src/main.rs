use crate::TakeValue::*;
use anyhow::{anyhow, bail, Result};
use clap::Parser;
use once_cell::sync::OnceCell;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `tail`
struct Args {
    /// Input file(s)
    #[arg(required = true)]
    files: Vec<String>,

    /// Number of lines
    #[arg(value_name = "LINES", short('n'), long, default_value = "10")]
    lines: String,

    /// Number of bytes
    #[arg(value_name = "BYTES", short('c'), long, conflicts_with("lines"))]
    bytes: Option<String>,

    /// Suppress headers
    #[arg(short, long)]
    quiet: bool,
}

static NUM_RE: OnceCell<Regex> = OnceCell::new();

#[derive(Debug, PartialEq)]
enum TakeValue {
    PlusZero,
    TakeNum(i64),
}

// --------------------------------------------------
fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// --------------------------------------------------
fn run(args: Args) -> Result<()> {
    let lines = parse_num(args.lines)
        .map_err(|e| anyhow!("illegal line count -- {e}"))?;

    let bytes = args
        .bytes
        .map(parse_num)
        .transpose()
        .map_err(|e| anyhow!("illegal byte count -- {e}"))?;

    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match File::open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(file) => {
                if !args.quiet && num_files > 1 {
                    println!(
                        "{}==> {filename} <==",
                        if file_num > 0 { "\n" } else { "" },
                    );
                }

                let mut stdout = std::io::stdout();
                if let Some(num_bytes) = &bytes {
                    print_bytes(file, num_bytes, &mut stdout)?;
                } else {
                    print_lines(file, &lines, &mut stdout)?;
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
fn parse_num(val: String) -> Result<TakeValue> {
    let num_re =
        NUM_RE.get_or_init(|| Regex::new(r"^([+-])?(\d+)$").unwrap());

    match num_re.captures(&val) {
        Some(caps) => {
            let sign = caps.get(1).map_or("-", |m| m.as_str());
            let signed_num =
                format!("{sign}{}", caps.get(2).unwrap().as_str());

            if let Ok(num) = signed_num.parse() {
                if sign == "+" && num == 0 {
                    Ok(PlusZero)
                } else {
                    Ok(TakeNum(num))
                }
            } else {
                bail!(val)
            }
        }
        _ => bail!(val),
    }
}

// --------------------------------------------------
fn output_buffer(buf: &[u8], target: &mut impl Write) -> Result<()> {
    if !buf.is_empty() {
        write!(target, "{}", String::from_utf8_lossy(buf))?;
    }
    Ok(())
}

// --------------------------------------------------
fn print_bytes<T: Read + Seek>(
    mut file: T,
    num_bytes: &TakeValue,
    target: &mut impl Write,
) -> Result<()> {
    match num_bytes {
        PlusZero => {
            // +0: Print the whole file
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            output_buffer(&buf, target)?;
        }
        TakeNum(n) if *n > 0 => {
             // +N: Skip N-1 bytes, print the rest
            file.seek(SeekFrom::Start((*n - 1) as u64))?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            output_buffer(&buf, target)?;
        }
        TakeNum(n) => {
            // -N or N: Print last |N| bytes
            // Note: parse_num converts "3" to -3, so this handles both "-c 3" and "-c -3"
            let len = file.seek(SeekFrom::End(0))?;
            let n = n.unsigned_abs();
            let start = len.saturating_sub(n);
            file.seek(SeekFrom::Start(start))?;
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            output_buffer(&buf, target)?;
        }
    }

    Ok(())
}

// --------------------------------------------------
fn print_lines<T: Read + Seek>(
    mut file: T,
    num_lines: &TakeValue,
    target: &mut impl Write,
) -> Result<()> {
    match num_lines {
        PlusZero => {
            let mut reader = BufReader::new(file);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            output_buffer(&buf, target)?;
        }
        TakeNum(n) if *n > 0 => {
             // +N: Skip N-1 lines
            let mut reader = BufReader::new(file);
            let lines_to_skip = *n as u64;
            if lines_to_skip > 0 {
                let mut skipped = 0;
                let mut buf = Vec::new();
                while skipped < lines_to_skip - 1 {
                    let bytes = reader.read_until(b'\n', &mut buf)?;
                    if bytes == 0 {
                        break;
                    }
                    skipped += 1;
                    buf.clear();
                }
            }
             // Print the rest
             let mut buf = Vec::new();
             reader.read_to_end(&mut buf)?;
             output_buffer(&buf, target)?;
        }
        TakeNum(n) => {
             // -N: Print last |N| lines
            let total_lines_to_print = n.unsigned_abs();
            if total_lines_to_print == 0 {
                return Ok(());
            }

            let file_len = file.seek(SeekFrom::End(0))?;
            if file_len == 0 {
                return Ok(());
            }

            let mut position = file_len;
            let mut lines_found = 0;
            let chunk_size = 4096;
            
            loop {
                let to_read = if position < chunk_size { position } else { chunk_size };
                position -= to_read;
                
                file.seek(SeekFrom::Start(position))?;
                let mut buf = vec![0; to_read as usize];
                file.read_exact(&mut buf)?;

                for (i, byte) in buf.iter().enumerate().rev() {
                    if *byte == b'\n' {
                        // Ignore the very last byte if it is a newline
                        if position + i as u64 + 1 == file_len {
                            continue;
                        }
                        
                        lines_found += 1;
                        if lines_found == total_lines_to_print {
                            let start_index = position + i as u64 + 1;
                             file.seek(SeekFrom::Start(start_index))?;
                             let mut reader = BufReader::new(file);
                             std::io::copy(&mut reader, target)?;
                             return Ok(());
                        }
                    }
                }

                if position == 0 {
                    file.seek(SeekFrom::Start(0))?;
                    let mut reader = BufReader::new(file);
                    std::io::copy(&mut reader, target)?;
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

// --------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::{Cursor, Read, Seek, SeekFrom};

    #[test]
    fn test_parse_num() {
        // All integers should be interpreted as negative numbers
        let res = parse_num("3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // A leading "+" should result in a positive number
        let res = parse_num("+3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(3));

        // An explicit "-" value should result in a negative number
        let res = parse_num("-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(-3));

        // Zero is zero
        let res = parse_num("0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(0));

        // Plus zero is special
        let res = parse_num("+0".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), PlusZero);

        // Test boundaries
        let res = parse_num(i64::MAX.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num((i64::MIN + 1).to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN + 1));

        let res = parse_num(format!("+{}", i64::MAX));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MAX));

        let res = parse_num(i64::MIN.to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), TakeNum(i64::MIN));

        // Test for overflow: a number that matches the regex but is too large for i64
        let overflow_val = format!("+{}", (i64::MAX as u64) + 1);
        let res = parse_num(overflow_val.clone());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), overflow_val);

        // A floating-point value is invalid
        let res = parse_num("3.14".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "3.14");

        // Any non-integer string is invalid
        let res = parse_num("foo".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "foo");
    }

    // --- Mock for IO Errors ---
    struct MockError {
        data: Cursor<Vec<u8>>,
        error_on_read: bool,
        error_on_seek: bool,
    }

    impl MockError {
        fn new(data: Vec<u8>) -> Self {
            Self {
                data: Cursor::new(data),
                error_on_read: false,
                error_on_seek: false,
            }
        }
        
        fn with_read_err() -> Self {
            let mut m = Self::new(vec![0; 10]);
            m.error_on_read = true;
            m
        }

        fn with_seek_err() -> Self {
            let mut m = Self::new(vec![0; 10]);
            m.error_on_seek = true;
            m
        }

        fn with_content(content: &str) -> Self {
            Self::new(content.as_bytes().to_vec())
        }
    }

    impl Read for MockError {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.error_on_read {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Mock read error"))
            } else {
                self.data.read(buf)
            }
        }
    }

    impl Seek for MockError {
        fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
            if self.error_on_seek {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Mock seek error"))
            } else {
                self.data.seek(pos)
            }
        }
    }

    // --- Tests for print_bytes IO errors ---
    #[test]
    fn test_print_bytes_read_err_plus_zero() {
        let file = MockError::with_read_err();
        let mut buffer = Vec::new();
        assert!(print_bytes(file, &PlusZero, &mut buffer).is_err());
    }

    #[test]
    fn test_print_bytes_seek_err_plus_n() {
        let file = MockError::with_seek_err();
        let mut buffer = Vec::new();
        assert!(print_bytes(file, &TakeNum(5), &mut buffer).is_err());
    }

    #[test]
    fn test_print_bytes_read_err_plus_n() {
        let file = MockError::with_read_err();
        let mut buffer = Vec::new();
        assert!(print_bytes(file, &TakeNum(5), &mut buffer).is_err());
    }

    #[test]
    fn test_print_bytes_seek_err_minus_n() {
        // seek to end
        let file = MockError::with_seek_err();
        let mut buffer = Vec::new();
        assert!(print_bytes(file, &TakeNum(-5), &mut buffer).is_err());
    }

    // --- Tests for print_bytes OK paths to cover MockError's else branches ---
    #[test]
    fn test_mock_read_ok() {
        let mut file = MockError::with_content("hello");
        let mut buf = [0; 5]; // Use fixed-size array for read
        assert_eq!(file.read(&mut buf).unwrap(), 5); // Directly call read
        assert_eq!(std::str::from_utf8(&buf).unwrap(), "hello");
    }

    #[test]
    fn test_mock_seek_ok() {
        let mut file = MockError::with_content("hello");
        assert!(file.seek(SeekFrom::Start(1)).is_ok());
        assert_eq!(file.seek(SeekFrom::Current(0)).unwrap(), 1);
    }

    // --- Tests for print_lines IO errors ---
    #[test]
    fn test_print_lines_read_err_plus_zero() {
        let file = MockError::with_read_err();
        let mut buffer = Vec::new();
        assert!(print_lines(file, &PlusZero, &mut buffer).is_err());
    }

    #[test]
    fn test_print_lines_read_err_plus_n() {
        let file = MockError::with_read_err();
        let mut buffer = Vec::new();
        assert!(print_lines(file, &TakeNum(5), &mut buffer).is_err());
    }

    #[test]
    fn test_print_lines_seek_err_minus_n() {
        // Seek to end
        let file = MockError::with_seek_err();
        let mut buffer = Vec::new();
        assert!(print_lines(file, &TakeNum(-5), &mut buffer).is_err());
    }

    // --- Tests for print_bytes / print_lines correct output (Success Cases) ---
    #[test]
    fn test_print_bytes_plus_zero_ok() {
        let file = MockError::with_content("1234567890");
        let mut buffer = Vec::new();
        assert!(print_bytes(file, &PlusZero, &mut buffer).is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "1234567890");
    }

    #[test]
    fn test_print_bytes_plus_n_ok() {
        let file = MockError::with_content("1234567890");
        let mut buffer = Vec::new();
        // +3 means skip 2 bytes (start from 3rd byte: '3')
        assert!(print_bytes(file, &TakeNum(3), &mut buffer).is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "34567890");
    }

    #[test]
    fn test_print_bytes_minus_n_ok() {
        let file = MockError::with_content("1234567890");
        let mut buffer = Vec::new();
        // -3 means last 3 bytes
        assert!(print_bytes(file, &TakeNum(-3), &mut buffer).is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "890");
    }

    #[test]
    fn test_print_lines_plus_zero_ok() {
        let file = MockError::with_content("line1\nline2\n");
        let mut buffer = Vec::new();
        assert!(print_lines(file, &PlusZero, &mut buffer).is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "line1\nline2\n");
    }

    #[test]
    fn test_print_lines_plus_n_ok() {
        let file = MockError::with_content("line1\nline2\nline3\n");
        let mut buffer = Vec::new();
        // +2 means skip 1 line, start from line 2
        assert!(print_lines(file, &TakeNum(2), &mut buffer).is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "line2\nline3\n");
    }

    #[test]
    fn test_print_lines_minus_n_ok() {
        let file = MockError::with_content("line1\nline2\nline3\n");
        let mut buffer = Vec::new();
        // -2 means last 2 lines
        assert!(print_lines(file, &TakeNum(-2), &mut buffer).is_ok());
        assert_eq!(String::from_utf8(buffer).unwrap(), "line2\nline3\n");
    }
}
