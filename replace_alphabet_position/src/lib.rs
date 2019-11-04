const LETTERS: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn alphabet_position(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|letter| match_letter(letter))
        .map(|number| format!("{}", number))
        .collect::<Vec<String>>()
        .join(" ")
}

fn match_letter(letter: char) -> Option<u8> {
    match letter {
        'a'...'z' => Some(LETTERS.iter().position(|&r| r == letter).unwrap() as u8 + 1),
        _ => None,
    }
}
