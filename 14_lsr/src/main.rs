mod owner;

use anyhow::Result;
use chrono::{DateTime, Local};
use clap::Parser;
use owner::Owner;
use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};
use tabular::{Row, Table};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `ls`
struct Args {
    /// Files and/or directories
    #[arg(default_value = ".")]
    paths: Vec<String>,

    /// Long listing
    #[arg(short, long)]
    long: bool,

    /// Show all files
    #[arg(short('a'), long("all"))]
    show_hidden: bool,
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
    let paths = find_files(&args.paths, args.show_hidden)?;
    if args.long {
        println!("{}", format_output(&paths));
    } else {
        for path in paths {
            println!("{}", path.display());
        }
    }
    Ok(())
}

// --------------------------------------------------
fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    let mut results = vec![];
    for name in paths {
        match fs::metadata(name) {
            Err(e) => eprintln!("{name}: {e}"),
            Ok(meta) => {
                if meta.is_dir() {
                    for entry in fs::read_dir(name)?.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        let is_hidden = path
                            .file_name()
                            .is_some_and(|file_name| file_name.to_string_lossy().starts_with('.'));
                        if !is_hidden || show_hidden {
                            results.push(entry.path());
                        }
                    }
                } else {
                    results.push(PathBuf::from(name));
                }
            }
        }
    }

    Ok(results)
}

// --------------------------------------------------
fn format_output(paths: &[PathBuf]) -> String {
    //         1   2     3     4     5     6     7     8
    let fmt = "{:<}{:<}  {:>}  {:<}  {:<}  {:>}  {:<}  {:<}";
    let mut table = Table::new(fmt);

    for path in paths {
        let metadata = match path.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}: {}", path.display(), e);
                continue;
            }
        };

        let uid = metadata.uid();
        let user = uid_to_name(uid);

        let gid = metadata.gid();
        let group = gid_to_name(gid);

        let file_type = if path.is_dir() { "d" } else { "-" };
        let perms = format_mode(metadata.mode());
        let modified: DateTime<Local> = DateTime::from(
            metadata
                .modified()
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
        );

        table.add_row(
            Row::new()
                .with_cell(file_type) // 1
                .with_cell(perms) // 2
                .with_cell(metadata.nlink()) // 3
                .with_cell(user) // 4
                .with_cell(group) // 5
                .with_cell(metadata.len()) // 6
                .with_cell(modified.format("%b %d %y %H:%M")) // 7
                .with_cell(path.display()), // 8
        );
    }

    format!("{table}")
}

// --------------------------------------------------
fn uid_to_name(uid: u32) -> String {
    match get_user_by_uid(uid) {
        Some(u) => u.name().to_string_lossy().into_owned(),
        None => uid.to_string(),
    }
}

// --------------------------------------------------
fn gid_to_name(gid: u32) -> String {
    match get_group_by_gid(gid) {
        Some(g) => g.name().to_string_lossy().into_owned(),
        None => gid.to_string(),
    }
}

// --------------------------------------------------
/// Given a file mode in octal format like 0o751,
/// return a string like "rwxr-x--x"
fn format_mode(mode: u32) -> String {
    format!(
        "{}{}{}",
        mk_triple(mode, Owner::User),
        mk_triple(mode, Owner::Group),
        mk_triple(mode, Owner::Other),
    )
}

// --------------------------------------------------
/// Given an octal number like 0o500 and an [`Owner`],
/// return a string like "r-x"
fn mk_triple(mode: u32, owner: Owner) -> String {
    let [read, write, execute] = owner.masks();
    format!(
        "{}{}{}",
        if mode & read == 0 { "-" } else { "r" },
        if mode & write == 0 { "-" } else { "w" },
        if mode & execute == 0 { "-" } else { "x" },
    )
}

// --------------------------------------------------
#[cfg(test)]
mod test {
    use super::{Owner, find_files, format_mode, format_output, gid_to_name, mk_triple, uid_to_name};
    use pretty_assertions::assert_eq;
    use std::path::PathBuf;

    #[test]
    fn test_find_files_empty() {
        let res = find_files(&[], false);
        assert!(res.is_ok());
        assert!(res.unwrap().is_empty());
    }

    #[test]
    fn test_find_files() {
        // Find all non-hidden entries in a directory
        let res = find_files(&["tests/inputs".to_string()], false);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );

        // Any existing file should be found even if hidden
        let res = find_files(&["tests/inputs/.hidden".to_string()], false);
        assert!(res.is_ok());
        let filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        assert_eq!(filenames, ["tests/inputs/.hidden"]);

        // Test multiple path arguments
        let res = find_files(
            &[
                "tests/inputs/bustle.txt".to_string(),
                "tests/inputs/dir".to_string(),
            ],
            false,
        );
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            ["tests/inputs/bustle.txt", "tests/inputs/dir/spiders.txt"]
        );
    }

    #[test]
    fn test_find_files_hidden() {
        // Find all entries in a directory including hidden
        let res = find_files(&["tests/inputs".to_string()], true);
        assert!(res.is_ok());
        let mut filenames: Vec<_> = res
            .unwrap()
            .iter()
            .map(|entry| entry.display().to_string())
            .collect();
        filenames.sort();
        assert_eq!(
            filenames,
            [
                "tests/inputs/.hidden",
                "tests/inputs/bustle.txt",
                "tests/inputs/dir",
                "tests/inputs/empty.txt",
                "tests/inputs/fox.txt",
            ]
        );
    }

    fn long_match(
        line: &str,
        expected_name: &str,
        expected_perms: &str,
        expected_size: Option<&str>,
    ) {
        let parts: Vec<_> = line.split_whitespace().collect();
        assert!(!parts.is_empty() && parts.len() <= 10);

        let perms = parts.first().unwrap();
        assert_eq!(perms, &expected_perms);

        if let Some(size) = expected_size {
            let file_size = parts.get(4).unwrap();
            assert_eq!(file_size, &size);
        }

        let display_name = parts.last().unwrap();
        assert_eq!(display_name, &expected_name);
    }

    #[test]
    fn test_format_output_one() {
        let bustle_path = "tests/inputs/bustle.txt";
        let bustle = PathBuf::from(bustle_path);

        let out = format_output(&[bustle]);
        let lines: Vec<&str> = out.split('\n').filter(|s| !s.is_empty()).collect();
        assert_eq!(lines.len(), 1);

        let line1 = lines.first().unwrap();
        long_match(line1, bustle_path, "-rw-r--r--", Some("193"));
    }

    #[test]
    fn test_format_output_two() {
        let out = format_output(&[
            PathBuf::from("tests/inputs/dir"),
            PathBuf::from("tests/inputs/empty.txt"),
        ]);
        let mut lines: Vec<&str> = out.split('\n').filter(|s| !s.is_empty()).collect();
        lines.sort();
        assert_eq!(lines.len(), 2);

        let empty_line = lines.remove(0);
        long_match(
            empty_line,
            "tests/inputs/empty.txt",
            "-rw-r--r--",
            Some("0"),
        );

        let dir_line = lines.remove(0);
        long_match(dir_line, "tests/inputs/dir", "drwxr-xr-x", None);
    }

    #[test]
    fn test_format_output_error() {
        let bad_path = PathBuf::from("non_existent_file_xyz");
        let res = format_output(&[bad_path]);
        assert!(res.is_empty());
    }

    #[test]
    fn test_mk_triple() {
        assert_eq!(mk_triple(0o751, Owner::User), "rwx");
        assert_eq!(mk_triple(0o751, Owner::Group), "r-x");
        assert_eq!(mk_triple(0o751, Owner::Other), "--x");
        assert_eq!(mk_triple(0o600, Owner::Other), "---");
    }

    #[test]
    fn test_format_mode() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
        assert_eq!(format_mode(0o421), "r---w---x");
    }

    #[test]
    fn test_uid_to_name() {
        assert_eq!(uid_to_name(999999), "999999");
    }

    #[test]
    fn test_gid_to_name() {
        assert_eq!(gid_to_name(999999), "999999");
    }
}
