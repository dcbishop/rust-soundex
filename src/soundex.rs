pub struct Soundex;

impl Soundex {
    pub fn new() -> Soundex {
        return Soundex;
    }

    pub fn encode(&self, string: &str) -> ~str {
        let encoded = string.to_owned();
        return zero_pad(encoded);
    }
}

fn zero_pad(string: &str) -> ~str {
    return string + "000";
}
