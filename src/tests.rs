extern crate collections;

use soundex::Soundex;

mod soundex;

fn check_soundex(word: ~str, expected: ~str) {
    let soundex = Soundex::new();
    assert_eq!(soundex.encode(word), expected);
}

#[test]
fn soundex_encoding_retains_sole_letter_of_one_letter_word() {
    check_soundex(~"A", ~"A000");
}

#[test]
fn soundex_encoding_pads_with_zeros_to_ensure_three_digits() {
    check_soundex(~"I", ~"I000");
}

#[test]
fn soundex_encoding_replaces_constants_with_appropriate_digits() {
    check_soundex(~"Ax", ~"A200");
}

#[test]
fn soundex_encoding_ignores_non_alphabetics() {
    check_soundex(~"A#", ~"A000");
}

#[test]
fn soundex_encoding_replaces_multiple_consonants_with_digits() {
    check_soundex(~"Acdl", ~"A234");
}
