impl MorseDecoder {

    pub fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .split("   ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|word| {
                word.to_string()
                    .split_whitespace()
                    .filter_map(|character| self.morse_code.get(&character.to_string()))
                    .map(|character| format!("{}", character))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string()
    }
    
}
