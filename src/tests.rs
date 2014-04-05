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

#[test]
fn soundex_encoding_limits_length_to_four_characters() {
    let soundex = Soundex::new();
    assert_eq!(soundex.encode("Dcdlb").len(), 4);
}

#[test]
fn soundex_encoding_ignores_vowel_like_letters() {
   check_soundex(~"Baeiouhycdl", ~"B234"); 
}

#[test]
fn soundex_encoding_combines_duplicate_encodings() {
   assert_eq!(soundex::encoded_digit('b'), soundex::encoded_digit('f'));
   assert_eq!(soundex::encoded_digit('c'), soundex::encoded_digit('g'));
   assert_eq!(soundex::encoded_digit('d'), soundex::encoded_digit('t'));
   check_soundex(~"Abfcgdt", ~"A123");
}
