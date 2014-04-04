pub struct Soundex;

impl Soundex {
    pub fn new() -> Soundex {
        return Soundex;
    }

    pub fn encode(&self, string: &str) -> ~str {
        return string.to_owned();
    }
}
