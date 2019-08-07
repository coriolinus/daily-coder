use lazy_static::lazy_static;

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
}
