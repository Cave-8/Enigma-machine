use colored::Colorize;
use Enigma::enigma_machine::EnigmaMachine;

fn main() {
    let mut enigma_machine: EnigmaMachine = Default::default();
    println!("{}", "Enigma machine configuration".cyan());
    enigma_machine.enigma_setup("./src/texts/config".to_string());
}
