use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringNecklace(VecDeque<char>);

impl StringNecklace {
    pub fn new<T>(s: T) -> StringNecklace
    where
        T: AsRef<str>,
    {
        let mut d: VecDeque<char> = s.as_ref().chars().collect();
        if !d.is_empty() {
            d = (0..d.len())
                .map(|shift| {
                    let mut c = d.clone();
                    c.rotate_left(shift);
                    c
                })
                .min()
                .unwrap();
        }
        StringNecklace(d)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn repeats(&self) -> usize {
        if self.0.is_empty() {
            return 1;
        }
        let mut count = 0;
        let mut m = self.clone();
        for _ in 0..self.len() {
            m.0.rotate_left(1);
            if &m == self {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest(
        a,
        b,
        expect_equal,
        case("nicole", "icolen", true),
        case("nicole", "lenico", true),
        case("nicole", "coneli", false),
        case("aabaaaaabaab", "aabaabaabaaa", true),
        case("abc", "cba", false),
        case("xxyyy", "xxxyy", false),
        case("xyxxz", "xxyxz", false),
        case("x", "x", true),
        case("x", "xx", false),
        case("x", "", false),
        case("", "", true)
    )]
    fn test_examples(a: &str, b: &str, expect_equal: bool) {
        let a = StringNecklace::new(a);
        let b = StringNecklace::new(b);
        assert_eq!(a == b, expect_equal);
    }

    #[rstest(
        s,
        expect,
        case("abc", 1),
        case("abcabcabc", 3),
        case("abcabcabcx", 1),
        case("aaaaaa", 6),
        case("a", 1),
        case("", 1)
    )]
    fn test_bonus_1(s: &str, expect: usize) {
        dbg!(s);
        let n = StringNecklace::new(s);
        assert_eq!(n.repeats(), expect);
    }
}
