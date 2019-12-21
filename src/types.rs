//use std::time::{SystemTime, UNIX_EPOCH};

pub struct Definition {
    pub word: String,
    pub defns: String,
}

/// Generates a hashmap of the dictionary definitions from a static
/// resource compiled right into the executable.
pub fn make_words() -> Vec<Definition> {
//    let now = SystemTime::now();
//    println!("{}", now.elapsed().unwrap().as_millis());
    const V: &str = include_str!("../dictomatic.tsv");
//    println!("{}", now.elapsed().unwrap().as_millis());
    let out = V.lines().map(|line| {
        let parts: Vec<&str> = line.splitn(2, "\t").collect();
        Definition { word: parts[0].to_string(), defns: parts[1].to_string() }
    }).collect();
//    println!("{}", now.elapsed().unwrap().as_millis());
    out
}