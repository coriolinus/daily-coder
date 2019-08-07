use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    pub static ref MORSE: Vec<&'static str> = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..".split(' ').collect();
}

fn morse(c: char) -> &'static str {
    match c {
        'a'..='z' => MORSE[c as usize - 'a' as usize],
        _ => "",
    }
}

pub fn smorse(s: &str) -> String {
    s.chars().map(morse).collect()
}

fn alpha_search(input: &[u8], alphabet: &mut HashSet<u8>, prefix: &mut Vec<u8>) -> bool {
    if input.is_empty() || alphabet.is_empty() {
        return input.is_empty() && alphabet.is_empty();
    }
    for chb in 0_u8..26 {
        let sym = MORSE[chb as usize].as_bytes();
        if input.starts_with(sym) && alphabet.remove(&chb) {
            prefix.push(chb);
            if alpha_search(&input[sym.len()..], alphabet, prefix) {
                return true;
            }
            prefix.pop();
            alphabet.insert(chb);
        }
    }
    return false;
}

pub fn smalpha(code: &str) -> Option<String> {
    let mut alphabet = (0_u8..26).collect::<HashSet<_>>();
    let mut prefix = Vec::with_capacity(26);
    if alpha_search(code.as_bytes(), &mut alphabet, &mut prefix) {
        Some(prefix.iter().map(|b| (b + b'a') as char).collect())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(morse('a'), ".-");
    }

    #[test]
    fn test_b() {
        assert_eq!(morse('b'), "-...");
    }

    #[test]
    fn test_sos() {
        assert_eq!(smorse("sos"), "...---...");
    }

    #[test]
    fn test_daily() {
        assert_eq!(smorse("daily"), "-...-...-..-.--");
    }

    #[test]
    fn test_programmer() {
        assert_eq!(smorse("programmer"), ".--..-.-----..-..-----..-.");
    }

    #[test]
    fn test_bits() {
        assert_eq!(smorse("bits"), "-.....-...");
    }

    #[test]
    fn test_three() {
        assert_eq!(smorse("three"), "-.....-...");
    }

    #[test]
    fn test_smalpha() {
        let input =
            ".--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----..";
        let result = smalpha(input);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.len(), 26);
        assert_eq!(
            result.chars().collect::<HashSet<_>>(),
            (b'a'..=b'z').map(char::from).collect::<HashSet<_>>()
        );
        assert_eq!(smorse(&result), input);
    }
}
