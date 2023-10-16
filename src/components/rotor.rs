use std::collections::HashMap;
use crate::components::reflector::Reflector;

/// Rotor of enigma machine:
///
/// num_of_rotation quantifies how many rotations the rotor did until that moment (0 <= ... < 26).
///
/// wiring is rotor wiring.
///
/// starting_character is character shown at the beginning of encryption
///
/// list_of_characters is the rotor rim
pub struct Rotor {
    wiring: HashMap<char, char>,
    list_of_characters: String,
    starting_character: char,
    num_of_rotation: usize,
}

impl Rotor {
    /// Setup of rotor internal wiring.
    pub fn setup_rotor_wiring(&mut self, wiring: &String) {
        if wiring.len() != 38 {
            panic!("Error, reflector doesn't have 13 required couples!");
        }
        let couples: Vec<String> = wiring.split(' ').map(|x| x.to_string()).collect();
        // Reset map
        self.wiring.remove(&'A');

        for c in couples {
            let char1: char = c.chars().nth(0).unwrap();
            let char2: char = c.chars().nth(1).unwrap();

            // Double insertion is done to speed up lookup
            self.wiring.insert(char1, char2);
            self.wiring.insert(char2, char1);
        }
    }

    /// Setup of rotor rim.
    pub fn setup_rotor_list_of_characters(&mut self, list_of_chars: String) {
        self.list_of_characters = list_of_chars;
    }

    /// Setup of starting character.
    pub fn setup_rotor_starting_character(&mut self, starting_char: char) {
        self.starting_character = starting_char;
    }

    /// Given a letter it returns corresponding encoded letter.
    ///
    /// It also rotates rotor, check num_of_rotations for each rotor and eventually reset it.
    ///
    /// Rotation in Enigma happens when key is pressed.
    pub fn rotor_routine (letter: char, rotors: &mut Vec<Rotor>, reflector: &Reflector) -> char {
        let mut curr_letter = letter;

        // Flags to make rotor 0 and 1 rotate if necessary
        let mut rotor2_full_rotation = false;
        let mut rotor1_full_rotation = false;

        curr_letter = rotors[2].get_letter(curr_letter);
        rotors[2].num_of_rotation += 1;
        if rotors[2].num_of_rotation == 26 {
            rotors[2].num_of_rotation = 0;
            rotor2_full_rotation = true;
        }
        else {
            if curr_letter == 'Z' {curr_letter = 'A'} else {
                let mut ascii_letter = curr_letter as u8;
                ascii_letter += 1;
                curr_letter = ascii_letter as char;
            }
        }

        curr_letter = rotors[1].get_letter(curr_letter);
        if rotor2_full_rotation {
            rotors[1].num_of_rotation += 1;
            if rotors[1].num_of_rotation == 26 {
                rotors[1].num_of_rotation = 0;
                rotor1_full_rotation = true;
            }
            else {
                if curr_letter == 'Z' {curr_letter = 'A'} else {
                    let mut ascii_letter = curr_letter as u8;
                    ascii_letter += 1;
                    curr_letter = ascii_letter as char;
                }
            }
        }

        curr_letter = rotors[0].get_letter(curr_letter);
        if rotor1_full_rotation {
            rotors[0].num_of_rotation += 1;
            if rotors[0].num_of_rotation == 26 {
                rotors[0].num_of_rotation = 0;
            }
            else {
                if curr_letter == 'Z' {curr_letter = 'A'} else {
                    let mut ascii_letter = curr_letter as u8;
                    ascii_letter += 1;
                    curr_letter = ascii_letter as char;
                }
            }
        }

        // Reflector
        curr_letter = reflector.get_letter(curr_letter);

        // Reverse path
        curr_letter = rotors[0].get_letter(curr_letter);
        curr_letter = rotors[1].get_letter(curr_letter);
        curr_letter = rotors[2].get_letter(curr_letter);

        return curr_letter;
    }

    /// Given letter it returns linked letter
    pub fn get_letter(&self, letter: char) -> char {
        return *self.wiring.get(&letter).unwrap();
    }
    /// Getter
    pub fn wiring (&mut self) -> &HashMap<char, char> {
        &self.wiring
    }
    /// Getter
    pub fn num_of_rotation (&mut self) -> &usize{
        &self.num_of_rotation
    }
    /// Getter
    pub fn list_of_characters (&mut self) -> &String {
        &self.list_of_characters
    }
    /// Getter
    pub fn starting_character (&mut self) -> &char {
        &self.starting_character
    }
}

impl Default for Rotor {
    /// Initialize l_side and r_side with basic alphabet and num_of_rotation with 0
    fn default() -> Self {
        Self {
            wiring: [('A', 'B')].iter().cloned().collect(),
            list_of_characters: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
            starting_character: 'A',
            num_of_rotation: 0,
        }
    }
}