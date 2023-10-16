use colored::Colorize;
use Enigma::enigma_machine::EnigmaMachine;

fn main() {
    let mut enigma_machine: EnigmaMachine = Default::default();
    println!("{}", "Enigma machine configuration".cyan());
    enigma_machine.enigma_setup("./src/texts/config".to_string());
    //enigma_machine.enigma_encrypt("./src/texts/input".to_string(), "./src/texts/output".to_string());
    enigma_machine.enigma_decrypt("./src/texts/input".to_string(), "./src/texts/output".to_string());
}
