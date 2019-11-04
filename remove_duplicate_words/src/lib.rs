// this is only here because the test requires it for some reason.
// I made my own solution using HashSet, but trashed it because 
// the result is always in the wrong order.
use std::collections::HashSet;

fn remove_duplicate_words(s: &str) -> String {
    let mut unique_list: Vec<String> = Vec::new();
    for word in s.split_whitespace() {
        if !unique_list.contains(&word.to_string()){
            unique_list.push(word.to_string());
        }
    }

    unique_list.join(" ")
}
