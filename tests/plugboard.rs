/// Test plugboard
use Enigma::components::plugboard::Plugboard;

/// Test if letter and substitution_vector are correctly bound
#[test]
fn plugboard_get_letter_test () {
    let sub_vec = "AB CD EF GH IJ KL MN OP QR ST UV WX YZ".to_string();
    let alphabet = "ACEGIKMOQSUWY".to_string();
    let mut i: usize = 0;
    let mut plugboard: Plugboard = Default::default();

    plugboard.setup_plugboard(&sub_vec);

    for c in alphabet.chars() {
        let el_ascii = plugboard.get_letter(c) as usize;
        assert_eq!(el_ascii - 65, c);
        i += 1;
    }
}