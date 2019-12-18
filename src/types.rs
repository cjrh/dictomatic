use std::collections::HashMap;

pub fn make_words() -> HashMap<String, String> {
//    let v = include_str!("../data/dictionary.json");
    let v = include_str!("../dictomatic.json");
    serde_json::from_str(v).unwrap()
}