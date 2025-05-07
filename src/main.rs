use std::{fs::File, io::{self, BufRead, BufReader}, process::ExitCode, path::Path};
use clap::{Parser, ValueEnum};
use walkdir::WalkDir;
use regex::{Regex, RegexBuilder};
use ansi_term::Colour;
use atty::Stream;

/// Color modes for output highlighting.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ColorMode {
    Auto,
    Always,
    Never,
}

/// Simple recursive grep-like tool in Rust, avoiding shell glob limits.
#[derive(Parser, Debug)]
#[command(
    name = "ggrep",
    author,
    version,
    about = "Recursive grep in Rust without shell globs",
    long_about = None
)]
struct Args {
    /// Pattern to search for (regex or literal)
    keyword: String,

    /// Directory prefix to start searching (default ".")
    #[arg(default_value = ".")]
    prefix: String,

    /// Case-insensitive match
    #[arg(short = 'i', long)]
    ignore_case: bool,

    /// Invert match
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// Count matches per file
    #[arg(short = 'c', long)]
    count: bool,

    /// List only filenames with matches
    #[arg(short = 'l', long)]
    list_files: bool,

    /// Fixed-string search (no regex)
    #[arg(short = 'F', long)]
    fixed_strings: bool,

    /// Match whole words only
    #[arg(short = 'w', long)]
    word_regexp: bool,

    /// Colorize matches: auto, always, or never
    #[arg(long, value_enum, default_value_t = ColorMode::Auto)]
    color: ColorMode,
}

/// Search depth and supported file extensions.
const DEFAULT_DEPTH: usize = 7;
const EXTENSIONS: &[&str] = &[
    "cpp", "h", "txt", "html", "php", "c", "css", "json", "py", "js",
];

/// Builds the Regex matcher according to CLI flags.
fn build_matcher(args: &Args) -> Regex {
    let mut pattern = if args.fixed_strings {
        regex::escape(&args.keyword)
    } else {
        args.keyword.clone()
    };
    if args.word_regexp {
        pattern = format!(r"\b{}\b", pattern);
    }
    RegexBuilder::new(&pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Invalid pattern '{}': {}", pattern, e);
            std::process::exit(2);
        })
}

/// Highlights all matches in a line with ANSI red bold when enabled.
fn highlight_line(line: &str, re: &Regex, colorize: bool) -> String {
    if !colorize {
        return line.to_string();
    }
    let mut result = String::new();
    let mut last_end = 0;
    for mat in re.find_iter(line) {
        result.push_str(&line[last_end..mat.start()]);
        result.push_str(&Colour::Red.bold().paint(mat.as_str()).to_string());
        last_end = mat.end();
    }
    result.push_str(&line[last_end..]);
    result
}

/// Counts matches in a file; applies invert logic if requested.
fn count_matches(path: &Path, re: &Regex, invert: bool) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut count = 0;
    for line in reader.lines() {
        let line = line?;
        let is_match = re.is_match(&line);
        if invert ^ is_match {
            count += 1;
        }
    }
    Ok(count)
}

/// Prints matching lines with highlighting; returns true if any match found.
fn print_matches(path: &Path, re: &Regex, invert: bool, colorize: bool) -> io::Result<bool> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut found = false;
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let is_match = re.is_match(&line);
        if invert ^ is_match {
            found = true;
            println!(
                "{}:{}:{}",
                path.display(),
                i + 1,
                highlight_line(&line, re, colorize)
            );
        }
    }
    Ok(found)
}

/// Entry point: walks directory, applies search logic, and sets exit code.
fn main() -> ExitCode {
    let args = Args::parse();
    let re = build_matcher(&args);

    // Determine whether to colorize output
    let colorize = match args.color {
        ColorMode::Always => true,
        ColorMode::Never => false,
        ColorMode::Auto => atty::is(Stream::Stdout),
    };

    let mut any_match = false;
    for entry in WalkDir::new(&args.prefix)
        .max_depth(DEFAULT_DEPTH + 1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|ext| EXTENSIONS.contains(&ext))
                .unwrap_or(false)
        })
    {
        let path = entry.path();
        if args.count {
            match count_matches(path, &re, args.invert_match) {
                Ok(0) => (),
                Ok(c) => {
                    println!("{}:{}", path.display(), c);
                    any_match = true;
                }
                Err(e) => eprintln!("Error reading {}: {}", path.display(), e),
            }
        } else if args.list_files {
            match count_matches(path, &re, args.invert_match) {
                Ok(c) if c > 0 => {
                    println!("{}", path.display());
                    any_match = true;
                }
                Ok(_) => (),
                Err(e) => eprintln!("Error reading {}: {}", path.display(), e),
            }
        } else {
            match print_matches(path, &re, args.invert_match, colorize) {
                Ok(found) => any_match |= found,
                Err(e) => eprintln!("Error reading {}: {}", path.display(), e),
            }
        }
    }

    if any_match {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
