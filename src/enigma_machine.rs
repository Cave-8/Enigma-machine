use std::fs::File;
use std::io::{BufRead, BufReader};
/// Enigma machine
/// It contains a plugboard, three rotors, a deflector
use crate::components as comp;

pub struct EnigmaMachine {
    plugboard: comp::plugboard::Plugboard,
    rotors: Vec<comp::rotor::Rotor>,
    reflector: comp::reflector::Reflector,
}

impl EnigmaMachine {

    /// Setup Enigma, path is config path
    fn enigma_setup(&mut self, path: String) {
        // TODO parse texts file
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let mut plugboard: String = Default::default();
        let mut r1r: String = Default::default();
        let mut r2r: String = Default::default();
        let mut r3r: String = Default::default();

        let reader = BufReader::new(File::open("./src/texts/config").unwrap());
        let lines = reader.lines();

        //TODO
        for l in lines {
            let r = l.unwrap().to_uppercase().trim();
            if r.contains("END_CONFIG") {
                break;
            }
            if r.contains("R1R") {

            }
            if r.contains("R2R") {

            }
            if r.contains("R3R") {

            }
        }
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