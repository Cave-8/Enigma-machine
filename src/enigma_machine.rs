use std::fs::File;
use std::io::{BufRead, BufReader};
/// Enigma machine
/// It contains a plugboard, three rotors, a deflector
use crate::components::plugboard::Plugboard;
use crate::components::reflector::Reflector;
use crate::components::rotor::Rotor;

pub struct EnigmaMachine {
    plugboard: Plugboard,
    rotors: Vec<Rotor>,
    reflector: Reflector,
}

impl EnigmaMachine {

    /// Setup Enigma, path is config path
    pub fn enigma_setup(&mut self, path: String) {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let reader = BufReader::new(File::open(path).unwrap());
        let lines = reader.lines();

        for l in lines {
            let r = l.unwrap().to_uppercase().trim().to_string();
            if r.contains("END_CONFIG") {
                break;
            }
            if r.contains("R1") {
                self.rotors[0].setup_rotor(&alphabet, &r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("R2") {
                self.rotors[1].setup_rotor(&alphabet, &r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("R3") {
                self.rotors[2].setup_rotor(&alphabet, &r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("PB") {
                self.plugboard.setup_plugboard(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("REFL") {
                self.reflector.setup_reflector(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
        }
        self.enigma_print_configuration();
    }

    /// Reset enigma
    fn enigma_reset(&mut self, path: String) {
        self.enigma_setup(path);
    }

    /// Encrypt file
    fn enigma_encrypt (&mut self) {

    }

    /// Decrypt file
    fn enigma_decrypt (&mut self) {

    }

    /// Print configuration
    fn enigma_print_configuration (&mut self) {
        println!("Plugboard: {}", self.plugboard.substitution_vector().iter().collect::<String>());
        println!("Rotor1: {}", self.rotors[0].r_side().iter().collect::<String>());
        println!("Rotor2: {}", self.rotors[1].r_side().iter().collect::<String>());
        println!("Rotor3: {}", self.rotors[2].r_side().iter().collect::<String>());
        println!("Reflector: {}", self.reflector.reflector().iter().collect::<String>())
    }
}

impl Default for EnigmaMachine {
    /// Initialize plugboard with basic alphabet.
    fn default() -> Self {
        Self {
            plugboard: Default::default(),
            rotors: vec![Default::default(), Default::default(), Default::default()],
            reflector: Default::default(),
        }
    }
}