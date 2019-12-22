pub struct Mappings {
    pub words: Vec<&'static str>,
    pub defns: Vec<&'static str>,
}

/// Generates a hashmap of the dictionary definitions from a static
/// resource compiled right into the executable. The length of the
/// two vectors is the same, and indexes from correspond to indexes
/// on the other. The `defns` are JSON-encoded arrays of strings,
/// since there can be multiple definitions for a given word.
pub fn make_words() -> Mappings {
    Mappings {
        words: include_str!("../words.txt").lines().collect(),
        defns: include_str!("../defns.txt").lines().collect(),
    }
}
