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
    /// Given a letter from l_side it returns corresponding letter to r_side.
    ///
    /// It also rotates rotor, check num_of_rotations and eventually reset it.
    ///
    /// Rotation in Enigma happens when key is pressed.
    pub fn rotor_routine (&mut self, letter: char) -> &char {
        self.num_of_rotation += 1;
        if self.num_of_rotation == 26 {
            // TODO Rotate following rotor?
            self.num_of_rotation = 0;
        }
        self.r_side.rotate_left(1);

        let l_index = self.l_side.iter().position(|x| *x == letter).unwrap();
        return self.r_side.get(l_index).unwrap();
    }

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