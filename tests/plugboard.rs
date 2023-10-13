/// Test plugboard
use Enigma::components::plugboard::Plugboard;

/// Test if substitution vector is correctly defined
/// Note: this test uses only shifter cipher to avoid hard-coding tests and make testing easier,
/// in reality plugboard won't have a simple shifter substitution cipher.
#[test]
#[should_panic]
fn plugboard_setup_test () {
    let mut sub_vec: String = "BCDEFGHIJKLMNOPQRSTUVWXYZA".to_string();
    // Shift from common alphabet
    let mut shift = 1;
    let mut plugboard: Plugboard = Default::default();

    plugboard.setup_plugboard(sub_vec);

    for i in 0..26 {
        let curr_el = *plugboard.substitution_vector().get(i).unwrap() as usize;
        assert_eq!(curr_el - 65, (i + shift).rem_euclid(26))
    }

    sub_vec = "CDEFGHIJKLMNOPQRSTUVWXYZAB".to_string();
    shift = 2;
    plugboard.setup_plugboard(sub_vec);

    for i in 0..26 {
        let curr_el = *plugboard.substitution_vector().get(i).unwrap() as usize;
        assert_eq!(curr_el - 65, (i + shift).rem_euclid(26))
    }

    // sub_vec length is 25
    sub_vec = "CDEFGHIJKLMNOPQRSTUVWXYZA".to_string();
    shift = 2;
    plugboard.setup_plugboard(sub_vec);
}

/// Test if letter and substitution_vector are correctly bound
#[test]
fn plugboard_get_letter_test () {
    let sub_vec = "BCDEFGHIJKLMNOPQRSTUVWXYZA".to_string();
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let shift: usize = 1;
    let mut i: usize = 0;
    let mut plugboard: Plugboard = Default::default();

    plugboard.setup_plugboard(sub_vec);

    for c in alphabet.chars() {
        let el_ascii = plugboard.get_letter(c) as usize;
        assert_eq!(el_ascii - 65, (i + shift).rem_euclid(26));
        i += 1;
    }

}