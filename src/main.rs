mod types;

use types::make_words;

use structopt::StructOpt;
use std::io::{BufReader, BufRead};
use fileinput::FileInput;
use std::io;
use crate::types::Definition;
use serde_json;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "")]
    word: String,
}

fn find_word(defns: &Vec<Definition>, word: String) -> String {
    let r = match defns.binary_search_by_key(&word, |d| d.word.clone()) {
        Ok(idx) => defns[idx].defns.clone(),
        Err(_) => panic!("Not found")
    };
    r
}

fn print_defns(defns: &str) {
    serde_json::from_str::<Vec<String>>(defns).unwrap()
        .iter()
        .filter(|&s| !s.trim().is_empty())
        .for_each(|s| println!("{}", &s));
}

fn main() -> io::Result<()> {
    let hm = make_words();
    let args = Cli::from_args();
    match &args.word as &str {
        // If no word has been given as a parameter, read from stdin
        "" => {
            let filenames: Vec<&str> = vec![];
            let fileinput = FileInput::new(&filenames);
            let reader = BufReader::new(fileinput);
            for line in reader.lines() {
//                println!("{}", line.unwrap());
                let defns = find_word(&hm, line.unwrap().to_string());
                print_defns(&defns);
                println!("\n");
            }
        }
        word => {
            let defns = find_word(&hm, word.to_string());
            print_defns(&defns);
        }
    }
    Ok(())
}
