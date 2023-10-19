use std::collections::HashMap;
use colored::Color::Magenta;
use crate::components::plugboard::Plugboard;
use crate::components::reflector::Reflector;

/// Rotor of enigma machine:
///
/// num_of_rotation: quantifies how many rotations the rotor did until that moment (0 <= ... < 26).
///
/// wiring: is rotor internal wiring.
///
/// starting_character: is character shown at the beginning of encryption
///
/// list_of_characters: is the rotor rim
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
    ///
    /// It also rotates list_of_characters according to it
    pub fn setup_rotor_starting_character(&mut self, starting_char: char) {
        self.starting_character = starting_char;

        let parts: Vec<String> = self.list_of_characters.splitn(2, self.starting_character).map(|x| x.to_string()).collect();
        let mut first_part = self.starting_character.to_string();
        first_part.push_str(parts.get(1).unwrap());
        first_part.push_str(parts.get(0).unwrap());

        self.list_of_characters = first_part;
    }

    /// Rotate rotor of one position.
    pub fn rotate_rotor(&mut self) {
        let mut rotated_string: Vec<char> = self.list_of_characters.chars().collect();
        rotated_string.rotate_right(1);
        self.list_of_characters = rotated_string.iter().collect();
    }

    /// Return correspondent letter in following rotor (or from/to entry wheel (0) and reflector (4))
    pub fn get_letter_in_next_rotor (letter: char, rotors: &mut Vec<Rotor>, starting_rotor: isize, ending_rotor: isize, input_wheel: &String, reflector: &Reflector) -> char {
        //From input wheel to first rotor
        if starting_rotor == -1 {
            let index = input_wheel.chars().position(|x| x == letter).unwrap();
            let char = rotors[2].list_of_characters.chars().nth(index).unwrap();
            return char;
        }
        // Between rotors
        else if starting_rotor != -1 && ending_rotor != 4 && starting_rotor != 4 && ending_rotor != -1 {
            let s_index = starting_rotor as usize;
            let e_index = ending_rotor as usize;
            let index = rotors[s_index].list_of_characters().chars().position(|x| x == letter).unwrap();
            rotors[e_index].list_of_characters.chars().nth(index).unwrap()
        }
        // From last rotor to reflector
        else if ending_rotor == 4 {
            let index = rotors[0].list_of_characters().chars().position(|x| x == letter).unwrap();
            reflector.list_of_characters().chars().nth(index).unwrap()
        }
        // From reflector to last rotor
        else if starting_rotor == 4 {
            let index = reflector.list_of_characters().chars().position(|x| x == letter).unwrap();
            rotors[0].list_of_characters().chars().nth(index).unwrap()
        }
        // From first rotor to input wheel
        else {
            let index = rotors[2].list_of_characters().chars().position(|x| x == letter).unwrap();
            input_wheel.chars().nth(index).unwrap()
        }
    }

    /// Given a letter it returns corresponding encoded letter.
    ///
    /// It also rotates rotor, check num_of_rotations for each rotor and eventually reset it.
    ///
    /// Rotation in Enigma happens when key is pressed.
    pub fn rotor_routine (letter: char, plugboard: &Plugboard, rotors: &mut Vec<Rotor>, reflector: &Reflector, input_wheel: &String) -> char {
        let mut curr_letter = letter;
        let mut rotor1rotated = false;
        let mut rotor0rotated = false;

        println!("-------------------------------");
        println!("Starting letter {}", curr_letter);


        /*rotors[2].rotate_rotor();
        if rotors[2].num_of_rotation == 26 {
            rotors[2].num_of_rotation = 0;
            rotors[1].rotate_rotor();
            rotor1rotated = true;
        }
        else {
            rotors[2].num_of_rotation += 1;
        }*/

        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, -1, 2, input_wheel, reflector);
        curr_letter = rotors[2].get_letter(curr_letter);
        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, 2, 1, input_wheel, reflector);
        curr_letter = rotors[1].get_letter(curr_letter);
        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, 1, 0, input_wheel, reflector);
        curr_letter = rotors[0].get_letter(curr_letter);
        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, 0, 4, input_wheel, reflector);
        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, 4, 0, input_wheel, reflector);
        curr_letter = rotors[0].get_letter(curr_letter);
        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, 0, 1, input_wheel, reflector);
        curr_letter = rotors[1].get_letter(curr_letter);
        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, 1, 2, input_wheel, reflector);
        curr_letter = rotors[2].get_letter(curr_letter);
        curr_letter = Rotor::get_letter_in_next_rotor(curr_letter, rotors, 2, 4, input_wheel, reflector);


        // Exchange letter with plugboard
        curr_letter = plugboard.get_letter(curr_letter);
        println!("Final letter {}", curr_letter);
        println!("-------------------------------");
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