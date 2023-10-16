use std::collections::HashMap;

/// Reflector of enigma machine.
///
/// Reflector is a hashmap of char pairs containing mapping between different letters
pub struct Reflector {
    reflector: HashMap<char, char>,
}

impl Reflector {
    /// Setup of reflector with appropriate chars mapping.
    pub fn setup_reflector(&mut self, reflector: &String) {
        if reflector.len() != 38 {
            panic!("Error, reflector doesn't have 13 required couples!");
        }
        let couples: Vec<String> = reflector.split(' ').map(|x| x.to_string()).collect();
        // Reset map
        self.reflector.remove(&'A');

        for c in couples {
            let char1: char = c.chars().nth(0).unwrap();
            let char2: char = c.chars().nth(1).unwrap();

            // Double insertion is done to speed up lookup
            self.reflector.insert(char1, char2);
            self.reflector.insert(char2, char1);
        }
    }

    /// Used to get substituted letter in reflected Enigma step.
    pub fn get_letter(&self, letter: char) -> char {
        return *self.reflector.get(&letter).unwrap();
    }

    /// Getter
    pub fn reflector (&self) -> &HashMap<char, char> {
        &self.reflector
    }
}

impl Default for Reflector {
    /// Initialize reflector with basic alphabet.
    fn default() -> Self {
        Self {
            reflector: [('A', 'B')].iter().cloned().collect()
        }
    }
}