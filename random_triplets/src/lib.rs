use std::collections::HashMap;

#[derive(Debug, Clone)]
struct SecretString {
    pub secret_string_characters: HashMap<char, SecretStringCharacter>,
}

#[derive(Debug, Clone)]
struct SecretStringCharacter {
    c: char,
    before: Vec<char>,
}

impl SecretString {
    pub fn new() -> SecretString {
        SecretString {
            secret_string_characters: HashMap::new(),
        }
    }

    pub fn add_char(&mut self, character: &char) {
        self.secret_string_characters
            .insert(*character, SecretStringCharacter::new(*character));
    }

    pub fn add_following_to_char(&mut self, character: char, following: char) {
        let entry = self
            .secret_string_characters
            .entry(character)
            .or_insert(SecretStringCharacter::new(character));
        entry.add_following_char(following);
    }

    // this is a hacky way of finding out if there are enough characters
    // with varying following lists. if this number is equal to the
    // number of unique characters in the triplets, then I can find out the
    // position of the character without guessing
    pub fn get_lengths_list(&self) -> Vec<usize> {
        let mut return_vec: Vec<usize> = Vec::new();
        for (_, character) in self.secret_string_characters.iter() {
            // return_vec.push(character.before.len());
            if !return_vec.contains(&character.before.len()){
                return_vec.push(character.before.len());
            }
        }

        return_vec
    }

    pub fn add_dependent_followers(&mut self){
        let copy = HashMap::clone(&self.secret_string_characters);

        // go through the copy's entries
        for (key, value) in copy {
            let mut new_befores: Vec<char> = Vec::new();
            // find 
            for character in value.before {
                // get the entry of this character's before values, and find their
                // before values to append to the other character
                match self.secret_string_characters.get(&character) {
                    Some(list) => new_befores.append(&mut list.before.iter().map(|character| *character).collect::<Vec<char>>()),
                    None => {}
                }
            }

            // if the new befores list isn't empty, add the new befores
            if !new_befores.is_empty(){
                //this or_insert should never happen
                let entry = self.secret_string_characters.entry(key).or_insert(SecretStringCharacter::new('z'));
                entry.add_multiple_following_chars(new_befores);
            }
        }

        while self.get_lengths_list().len() != self.secret_string_characters.len() {
            self.add_dependent_followers();
        }
    }

    pub fn print_final(&self) -> String {
        (0..self.secret_string_characters.len()).rev().map(|no_following| {
            for (_, character) in self.secret_string_characters.iter() {
                if character.before.len() == no_following {
                    return character.c.to_string();
                }
            }
            "".to_string()
        }).collect::<Vec<String>>().join("").to_string()
    }
}

impl SecretStringCharacter {
    pub fn new(c: char) -> SecretStringCharacter {
        SecretStringCharacter {
            c,
            before: Vec::new(),
        }
    }

    pub fn add_following_char(&mut self, c: char) {
        if !self.before.contains(&c) {
            self.before.push(c);
        }
    }

    pub fn add_multiple_following_chars(&mut self, chars: Vec<char>){
        for character in chars {
            self.add_following_char(character);
        }
    }
}

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut secret_string = SecretString::new();

    for triplet in &triplets {
        for character in triplet.iter() {
            secret_string.add_char(character);
        }
    }

    for triplet in triplets {
        // add following chars for the first in the array
        secret_string.add_following_to_char(triplet[0], triplet[1]);
        secret_string.add_following_to_char(triplet[0], triplet[2]);
        // add following char for the second in the array
        secret_string.add_following_to_char(triplet[1], triplet[2]);
    }

    secret_string.add_dependent_followers();

    secret_string.print_final()
}
