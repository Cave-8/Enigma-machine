# Enigma-machine
An enigma machine simulator built in Rust.

# Configuration
The machine is completely configurable from texts/config.<br>
Rotors are listed from 1 (leftmost) to 3 (rightmost) and for each rotor you'll have to define internal wiring with couples, defined using german/english alphabet, separated by a space (AB CD... to map A with B, C with D, ecc...).<br><br>
After rotors wiring you'll need to define order of characters on rotor's rim.
Then you'll write rotor orientation (shown letters in small windows located on upper part of Enigma, from left to right).<br><br>
Finally, you'll have to define plugboard and reflector bindings with the same logic of rotor wiring.<br>
Config file starts with START_CONFIG and ends with END_CONFIG.<br>
An example of accepted syntax is the following:<br>

START_CONFIG<br>
R1: AB CD EF GH IJ KL MN OP QR ST UV WX YZ<br>
R2: AB CD EF GH IJ KL MN OP QR ST UV WX YZ<br>
R3: AB CD EF GH IJ KL MN OP QR ST UV WX YZ<br>
RIM1: ABCDEFGHIJKLMNOPQRSTUVWXYZ<br>
RIM2: ABCDEFGHIJKLMNOPQRSTUVWXYZ<br>
RIM3: ABCDEFGHIJKLMNOPQRSTUVWXYZ<br>
ORIENTATION: ABC
PL: AB CD EF GH IJ KL MN OP QR ST UV WX YZ<br>
REFL: AB CD EF GH IJ KL MN OP QR ST UV WX YZ<br>
END_CONFIG<br>

# How to encrypt/decrypt
Compile the main with: `cargo run ./path/main.rs`.<br>
Selected desired mode, the machine will read from input and print on output files located in texts folder.<br>
Machine configuration is read from config file in texts folder.<br>
Accepted characters are: [A-Za-z] including (\r)\n.<br>
Due to design choices encrypted/decrypted texts will be in uppercase only.

# Side notes
Some choices were made during creation:
- This Enigma machine replica is based on three rotors variant of Enigma machine.
- Chosen alphabet is the english/german one with uppercase letters only.
- This replica has the exact flaws of original Enigma machine, so a letter will never be encrypted to itself (due to reflector).