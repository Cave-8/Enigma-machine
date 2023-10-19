use std::fs::{File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use colored::Colorize;
/// Enigma machine
/// It contains a plugboard, three rotors, a deflector
use crate::components::plugboard::Plugboard;
use crate::components::reflector::Reflector;
use crate::components::rotor::Rotor;

pub struct EnigmaMachine {
    plugboard: Plugboard,
    rotors: Vec<Rotor>,
    reflector: Reflector,
    input_wheel: String,
}

impl EnigmaMachine {

    /// Setup Enigma, path is config path
    pub fn enigma_setup(&mut self, path: String) {
        let reader = BufReader::new(File::open(path).unwrap());
        let lines = reader.lines();

        for l in lines {
            let r = l.unwrap().to_uppercase().trim().to_string();
            if r.contains("END_CONFIG") {
                break;
            }
            if r.contains("ROTL") {
                self.rotors[0].setup_rotor_wiring(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
                self.rotors[0].setup_rotor_list_of_characters(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("ROTC") {
                self.rotors[1].setup_rotor_wiring(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
                self.rotors[1].setup_rotor_list_of_characters(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("ROTR") {
                self.rotors[2].setup_rotor_wiring(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
                self.rotors[2].setup_rotor_list_of_characters(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("ORIENTATION") {
                let orientation: String = r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap();
                self.rotors[0].setup_rotor_starting_character(orientation.chars().nth(0).unwrap());
                self.rotors[1].setup_rotor_starting_character(orientation.chars().nth(1).unwrap());
                self.rotors[2].setup_rotor_starting_character(orientation.chars().nth(2).unwrap());
            }
            if r.contains("PB") {
                self.plugboard.setup_plugboard(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
            if r.contains("REFL") {
                self.reflector.setup_reflector(&r.split(':').map(|x| x.to_string()).collect::<Vec<String>>().get(1).unwrap().to_string().trim().parse().unwrap());
            }
        }
    }

    /// Reset enigma
    fn enigma_reset(&mut self, path: String) {
        self.enigma_setup(path);
    }

    /// Encrypt/Decrypt file
    pub fn enigma_routine (&mut self, path_input: String, path_output: String) {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
        let reader_input = BufReader::new(File::open(path_input).unwrap());
        let lines_input = reader_input.lines();
        let mut writer_output = BufWriter::new(File::create(path_output).unwrap());

        for l in lines_input {
            let uppercase_l = l.unwrap().to_uppercase();
            let curr_line = uppercase_l.chars();
            let mut encrypted_line: String = Default::default();
            for c in curr_line {
                if alphabet.contains(c) {
                    encrypted_line.push_str(Rotor::rotor_routine(c, &self.plugboard, &mut self.rotors, &self.reflector, &self.input_wheel).to_string().as_ref())
                }
                else {
                    encrypted_line.push_str(c.to_string().as_ref());
                }
            }
            encrypted_line.push_str("\r\n");
            writer_output.write_all(encrypted_line.as_ref()).unwrap();
            writer_output.flush().unwrap();
            println!("{}", "Done.".green())
        }
    }

    /// Print configuration (useful for debugging)
    fn enigma_print_configuration (&mut self) {
        print!("Plugboard: ");
        self.plugboard.substitution_vector().iter().for_each(|x| print!("{} : {} ", x.0, x.1));
        print!("\n");

        print!("RotorL: ");
        self.rotors[0].wiring().iter().for_each(|x| print!("{} : {} ", x.0, x.1));
        print!("\n");
        println!("{}", self.rotors[0].list_of_characters());

        print!("RotorC: ");
        self.rotors[1].wiring().iter().for_each(|x| print!("{} : {} ", x.0, x.1));
        print!("\n");
        println!("{}", self.rotors[1].list_of_characters());

        print!("RotorR: ");
        self.rotors[2].wiring().iter().for_each(|x| print!("{} : {} ", x.0, x.1));
        print!("\n");
        println!("{}", self.rotors[2].list_of_characters());

        let dummy_r1 = *self.rotors[0].starting_character();
        let dummy_r2 = *self.rotors[1].starting_character();
        let dummy_r3 = *self.rotors[2].starting_character();
        println!("Orientation {} - {} - {}", dummy_r1, dummy_r2, dummy_r3);

        print!("Reflector: ");
        self.reflector.reflector().iter().for_each(|x| print!("{} : {} ", x.0, x.1));
        print!("\n");
    }
}

impl Default for EnigmaMachine {
    /// Initialize plugboard with basic alphabet.
    fn default() -> Self {
        Self {
            plugboard: Default::default(),
            rotors: vec![Default::default(), Default::default(), Default::default()],
            reflector: Default::default(),
            input_wheel: "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        }
    }
}