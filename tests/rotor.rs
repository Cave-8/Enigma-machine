/// Test rotor
use Enigma::components::rotor::Rotor;

/// Test rotor routine with dummy l and r side.
#[test]
fn test_rotor_routine () {
    let mut rot: Rotor = Default::default();
    rot.setup_rotor("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());

    assert_eq!('B', *rot.rotor_routine('A'));
    assert_eq!('C', *rot.rotor_routine('A'));
    assert_eq!(2, *rot.num_of_rotation());
}