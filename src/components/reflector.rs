/// Reflector of enigma machine.
///
/// Reflector is a vector of char containing mapping similar to what is done in plugboard.
pub struct Reflector {
    reflector: Vec<char>,
}

impl Reflector {
    /// Setup of reflector with appropriate char vector.
    pub fn setup_reflector(&mut self, reflector: &String) {
        if reflector.len() != 26 {
            panic!("Error, plugboard vector length is not 26!");
        }
        self.reflector = reflector.chars().collect();
    }

    /// Used to get substituted letter in reflected Enigma step.
    pub fn get_letter(&self, letter: char) -> char {
        let letter_as_ascii = letter as u32;
        let letter_index: usize = (letter_as_ascii - 65) as usize;
        return *self.reflector.get(letter_index).unwrap();
    }

    /// Getter
    pub fn reflector (&self) -> &Vec<char> {
        &self.reflector
    }
}

impl Default for Reflector {
    /// Initialize reflector with basic alphabet.
    fn default() -> Self {
        Self {
            reflector: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
        }
    }
}