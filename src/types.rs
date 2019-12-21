use std::collections::HashMap;

/// Generates a hashmap of the dictionary definitions from a static
/// resource compiled right into the executable.
pub fn make_words() -> HashMap<String, String> {
    let v = include_str!("../dictomatic.json");
    serde_json::from_str(v).unwrap()
}