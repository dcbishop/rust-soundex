use soundex::Soundex;

mod soundex;

#[allow(unused_variable)]
#[test]
fn soundex_encoding_has_encode_function() {
    let soundex = Soundex::new();
    let encoded = soundex.encode("A");
}
