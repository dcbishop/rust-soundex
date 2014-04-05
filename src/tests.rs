extern crate collections;

use soundex::Soundex;

mod soundex;

#[test]
fn soundex_encoding_retains_sole_letter_of_one_letter_word() {
    let soundex = Soundex::new();
    let encoded = soundex.encode("A");
    assert_eq!(encoded, ~"A000");
}

#[test]
fn soundex_encoding_pads_with_zeros_to_ensure_three_digits() {
    let soundex = Soundex::new();
    let encoded = soundex.encode("I");
    assert_eq!(encoded, ~"I000");
}

#[test]
fn soundex_encoding_replaces_constants_with_appropriate_digits() {
    let soundex = Soundex::new();
    assert_eq!(soundex.encode("Ax"), ~"A200");
}

#[test]
fn soundex_encoding_ignores_non_alphabetics() {
    let soundex = Soundex::new();
    assert_eq!(soundex.encode("A#"), ~"A000");
}
