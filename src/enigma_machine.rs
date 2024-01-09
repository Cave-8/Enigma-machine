use std::fs::{File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
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
	/// Setup Enigma, path is config.txt path
	pub fn enigma_setup(&mut self, path: String) {
		let reader = BufReader::new(File::open(path).unwrap());
		let lines = reader.lines();

		for l in lines {
			let r: String = l.unwrap()
			                 .to_uppercase()
			                 .trim()
			                 .to_string();
			let assign_op = |r: &String| {
				let mut to_ret = 0;
				if r.contains("ROTL") { to_ret = 0; }
				else if r.contains("ROTC") { to_ret = 1; }
				else if r.contains("ROTR") { to_ret = 2; }
				else if r.contains("ORIENTATION") { to_ret = 3; }
				else if r.contains("PB") { to_ret = 4; }
				else if r.contains("REFL") { to_ret = 5; }
				else { to_ret = 6; }
				return to_ret;
			};
			let op = assign_op(&r);

			match op {
				0 | 1 | 2 => {
					self.rotors[op].setup_rotor_wiring(&r.split(':')
					                                     .map(|x| x.to_string())
					                                     .collect::<Vec<String>>()
					                                     .get(1)
					                                     .unwrap()
					                                     .to_string()
					                                     .trim()
					                                     .parse()
					                                     .unwrap());
					self.rotors[op].setup_rotor_list_of_characters(&r.split(':')
					                                                 .map(|x| x.to_string())
					                                                 .collect::<Vec<String>>()
					                                                 .get(1)
					                                                 .unwrap()
					                                                 .to_string()
					                                                 .trim()
					                                                 .parse()
					                                                 .unwrap());
				}
				3 => {
					let orientation: String = r.split(':')
					                           .map(|x| x.to_string())
					                           .collect::<Vec<String>>()
					                           .get(1)
					                           .unwrap()
					                           .to_string()
					                           .trim()
					                           .parse()
					                           .unwrap();
					self.rotors[0].setup_rotor_starting_character(orientation.chars().nth(0).unwrap());
					self.rotors[1].setup_rotor_starting_character(orientation.chars().nth(1).unwrap());
					self.rotors[2].setup_rotor_starting_character(orientation.chars().nth(2).unwrap());
				}
				4 => {
					self.plugboard.setup_plugboard(&r.split(':')
					                                 .map(|x| x.to_string())
					                                 .collect::<Vec<String>>()
					                                 .get(1)
					                                 .unwrap()
					                                 .to_string()
					                                 .trim()
					                                 .parse()
					                                 .unwrap());
				}
				5 => {
					self.reflector.setup_reflector(&r.split(':')
					                                 .map(|x| x.to_string())
					                                 .collect::<Vec<String>>()
					                                 .get(1)
					                                 .unwrap()
					                                 .to_string()
					                                 .trim()
					                                 .parse()
					                                 .unwrap());
				}
				_ => (),
			}
		}
	}

	/// Reset enigma
	pub fn enigma_reset(&mut self, path: String) {
		self.enigma_setup(path);
	}

	/// Encrypt/Decrypt file
	pub fn enigma_routine(&mut self, path_input: String, path_output: String) {
		let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
		let mut reader_input = BufReader::new(File::open(path_input).unwrap());
		let mut text: String = Default::default();
		let _ = reader_input.read_to_string(&mut text);
		let mut writer_output = BufWriter::new(File::create(path_output).unwrap());

		let mut encrypted_line: String = Default::default();
		text = text.to_uppercase();

		for c in text.chars() {
			if alphabet.contains(c) {
				encrypted_line.push_str(Rotor::rotor_routine(c, &self.plugboard, &mut self.rotors, &self.reflector, &self.input_wheel).to_string().as_ref())
			} else {
				encrypted_line.push_str(c.to_string().as_ref());
			}
		}
		writer_output.write_all(encrypted_line.as_ref()).unwrap();
		writer_output.flush().unwrap();
	}

	/// Print configuration (useful for debugging)
	fn enigma_print_configuration(&mut self) {
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