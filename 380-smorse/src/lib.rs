use lazy_static::lazy_static;
use std::collections::HashSet;

pub mod input_generator;

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

struct AlphaSearch<'a> {
    input: &'a [u8],
    alphabet: u32,
    prefix: [u8; 26],
}

impl<'a> AlphaSearch<'a> {
    fn new(code: &'a str) -> AlphaSearch<'a> {
        AlphaSearch {
            input: code.as_bytes(),
            alphabet: 0x03ff_ffff, // 26 low bits set
            prefix: [0; 26],
        }
    }

    fn add_alpha(&mut self, idx: u8) {
        self.alphabet |= 1 << idx;
    }

    fn contains_alpha(&self, idx: u8) -> bool {
        self.alphabet & (1 << idx) > 0
    }

    fn remove_alpha(&mut self, idx: u8) -> bool {
        let is_set = self.contains_alpha(idx);
        self.alphabet &= !(1 << idx);
        is_set
    }

    fn next_inner(&mut self, input: &[u8], idx: usize) -> bool {
        if input.is_empty() || self.alphabet == 0 {
            return input.is_empty() && self.alphabet == 0;
        }

        for chb in self.prefix[idx]..26 {
            self.prefix[idx] = chb;
            let sym = MORSE[chb as usize].as_bytes();
            if input.starts_with(sym) && self.remove_alpha(chb) {
                let ok = self.next_inner(&input[sym.len()..], idx + 1);
                if !ok {
                    self.prefix[idx + 1] = 0;
                }
                self.add_alpha(chb);
                if ok {
                    return true;
                }
            }
        }
        return false;
    }
}

impl<'a> Iterator for AlphaSearch<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.next_inner(self.input, 0) {
            Some(self.prefix.iter().map(|b| (b + b'a') as char).collect())
        } else {
            None
        };

        // we now have to clean up the internal state: if we were to call self.next_inner
        // again right away, we'd immediately generate the same result, because
        // we'd just descend right down the same path as before.
        for idx in (0..26).rev() {
            if self.prefix[idx] == 25 {
                // if the last letter is a z, we zeroize it and continue on
                self.add_alpha(25);
                self.prefix[idx] = 0;
            } else {
                // otherwise, we increment it and stop. This ensures that the normal
                // recursive pattern will keep generating new combinations for us.
                self.add_alpha(self.prefix[idx]);
                self.prefix[idx] += 1;
                break;
            }
        }

        result
    }
}

pub fn smalpha_all(code: &str) -> impl Iterator<Item = String> + '_ {
    AlphaSearch::new(code)
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

    fn check_result(input: &str, result: &str) {
        assert_eq!(result.len(), 26);
        assert_eq!(
            result.chars().collect::<HashSet<_>>(),
            (b'a'..=b'z').map(char::from).collect::<HashSet<_>>()
        );
        assert_eq!(smorse(&result), input);
    }

    #[test]
    fn test_smalpha() {
        let input =
            "......-..--...---.-....---...--....--.-..---.....---.-.---..---.-....--.-.---.-.--";
        let result = smalpha(input);
        assert!(result.is_some());
        check_result(input, &result.unwrap());
    }

    #[test]
    fn test_smalpha_all() {
        let input =
            "......-..--...---.-....---...--....--.-..---.....---.-.---..---.-....--.-.---.-.--";
        let result = smalpha_all(input).next();
        assert!(result.is_some());
        check_result(input, &result.unwrap());
    }

    #[test]
    fn test_smalpha_compare() {
        let input =
            "......-..--...---.-....---...--....--.-..---.....---.-.---..---.-....--.-.---.-.--";
        let r1 = smalpha(input);
        let r2 = smalpha_all(input).next();
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_smalpha_state() {
        let input =
            "......-..--...---.-....---...--....--.-..---.....---.-.---..---.-....--.-.---.-.--";
        let results = smalpha_all(input).take(2).collect::<Vec<_>>();
        assert_eq!(results.len(), 2);
        assert_ne!(results[0], results[1]);
    }

    #[test]
    #[ignore]
    // this may take a while...
    fn test_smalpha_container() {
        let input =
            ".--...-.-.-.....-.--........----.-.-..---.---.--.--.-.-....-..-...-.---..--.----..";
        let results = smalpha_all(input).collect::<Vec<_>>();
        println!("results:");
        for result in &results {
            println!("  {}", result);
        }
        for result in &results {
            check_result(input, result);
        }
        assert!(results.iter().any(|r| r == "wirnbfzehatqlojpgcvusyxkmd"));
    }

    #[test]
    #[ignore]
    // this may take a while...
    fn test_smalpha_count() {
        let input =
            "......-..--...---.-....---...--....--.-..---.....---.-.---..---.-....--.-.---.-.--";
        assert_eq!(smalpha_all(input).count(), 41);
    }
}
