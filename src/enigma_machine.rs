/// Enigma machine
/// It contains a plugboard, three rotors, a deflector
use crate::components as comp;

pub struct EnigmaMachine {
    plugboard: comp::plugboard::Plugboard,
    rotors: Vec<comp::rotor::Rotor>,
    reflector: comp::reflector::Reflector,
}

impl EnigmaMachine {

    /// Setup enigma
    fn enigma_setup(&mut self) {
        // TODO parse config file
        // self.plugboard.setup_plugboard()
    }

    /// Reset enigma
    fn enigma_reset(&mut self) {

    }

    /// Encrypt file
    fn enigma_encrypt (&mut self) {

    }

    /// Decrypt file
    fn enigma_decrypt (&mut self) {

    }
}