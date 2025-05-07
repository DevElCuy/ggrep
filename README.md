# ggrep

`ggrep` is a fast, recursive grep‑like command‑line utility written in Rust that avoids shell glob limits and large argument lists. It searches files under a directory prefix for a pattern (regex or fixed string) and supports the familiar `grep` flags for inversion, counting, file listing, and word matching.

## Features

* Recursive file traversal using [`walkdir`](https://crates.io/crates/walkdir)
* Regex or literal (fixed‑string) search via [`regex`](https://crates.io/crates/regex)
* Case‑insensitive matching (`-i` / `--ignore-case`)
* Invert match to show non‑matching lines (`-v` / `--invert-match`)
* Count matches per file (`-c` / `--count`)
* List only file names with matches (`-l` / `--list-files`)
* Whole‑word matching (`-w` / `--word-regexp`)
* Configurable color highlighting (`--color auto|always|never`)
* Skips binary files automatically

## Installation

Make sure you have Rust and Cargo installed (via [rustup](https://rustup.rs/)). Then:

```sh
cargo install ggrep
```

Alternatively, to build from source:

```sh
git clone https://github.com/DevElCuy/ggrep.git
cd ggrep
cargo build --release
# Optionally install:
cargo install --path .
```

## Usage

```text
ggrep [OPTIONS] <keyword> [prefix]
```

* `<keyword>`: Pattern to search for (interpreted as regex by default)
* `[prefix]` : Directory to start searching (defaults to `.`)

### Options

```
-h, --help             Print help information
-V, --version          Print version information
-i, --ignore-case      Case-insensitive search
-v, --invert-match     Show lines that do *not* match
-c, --count            Print count of matching lines per file
-l, --list-files       Print only file names with matches
-F, --fixed-strings    Treat pattern as a literal string, not regex
-w, --word-regexp      Match whole words only
    --color <mode>     Colorize matches: auto, always, or never (default: auto)
```

## Examples

Search current directory for the regex `fn main` (case‑insensitive, highlight matches):

```sh
ggrep -i "fn main" .
```

Count occurrences of `TODO` in all files under `src/`:

```sh
ggrep -c TODO src/
```

List files under `/var/log` that do *not* contain the literal `error`:

```sh
ggrep -v -l -F error /var/log
```

Match the whole word `unsafe` in `.rs` files (no regex):

```sh
ggrep -w -F unsafe src/
```

Always disable coloring (even in a terminal):

```sh
ggrep --color never FIXME
```

## Contributing

Contributions, issues, and feature requests are welcome!

1. Fork the repository
2. Create a new branch (`git checkout -b feature-name`)
3. Commit your changes (`git commit -m "Add feature-name"`)
4. Push to the branch (`git push origin feature-name`)
5. Open a pull request

Please format code with `cargo fmt` and check for warnings with `cargo clippy` before submitting.

## License

Licensed under MIT OR Apache‑2.0. See [LICENSE](LICENSE) for details.

