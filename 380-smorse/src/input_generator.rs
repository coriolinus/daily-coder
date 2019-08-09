const INPUT_SIZE: u32 = 82;
const ON_BITS: u32 = 44;

// 44 on bits, all leading 0s
const LOW_ALPHABET: i128 = 0x0fff_ffff_ffff;
// 44 on bits, shifted to the high end of the range. there's no point generating
// possibilities past this point.
const HIGH_ALPHABET: i128 = LOW_ALPHABET << (INPUT_SIZE - ON_BITS);

/// From <https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation>
///
/// Suppose we have a pattern of N bits set to 1 in an integer and we want the next
/// permutation of N 1 bits in a lexicographical sense. For example, if N is 3 and
/// the bit pattern is `00010011`, the next patterns would be `00010101`, `00010110`,
/// `00011001`, `00011010`, `00011100`, `00100011`, and so forth. The following is
/// a fast way to compute the next permutation.
///
/// ```notrust
/// unsigned int v; // current permutation of bits
/// unsigned int w; // next permutation of bits
///
/// unsigned int t = v | (v - 1); // t gets v's least significant 0 bits set to 1
/// // Next set to 1 the most significant bit to change,
/// // set to 0 the least significant ones, and add the necessary 1 bits.
/// w = (t + 1) | (((~t & -~t) - 1) >> (__builtin_ctz(v) + 1));
/// ```
///
/// The `__builtin_ctz(v)` GNU C compiler intrinsic for x86 CPUs returns the number
/// of trailing zeros. If you are using Microsoft compilers for x86, the intrinsic
/// is `_BitScanForward`. These both emit a bsf instruction, but equivalents may be
/// available for other architectures. If not, then consider using one of the
/// methods for counting the consecutive zero bits mentioned earlier.
fn next_permutation(v: i128) -> i128 {
    let t = v | (v - 1);
    (t + 1) | (((!t & -!t) - 1) >> (v.trailing_zeros() + 1))
}

pub struct InputGenerator {
    n: i128,
    buffer: [u8; INPUT_SIZE as usize],
}

impl InputGenerator {
    pub fn new() -> InputGenerator {
        InputGenerator {
            n: LOW_ALPHABET,
            buffer: [0; INPUT_SIZE as usize],
        }
    }

    pub fn start_at(mut n: i128) -> InputGenerator {
        // avoid some work if the caller is naive
        if n < 0 {
            n = LOW_ALPHABET;
        }
        InputGenerator {
            n,
            buffer: [0; INPUT_SIZE as usize],
        }
    }

    pub fn maybe_start_at(n: Option<i128>) -> InputGenerator {
        match n {
            None => InputGenerator::new(),
            Some(n) => InputGenerator::start_at(n),
        }
    }

    pub fn get_n(&mut self) -> i128 {
        self.n
    }

    /// convert an input into the number which produced it, for restarts
    pub fn i2n(input: &str) -> i128 {
        let input = input.as_bytes();
        let mut n = 0;
        for idx in 0..INPUT_SIZE {
            if input[(INPUT_SIZE - 1 - idx) as usize] == b'.' {
                n |= 1 << idx;
            }
        }
        n
    }

    fn update_buffer(&mut self) {
        for idx in 0..INPUT_SIZE {
            self.buffer[(INPUT_SIZE - 1 - idx) as usize] =
                if self.n & (1 << idx) > 0 { b'.' } else { b'-' };
        }
    }

    pub fn current_input(&mut self) -> String {
        self.update_buffer();
        unsafe { String::from_utf8_unchecked(self.buffer.to_vec()) }
    }
}

impl Iterator for InputGenerator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.n > HIGH_ALPHABET {
            return None;
        }
        let out = self.current_input();
        self.n = next_permutation(self.n);
        Some(out)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_permutations() {
        let mut n = LOW_ALPHABET;

        // we're not going to generate all permutations, but a representative
        // sampling should be convincing
        for _ in 0..1024 {
            assert!(n >= LOW_ALPHABET);
            assert!(n <= HIGH_ALPHABET);
            assert_eq!(n.count_ones(), ON_BITS);

            let next = next_permutation(n);
            assert!(next > n);
            n = next;
        }
    }

    #[test]
    fn test_i2n() {
        for n in 0_i128..1024 {
            let mut ig = InputGenerator::start_at(n);
            assert_eq!(InputGenerator::i2n(&ig.current_input()), n);
        }
    }
}
