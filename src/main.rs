use colored::Colorize;
use Enigma::enigma_machine::EnigmaMachine;

fn main() {
    let mut enigma_machine: EnigmaMachine = Default::default();
    println!("{}", "Configuring the machine...".cyan());
    enigma_machine.enigma_setup("./src/texts/config".to_string());
    println!("{}", "Encrypting/Decrypting file...".yellow());
    enigma_machine.enigma_routine("./src/texts/input".to_string(), "./src/texts/output".to_string());
}
