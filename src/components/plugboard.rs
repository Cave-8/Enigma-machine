/// This module represents plug-board, the first cypher step in original enigma
/// Plugboard module and struct, containing substitution_vector.
pub struct Plugboard {
    substitution_vector: String,
}

impl Plugboard {
    /// Setup of plugboard with appropriate substitution vector.
    pub fn setup_plugboard(&mut self, sub_vec: String) {
        if sub_vec.len() != 26 {
            panic!("Error, plugboard vector length is not 26!");
        }
        self.substitution_vector = sub_vec;
    }

    /// Used to get substituted letter in first Enigma step.
    pub fn get_letter(&self, letter: char) -> char {
        let vec_as_char = self.substitution_vector.chars().collect();
        let letter_as_ascii = letter as u32;
        let letter_index = letter_as_ascii - 65;
        return vec_as_char[letter_index];
    }
}