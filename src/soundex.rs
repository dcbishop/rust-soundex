use std::str;

pub struct Soundex;

static MAX_CODE_LENGTH: uint = 4;
static NOT_A_DIGIT: &'static str = "*";

impl Soundex {
    pub fn new() -> Soundex {
        return Soundex;
    }

    pub fn encode(&self, word: &str) -> String {
        let s = "".to_string() + upper_front(head(word)).as_slice() + tail(encoded_digits(word).as_slice());
        let encoded = zero_pad(s.as_slice());
        return encoded;
    }
}

fn zero_pad(string: &str) -> String {
    let zeros_needed = MAX_CODE_LENGTH - string.len();
    return "".to_string() + string + "000".slice_to(zeros_needed);
}

fn upper_front(string: &str) -> String {
    let mut upper_string = "".to_string();
    upper_string.push_char(string.char_at(0).to_uppercase());
    upper_string.push_str(string.slice_from(1));
    return upper_string
}

fn head<'a>(string: &'a str) -> &'a str {
    return string.slice_to(1);
}

fn tail<'a>(string: &'a str) -> &'a str {
    return string.slice_from(1);
}

fn encoded_digits(word: &str) -> String {
    let mut encoding: String;
    encoding = encode_head(word).to_string();
    encoding = encode_tail(encoding, word);
    return encoding;
}

fn encode_head<'a>(word: &'a str) -> &'a str{
   return encoded_digit(word.char_at(0));
}

fn encode_tail(mut encoding: String, word: &str) -> String {
    for i in range(1, word.len()) {
        if !is_complete(encoding.as_slice()) {
            encoding = encode_letter(encoding, word.char_at(i), word.char_at(i-1));
        }
    }

    return encoding;
}

fn is_complete(encoding: &str) -> bool {
    return encoding.len() == MAX_CODE_LENGTH;
}

fn encode_letter(mut encoding: String, letter: char, last_letter: char) -> String {
    let digit = encoded_digit(letter);
    if !str::eq_slice(digit, NOT_A_DIGIT) && (digit != last_digit(encoding.as_slice()) || is_vowel(last_letter)) {
        encoding.push_str(encoded_digit(letter));
    }
    return encoding;
}

pub fn encoded_digit(letter: char) -> &'static str {
    return match letter.to_lowercase() {
        'b' | 'f' | 'p' | 'v' => "1",
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => "2",
        'd' | 't' => "3",
        'l' => "4",
        'm' | 'n' => "5",
        'r' => "6",
        _ => NOT_A_DIGIT,
    };
}

fn last_digit<'a>(encoding: &'a str) -> &'a str {
    if encoding.len() == 0 {
        return NOT_A_DIGIT;
    }
    return encoding.slice(encoding.len()-1, encoding.len());
}

fn is_vowel(letter: char) -> bool {
   return match letter {
       'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
   };
}
