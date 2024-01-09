use std::io;
use std::io::{stdout, Write};
use std::time::Instant;
use colored::Colorize;
use Enigma::enigma_machine::EnigmaMachine;

fn main() {
    println!("Select mode (0 simple mode, 1 benchmark mode):");
    print!("> ");
    stdout().flush().expect("Error in flushing");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let choice: i32 = input_line.trim().parse().expect("Input not an integer");

    match choice {
        0 => {
            let mut enigma_machine: EnigmaMachine = Default::default();

            println!("{}", "Configuring the machine...".cyan());
            enigma_machine.enigma_setup("./src/texts/config.txt".to_string());
            println!("{}", "Encrypting/Decrypting file...".yellow());
            enigma_machine.enigma_routine("./src/texts/input.txt".to_string(), "./src/texts/output.txt".to_string());
            println!("{}", "Done".green());
        }
        1 => {
            let mut enigma_machine: EnigmaMachine = Default::default();
            let now = Instant::now();

            println!("{}", "Configuring the machine...".cyan());
            enigma_machine.enigma_setup("./src/texts/config.txt".to_string());
            println!("{}", "Encrypting/Decrypting file...".yellow());
            enigma_machine.enigma_routine("./src/texts/divine_comedy.txt".to_string(), "./src/texts/divine_comedy_encrypted.txt".to_string());

            println!("Time to encrypt: {:?}", now.elapsed());

            drop(enigma_machine);

            let now = Instant::now();
            let mut enigma_machine: EnigmaMachine = Default::default();
            enigma_machine.enigma_setup("./src/texts/config.txt".to_string());
            enigma_machine.enigma_routine("./src/texts/divine_comedy_encrypted.txt".to_string(), "./src/texts/divine_comedy.txt".to_string());
            println!("Time to decrypt: {:?}", now.elapsed());

            println!("{}", "Done I benchmark".green());

            drop(enigma_machine);

            let mut enigma_machine: EnigmaMachine = Default::default();
            let now = Instant::now();

            println!("{}", "Configuring the machine...".cyan());
            enigma_machine.enigma_setup("./src/texts/config.txt".to_string());
            println!("{}", "Encrypting/Decrypting file...".yellow());
            enigma_machine.enigma_routine("./src/texts/recherche.txt".to_string(), "./src/texts/recherche_encrypted.txt".to_string());

            println!("Time to encrypt: {:?}", now.elapsed());

            drop(enigma_machine);

            let now = Instant::now();
            let mut enigma_machine: EnigmaMachine = Default::default();
            enigma_machine.enigma_setup("./src/texts/config.txt".to_string());
            enigma_machine.enigma_routine("./src/texts/recherche_encrypted.txt".to_string(), "./src/texts/recherche.txt".to_string());
            println!("Time to decrypt: {:?}", now.elapsed());

            println!("{}", "Done II benchmark".green());
        }
        _ => {
            println!("Unrecognized mode.");
        }
    }
}
