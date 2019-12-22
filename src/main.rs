mod types;

use types::{make_words};

use structopt::StructOpt;
use std::io::{BufReader, BufRead};
use fileinput::FileInput;
use std::io;
use serde_json;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_str))]
    words: Vec<String>,
}

fn print_defns(defns: &Vec<&str>, idx: usize, word: &str) {
    serde_json::from_str::<Vec<String>>(defns[idx]).unwrap()
        .iter()
        .filter(|&s| !s.trim().is_empty())
        .for_each(|s| println!("{}\t{}", word, &s));
}

fn main() -> io::Result<()> {
    // TODO: ^WORD\tPOS\tDEFN\tEXAMPLE$
    let hm = make_words();
    let words = hm.words;
    let ds = hm.defns;
    let args = Cli::from_args();
    if args.words.is_empty() {
        let filenames: Vec<&str> = vec![];
        let fileinput = FileInput::new(&filenames);
        let reader = BufReader::new(fileinput);
        for line in reader.lines() {
            let w = line.unwrap();
            let idx = match words.binary_search(&w.as_str()) {
                Ok(idx) => idx,
                Err(_) => continue
            };
            print_defns(&ds, idx, &w);
            println!();
        }
    } else {
        for w in args.words {
            if let Ok(idx) = words.binary_search(&w.as_str()) {
                print_defns(&ds, idx, &w);
                println!();
            };
        }
    }
    Ok(())
}
