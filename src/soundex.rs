pub struct Soundex;

impl Soundex {
    pub fn new() -> Soundex {
        return Soundex;
    }

    pub fn encode(&self, word: &str) -> ~str {
        let mut encoded = word.slice_to(1).to_owned();
        if word.len() > 1 {
            encoded.push_char('1');
        }
        return zero_pad(encoded);
    }
}

fn zero_pad(string: &str) -> ~str {
    let zeros_needed = 4 - string.len();
    return string + "000".slice_to(zeros_needed);
}
