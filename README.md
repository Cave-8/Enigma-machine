# Enigma-machine
An Enigma machine simulator built in Rust.

# Configuration
The machine is completely configurable from texts/config.<br>
Rotors are identified with ROTL(eft), ROTC(enter), ROTR(ight): implicitly you're binding each letter with the traditional alphabet.<br>
For instance, in the following file example ROTL, you're binding A->E, B->K ecc...<br>
Then you'll have to write orientation of the rotors from left to right.<br>
If you want to use plugboard simply digit couples of exchanged letter, if not you can leave empty.<br>
Finally, you must type couples of letters reflected by reflector, separated by a whitespace.<br>
An example of accepted file is the following:<br>

START_CONFIG<br>
ROTL: EKMFLGDQVZNTOWYHXUSPAIBRCJ<br>
ROTC: AJDKSIRUXBLHWTMCQGZNPYFVOE<br>
ROTR: BDFHJLCPRTXVZNYEIWGAKMUSQO<br>
ORIENTATION: EAB<br>
PB: AB CD EF<br>
REFL: AY BR CU DH EQ FS GL IP JX KN MO TZ VW<br>
END_CONFIG<br>

# How to encrypt/decrypt
Compile the main with: `cargo run --release ./path/main.rs`.<br>
The machine will read from input and print on output files located in texts folder.<br>
Machine configuration is read from config file in texts folder.<br>
Accepted characters are: [A-Za-z] including (\r)\n.<br>
Due to design choices encrypted/decrypted texts will be in uppercase only.

# Benchmark
Some benchmarks (to correctly execute benchmark mode insert proper files in texts folder):
* Divine Comedy (about 590000 characters) -> ca. 257ms
* In Search of Lost Time (1st volume) (about 977000 characters) -> ca. 434ms
  
These results heavily depends on your machine, this implementation of Enigma is single-threaded and could be optimized.

# Side notes
Some choices were made during creation:
- This Enigma machine replica is based on three rotors variant of Enigma machine.
- Chosen alphabet is the english/german one with uppercase letters only (you can type in lowercase but output file will be in uppercase only).
- This replica has the exact flaws of original Enigma machine, so a letter will never be encrypted to itself (due to reflector).
