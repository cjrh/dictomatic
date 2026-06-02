mod types;

use clap::Parser;
use std::io::{self, BufRead, Write};

#[derive(Parser)]
struct Cli {
    words: Vec<String>,
}

/// Writes a word's definitions to `out`: one `word\t<defn>` line per non-empty
/// definition, followed by a trailing blank line. `defns` is the raw JSON array
/// of "pos\tdefn\texample" strings stored for the word; parsing it here decodes
/// JSON escapes (e.g. `\t`, `\uXXXX`) into their characters.
fn print_defns(out: &mut impl Write, word: &str, defns: &str) -> io::Result<()> {
    let parsed: Vec<String> = serde_json::from_str(defns).unwrap();
    for s in parsed.iter().filter(|s| !s.trim().is_empty()) {
        writeln!(out, "{}\t{}", word, s)?;
    }
    writeln!(out)
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // With no word arguments, read one word per line from stdin; otherwise look
    // up each argument. Either way, a hit prints its definitions and a miss is
    // silently skipped.
    if args.words.is_empty() {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let w = line?;
            if let Some(defns) = types::lookup(&w) {
                print_defns(&mut out, &w, defns)?;
            }
        }
    } else {
        for w in &args.words {
            if let Some(defns) = types::lookup(w) {
                print_defns(&mut out, w, defns)?;
            }
        }
    }

    out.flush()
}
