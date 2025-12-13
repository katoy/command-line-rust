# findr

Rust implementation of the `find` command.
This CLI tool allows searching for files and directories based on paths, names (regex), and types.

## Table of Contents

- [Usage](#usage)
- [Installation](#installation)
- [Development](#development)
  - [Linting](#linting)
  - [Testing](#testing)
  - [Coverage](#coverage)

## Usage

```bash
findr [OPTIONS] [PATH]...
```

### Options

- `-n`, `--name <NAME>`: Filter by file name using regex (can be used multiple times).
- `-t`, `--type <TYPE>`: Filter by entry type (d: directory, f: file, l: link).
- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print version information.

### Examples

Search for all CSV files in the current directory:
```bash
findr -n ".*\.csv"
```

Search for directories named "src":
```bash
findr -t d -n "src"
```

## Installation

```bash
cargo install --path .
```

## Development

### Linting

Run `clippy` to lint the code:

```bash
cargo clippy
```

Output example:
```text
    Checking findr v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
```

### Testing

Run tests:

```bash
cargo test
```

### Coverage

Run coverage analysis using `cargo-llvm-cov`:

```bash
cargo llvm-cov --summary-only
```

Current coverage status:

```text
Filename                                                                  Regions    Missed Regions     Cover   Func
tions  Missed Functions  Executed       Lines      Missed Lines     Cover    Branches   Missed Branches     Cover
--------------------------------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------------------------
src/main.rs                                                                    66                 0   100.00%       
    9                 0   100.00%          52                 0   100.00%           0                 0         -
--------------------------------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------------------------
TOTAL                                                                          66                 0   100.00%       
    9                 0   100.00%          52                 0   100.00%           0                 0         -
```
