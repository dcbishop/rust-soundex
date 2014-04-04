use soundex::Soundex;

mod soundex;

#[test]
fn soundex_encoding_retains_sole_letter_of_one_letter_word() {
    let soundex = Soundex::new();
    let encoded = soundex.encode("A");
    assert_eq!(encoded, ~"A");
}
