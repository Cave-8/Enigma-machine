# Enigma-machine
An enigma machine simulator built in Rust.

# Configuration
The machine is completely configurable from texts/config.<br>
Rotors are listed from 1 (leftmost) to 3 (rightmost) and for each rotor you'll have to define r_side bindings with german/english alphabet (input a string, the first character will replace A, the second B ecc...).<br>
After rotors you'll have to define plugboard bindings with the same logic.<br>
Config file starts with START_CONFIG and ends with END_CONFIG.<br>
An example of accepted syntax is the following:<br>

START_CONFIG<br>
R1L: ABCDEFGHIJKLMNOPQRSTUVWXYZ<br>
R1R: BCDEFGHIJKLMNOPQRSTUVWXYZA<br>
R2L: ABCDEFGHIJKLMNOPQRSTUVWXYZ<br>
R2R: DEFGHIJKLMNOPQRSTUVWXYZABC<br>
R3L: ABCDEFGHIJKLMNOPQRSTUVWXYZ<br>
R3R: FGHIJKLMNOPQRSTUVWXYZABCDE<br>
P1: ABCDEFGHIJKLMNOPQRSTUVWXYZ<br>
P2: BCDEFGHIJKLMNOPQRSTUVWXYZA<br>
END_CONFIG<br>

# How to encrypt/decrypt
Compile the main with: `cargo run ./path/main.rs`.<br>
Selected desired mode, the machine will read from input and print on output files located in texts folder.<br>
Machine configuration is read from config file in texts folder.<br>
Accepted characters are: [A-Za-z] including (\r)\n.<br>
Due to design choices encrypted/decrypted texts will be in uppercase only<br>.

# Side notes
Some choices were made during creation:
- This Enigma machine replica is based on three rotors variant of Enigma machine.
- Chosen alphabet is the english/german one with uppercase letters only.
- This replica has the exact flaws of original Enigma machine, so a letter will never be encrypted to itself (due to reflector).