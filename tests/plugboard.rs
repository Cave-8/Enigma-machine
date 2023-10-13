/// Test plugboard
use Enigma::components::plugboard::Plugboard;

/// Test if substitution vector is correctly defined
#[test]
#[should_panic]
fn plugboard_test_1 () {
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