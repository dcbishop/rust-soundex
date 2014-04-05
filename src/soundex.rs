pub struct Soundex;

static MAX_CODE_LENGTH: uint = 4;

impl Soundex {
    pub fn new() -> Soundex {
        return Soundex;
    }

    pub fn encode(&self, word: &str) -> ~str {
        let encoded = zero_pad(head(word) + encoded_digits(tail(word)));
        return encoded;
    }
}

fn zero_pad(string: &str) -> ~str {
    let zeros_needed = MAX_CODE_LENGTH - string.len();
    return string + "000".slice_to(zeros_needed);
}

fn head<'a>(string: &'a str) -> &'a str {
    return string.slice_to(1);
}

fn tail<'a>(string: &'a str) -> &'a str {
    return string.slice_from(1);
}

fn encoded_digits(word: &str) -> ~str {
    let mut encoding = ~"";

    for c in word.bytes() {
        encoding.push_str(encode_digit(c));
    }

    return encoding;
}

fn encode_digit(letter: u8) -> ~str {
    let c_letter = letter as char;

    return match c_letter {
        'b' | 'f' | 'p' | 'v' => ~"1",
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => ~"2",
        'd' | 't' => ~"3",
        'l' => ~"4",
        'm' | 'n' => ~"5",
        'r' => ~"6",
        _ => ~"",
    };
}
