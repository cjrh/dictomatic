mod types;

use types::make_words;

use structopt::StructOpt;
use std::io::{BufReader, BufRead};
use fileinput::FileInput;
use std::io;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "")]
    word: String,
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
                println!("{}", hm[&line.unwrap()]);
            }
        },
        _ => println!("{}", hm[&args.word]),
    }
    Ok(())
}
