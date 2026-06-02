//! Regression tests pinning dictomatic's observable command-line behaviour.
//!
//! These run the compiled binary as a subprocess (Cargo exposes its path via
//! the CARGO_BIN_EXE_<name> env var) and compare its stdout byte-for-byte
//! against golden output captured from the original implementation. They exist
//! as a safety net for performance work: the lookup machinery may be rewritten
//! freely as long as these outputs stay identical.

use std::io::Write;
use std::process::{Command, Stdio};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_dictomatic"))
}

/// Run dictomatic with the given args and stdin, returning (stdout, exit_ok).
fn run(args: &[&str], stdin: &str) -> (String, bool) {
    let mut child = bin()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn dictomatic");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(stdin.as_bytes())
        .unwrap();
    let out = child.wait_with_output().expect("wait for dictomatic");
    (String::from_utf8(out.stdout).unwrap(), out.status.success())
}

const SNAG: &str = "\
snag\tnoun\tan unforeseen obstacle\t-
snag\tnoun\tan opening made forcibly as by pulling apart\t-
snag\tnoun\ta dead tree that is still standing, usually in an undisturbed forest\ta snag can provide food and a habitat for insects and birds
snag\tnoun\ta sharp protuberance\t-
snag\tverb\thew jaggedly\t-
snag\tverb\tcatch on a snag\tI snagged my stocking
snag\tverb\tget by acting quickly and smartly\tsnag a bargain
\n";

const CAT: &str = "\
cat\tnoun\tfeline mammal usually having thick soft fur and no ability to roar: domestic cats\twildcats
cat\tnoun\tany of several large cats typically able to roar and living in the wild\t-
cat\tnoun\ta large tracked vehicle that is propelled by two endless metal belts\t-
cat\tnoun\ta whip with nine knotted cords\tBritish sailors feared the cat
cat\tnoun\tthe leaves of the shrub Catha edulis which are chewed like tobacco or used to make tea\t-
cat\tnoun\ta spiteful woman gossip\tYou are such a cat!
cat\tnoun\tan informal term for a youth or man\t-
cat\tverb\tto eject the contents of the stomach through the mouth\t-
cat\tverb\tto beat with a cat-o'-nine-tails\t-
\n";

const A_MULTI: &str = "\
A\tnoun\tthe unit symbol for the ampere, the basic unit of electric current adopted under the Systeme International d'Unites\tMy headlights have a current of 5 A each.
A\tnoun\tshort form for adenine, a purine base found in DNA and RNA\tI used 2mg of A in the experiment.
A\tnoun\tone of the four nucleotides used in building DNA\tMy DNA gene sequence had 3,000 As in it.
A\tnoun\tany of several fat-soluble vitamins essential for normal vision\tDid you get your daily dose of A today?
\n";

#[test]
fn single_word_arg() {
    let (out, ok) = run(&["snag"], "");
    assert!(ok);
    assert_eq!(out, SNAG);
}

#[test]
fn multi_definition_word() {
    let (out, ok) = run(&["A"], "");
    assert!(ok);
    assert_eq!(out, A_MULTI);
}

#[test]
fn punctuation_keys() {
    let (out, _) = run(&["#"], "");
    assert_eq!(
        out,
        "#\tnoun\ta typographic symbol for number\tThe octothorpe is common on most phones.\n\n"
    );
    let (out, _) = run(&["&"], "");
    assert_eq!(
        out,
        "&\tnoun\ta typographic mark used to represent 'and'\tThe name of the firm was usually written with an ampersand.\n\n"
    );
}

#[test]
fn unknown_word_arg_prints_nothing() {
    let (out, ok) = run(&["zzqqxxnotaword"], "");
    assert!(ok);
    assert_eq!(out, "");
}

#[test]
fn multiple_word_args_in_order() {
    let (out, ok) = run(&["snag", "cat"], "");
    assert!(ok);
    assert_eq!(out, format!("{SNAG}{CAT}"));
}

#[test]
fn unknown_word_among_args_is_skipped() {
    // A missing word produces no output and no blank line; surrounding hits
    // are unaffected.
    let (out, ok) = run(&["snag", "zzqqxxnotaword", "cat"], "");
    assert!(ok);
    assert_eq!(out, format!("{SNAG}{CAT}"));
}

#[test]
fn stdin_mode_one_word_per_line() {
    let (out, ok) = run(&[], "snag\nnotaword\ncat\n");
    assert!(ok);
    assert_eq!(out, format!("{SNAG}{CAT}"));
}

#[test]
fn empty_stdin_prints_nothing() {
    let (out, ok) = run(&[], "");
    assert!(ok);
    assert_eq!(out, "");
}

#[test]
fn first_and_last_dictionary_entries() {
    // Exercises the binary-search boundaries: '#' is the first line of
    // words.txt and '😂' is the very last (and a multi-byte key whose JSON
    // definition contains an escaped surrogate pair).
    let (out, ok) = run(&["#"], "");
    assert!(ok);
    assert!(out.starts_with("#\tnoun"));

    let (out, ok) = run(&["😂"], "");
    assert!(ok);
    assert_eq!(
        out,
        "😂\tnoun\tthe feeling of being so happy, or laughing so much that you cry\tSeeing that hilarious video made me so 😂.\n\n"
    );
}
