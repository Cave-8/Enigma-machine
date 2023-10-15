/// This module represents plug-board, the first cypher step in original enigma.
///
/// Plugboard module and struct, containing substitution_vector.
pub struct Plugboard {
    substitution_vector: Vec<char>,
}

impl Plugboard {
    /// Setup of plugboard with appropriate substitution vector.
    pub fn setup_plugboard(&mut self, sub_vec: &String) {
        if sub_vec.len() != 26 {
            panic!("Error, plugboard vector length is not 26!");
        }
        self.substitution_vector = sub_vec.chars().collect();
    }

    /// Used to get substituted letter in first Enigma step.
    pub fn get_letter(&self, letter: char) -> char {
        let letter_as_ascii = letter as u32;
        let letter_index: usize = (letter_as_ascii - 65) as usize;
        return *self.substitution_vector.get(letter_index).unwrap();
    }

    /// Getter
    pub fn substitution_vector (&self) -> &Vec<char> {
        &self.substitution_vector
    }
}

impl Default for Plugboard {
    /// Initialize plugboard with basic alphabet.
    fn default() -> Self {
        Self {
            substitution_vector: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        }
    }
}