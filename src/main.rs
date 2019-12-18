mod types;

use types::make_words;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    word: String,
}

fn main() {
    let args = Cli::from_args();
    let hm = make_words();
    println!("{}", hm[&args.word]);
}
