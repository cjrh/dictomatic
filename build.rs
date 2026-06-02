//! Precomputes line indexes for the embedded dictionary blobs.
//!
//! `words.txt` and `defns.txt` are baked into the binary as raw text. Splitting
//! those 13 MB into lines on every invocation was the program's whole startup
//! cost. Instead, we do that work once, here at build time, emitting for each
//! file a compact table of `(byte_offset: u32, byte_len: u32)` records — one
//! per line, little-endian. Entry `i` locates line `i` within the blob, letting
//! the runtime binary-search the word list without ever scanning it.

use std::env;
use std::fs;
use std::path::Path;

/// Reads `src` and writes a `(offset, len)` record per line to `dst`, matching
/// `str::lines()` semantics: split on `\n`, drop one trailing `\r`, and do not
/// emit a spurious final record for a trailing newline.
fn build_index(src: &str, dst: &Path) {
    let text = fs::read(src).unwrap_or_else(|e| panic!("read {}: {}", src, e));
    let mut idx = Vec::with_capacity(text.len() / 8);

    let mut push = |start: usize, end: usize| {
        idx.extend_from_slice(&(start as u32).to_le_bytes());
        idx.extend_from_slice(&((end - start) as u32).to_le_bytes());
    };

    let mut start = 0;
    for (i, &b) in text.iter().enumerate() {
        if b == b'\n' {
            let end = if i > start && text[i - 1] == b'\r' {
                i - 1
            } else {
                i
            };
            push(start, end);
            start = i + 1;
        }
    }
    if start < text.len() {
        let end = if text[text.len() - 1] == b'\r' {
            text.len() - 1
        } else {
            text.len()
        };
        push(start, end);
    }

    fs::write(dst, &idx).unwrap_or_else(|e| panic!("write {}: {e}", dst.display()));
}

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    build_index("words.txt", &Path::new(&out).join("words.idx"));
    build_index("defns.txt", &Path::new(&out).join("defns.idx"));

    println!("cargo:rerun-if-changed=words.txt");
    println!("cargo:rerun-if-changed=defns.txt");
    println!("cargo:rerun-if-changed=build.rs");
}
