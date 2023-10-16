use std::collections::HashMap;

/// This module represents plug-board, the first cypher step in original enigma.
///
/// Plugboard module and struct, containing substitution_vector.
pub struct Plugboard {
    sub_vec: HashMap<char, char>,
}

impl Plugboard {
    /// Setup of plugboard with appropriate substitution vector.
    pub fn setup_plugboard(&mut self, sub_vec: &String) {
        if sub_vec.len() != 38 {
            panic!("Error, reflector doesn't have 13 required couples!");
        }
        let couples: Vec<String> = sub_vec.split(' ').map(|x| x.to_string()).collect();

        // Reset map
        self.sub_vec.remove(&'A');

        for c in couples {
            let char1: char = c.chars().nth(0).unwrap();
            let char2: char = c.chars().nth(1).unwrap();

            // Double insertion is done to speed up lookup
            self.sub_vec.insert(char1, char2);
            self.sub_vec.insert(char2, char1);
        }
    }

    /// Used to get substituted letter in last Enigma step.
    pub fn get_letter(&self, letter: char) -> char {
        return *self.sub_vec.get(&letter).unwrap();
    }

    /// Getter
    pub fn substitution_vector (&self) -> &HashMap<char, char> {
        &self.sub_vec
    }
}

impl Default for Plugboard {
    /// Initialize plugboard with basic alphabet.
    fn default() -> Self {
        Self {
            sub_vec: [('A', 'B')].iter().cloned().collect()
        }
    }
}