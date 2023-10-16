use crate::components::reflector::Reflector;

/// Rotor of enigma machine.
///
/// Field: l_side and r_side represent corresponding elements on original rotor, to simulate physical wiring
/// two char vectors are used, corresponding indexes have corresponding character (I.E: if A and X have
/// same index in two vectors than A is bound to X).
///
/// Field: num_of_rotation quantifies how many rotations the rotor did until that moment (0 <= ... < 26).
/// r_side is rotating side, the rotation is simulated by rotating string one step to the left.
pub struct Rotor {
    l_side: Vec<char>,
    r_side: Vec<char>,
    num_of_rotation: usize,
}

impl Rotor {
    /// Setup of rotor l/r side.
    pub fn setup_rotor(&mut self, l_side: &String, r_side: &String) {
        if l_side.len() != 26 || r_side.len() !=  26 {
            panic!("Error, l_side or r_side lengths is not 26!");
        }
        self.l_side = l_side.chars().collect();
        self.r_side = r_side.chars().collect();
    }

    // TODO check rotation
    /// Given a letter it returns corresponding encoded letter.
    ///
    /// It also rotates rotor, check num_of_rotations for each rotor and eventually reset it.
    ///
    /// Rotation in Enigma happens when key is pressed.
    pub fn rotor_routine (letter: char, rotors: &mut Vec<Rotor>, reflector: &Reflector) -> char {
        let mut curr_letter = 'a';

        // Rightmost rotor
        rotors[2].shift_rotor();
        curr_letter = rotors[2].get_letter_l_to_r(letter);
        if rotors[2].num_of_rotation == 26 {
            rotors[2].num_of_rotation = 0;
            rotors[1].num_of_rotation += 1;
            rotors[1].shift_rotor();
        }
        else {
            rotors[2].num_of_rotation += 1;
        }

        // Middle rotor
        if rotors[1].num_of_rotation == 26 {
            rotors[1].num_of_rotation = 0;
            rotors[0].num_of_rotation += 1;
            rotors[0].shift_rotor();
        }
        curr_letter = rotors[1].get_letter_l_to_r(curr_letter);

        // Leftmost rotor
        if rotors[0].num_of_rotation == 26 {
            rotors[0].num_of_rotation = 0;
        }
        curr_letter = rotors[0].get_letter_l_to_r(curr_letter);

        // Reflector
        curr_letter = reflector.get_letter(curr_letter);

        // Reverse path
        curr_letter = rotors[0].get_letter_r_to_l(curr_letter);
        curr_letter = rotors[1].get_letter_r_to_l(curr_letter);
        curr_letter = rotors[2].get_letter_r_to_l(curr_letter);

        return curr_letter;
    }

    /// Given letter from l_side it returns letter from r_side
    pub fn get_letter_l_to_r (&mut self, letter: char) -> char {
        let letter_as_ascii = letter as u32;
        let letter_index: usize = (letter_as_ascii - 65) as usize;
        return *self.r_side.get(letter_index).unwrap();
    }

    /// Given letter from r_side it returns letter from l_side
    pub fn get_letter_r_to_l (&mut self, letter: char) -> char {
        let letter_as_ascii = letter as u32;
        let letter_index: usize = (letter_as_ascii - 65) as usize;
        return *self.l_side.get(letter_index).unwrap();
    }

    /// Shift rotor of one position
    pub fn shift_rotor (&mut self) {&self.r_side.rotate_left(1);}
    /// Getter
    pub fn l_side (&mut self) -> &Vec<char> {
        &self.l_side
    }
    /// Getter
    pub fn r_side (&mut self) -> &Vec<char> {
        &self.r_side
    }
    /// Getter
    pub fn num_of_rotation (&mut self) -> &usize{
        &self.num_of_rotation
    }
}

impl Default for Rotor {
    /// Initialize l_side and r_side with basic alphabet and num_of_rotation with 0
    fn default() -> Self {
        Self {
            l_side: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            r_side: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            num_of_rotation: 0,
        }
    }
}