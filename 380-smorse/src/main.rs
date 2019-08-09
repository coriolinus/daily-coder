use counter::Counter;
use smorse::{smalpha, smalpha_all, smorse};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "smorse", about = "convert strings to squashed morse code")]
struct Opts {
    /// convert a lowercase string to squashed morse
    input: Option<String>,

    /// path to wordlist
    #[structopt(short, long = "word-list", parse(from_os_str))]
    word_list: Option<PathBuf>,

    /// find the only sequence that's the code for 13 different words
    #[structopt(long = "bonus-1-1")]
    bonus_1_1: bool,

    /// find the only word that has 15 dashes in a row
    #[structopt(long = "bonus-1-2")]
    bonus_1_2: bool,

    /// find all 21-letter words with the same number of dots and dashes
    #[structopt(long = "bonus-1-3")]
    bonus_1_3: bool,

    /// find the only 13-letter word that encodes to a palindrome
    #[structopt(long = "bonus-1-4")]
    bonus_1_4: bool,

    /// find all 13-char sequences which do not appear in the encoding of any word
    #[structopt(long = "bonus-1-5")]
    bonus_1_5: bool,

    /// search for permutations of an alphabet which produce this squashed morse value
    #[structopt(long)]
    smalpha: Option<String>,

    /// search for permutations of an alphabet which produce this sm value for each line in this input file
    #[structopt(long = "smorse-file", parse(from_os_str))]
    smorse_file: Option<PathBuf>,

    /// find lexicographically minimal input which produces exactly one alphabet
    #[structopt(long = "bonus-2-2")]
    bonus_2_2: Option<Option<i128>>,
}

type Rv = Result<(), Box<dyn Error>>;

fn main() -> Rv {
    let opts = Opts::from_args();

    if let Some(s) = opts.input {
        println!("{}", smorse(&s));
    }

    if let Some(wl_path) = opts.word_list {
        if opts.bonus_1_1 {
            bonus_1_1(&wl_path)?;
        }
        if opts.bonus_1_2 {
            bonus_1_2(&wl_path)?;
        }
        if opts.bonus_1_3 {
            bonus_1_3(&wl_path)?;
        }
        if opts.bonus_1_4 {
            bonus_1_4(&wl_path)?;
        }
        if opts.bonus_1_5 {
            bonus_1_5(&wl_path)?;
        }
        if !(opts.bonus_1_1 || opts.bonus_1_2 || opts.bonus_1_3 || opts.bonus_1_4 || opts.bonus_1_5)
        {
            let mut counts: Counter<u8> = Counter::new();
            for word in get_words(&wl_path)? {
                counts += smorse(&word).as_bytes().iter().cloned();
            }
            println!("Total counts:");
            for (b, c) in counts.iter() {
                println!(" {}: {}", *b as char, c);
            }
        }
    } else {
        if opts.bonus_1_1 || opts.bonus_1_2 || opts.bonus_1_3 || opts.bonus_1_4 || opts.bonus_1_5 {
            eprintln!("bonus challenges require wordlist");
        }
    }

    if let Some(s) = opts.smalpha {
        let expect = smorse(&(b'a'..=b'z').map(char::from).collect::<String>())
            .chars()
            .collect::<Counter<_>>();
        let got = s.trim().chars().collect::<Counter<_>>();
        if expect != got {
            eprintln!("Bad input for smalpha: require:\n{:#?}", expect);
        }
        match smalpha(&s) {
            None => println!("no permutation found for this alphabet"),
            Some(s) => println!("{}", s),
        }
    }

    if let Some(path) = opts.smorse_file {
        for input in get_words(&path)? {
            println!("{} -> {:?}", input, smalpha(&input));
        }
    }

    if let Some(start) = opts.bonus_2_2 {
        bonus_2_2(start);
    }

    Ok(())
}

fn get_words(wl_path: &Path) -> Result<Box<impl Iterator<Item = String>>, Box<dyn Error>> {
    let wordlist = File::open(wl_path)?;
    let reader = BufReader::new(wordlist);
    Ok(Box::new(reader.lines().filter_map(|r| r.ok())))
}

/// find the only sequence that's the code for 13 different words
fn bonus_1_1(wl_path: &Path) -> Rv {
    let counts = get_words(wl_path)?
        .map(|word| smorse(&word))
        .collect::<Counter<_>>();
    for (seq, count) in counts.iter() {
        if *count == 13 {
            println!("Sequence encoding 13 words: {}", seq);
            break;
        }
    }

    Ok(())
}

/// find the only word that has 15 dashes in a row
fn bonus_1_2(wl_path: &Path) -> Result<(), Box<dyn Error>> {
    let needle = b"---------------";
    'outer: for word in get_words(wl_path)? {
        let seq = smorse(&word);
        let hay = seq.as_bytes();
        for w in hay.windows(15) {
            if w == needle {
                println!("{} encodes as {} which has 15 dashes in a row", word, seq);
                break 'outer;
            }
        }
    }

    Ok(())
}

/// find all 21-letter words with the same number of dots and dashes
fn bonus_1_3(wl_path: &Path) -> Rv {
    for word in get_words(wl_path)?.filter(|word| word.len() == 21) {
        let seq = smorse(&word);
        let counts = seq.as_bytes().iter().collect::<Counter<_>>();
        if counts.get(&b'-').is_some() && counts.get(&b'-') == counts.get(&b'.') {
            println!(
                "{} encodes as {} which has {} each dots and dashes",
                word,
                seq,
                counts.get(&b'-').unwrap()
            );
        }
    }

    Ok(())
}

/// find the only 13-letter word that encodes to a palindrome
fn bonus_1_4(wl_path: &Path) -> Rv {
    for word in get_words(wl_path)?.filter(|word| word.len() == 13) {
        let seq = smorse(&word);
        let seqb = seq.as_bytes();
        if seqb.iter().zip(seqb.iter().rev()).all(|(a, b)| a == b) {
            println!("{} encodes as {} which is a palindrome", word, seq);
        }
    }

    Ok(())
}

fn n2s(n: u16) -> String {
    let mut out = String::with_capacity(16);
    for i in (0_u16..16).rev() {
        if n & (1 << i) > 0 {
            out.push('-')
        } else {
            out.push('.')
        }
    }
    out
}

/// find all 13-char sequences which do not appear in the encoding of any word
fn bonus_1_5(wl_path: &Path) -> Rv {
    let word_sequences = get_words(wl_path)?
        .map(|word| smorse(&word).into_bytes())
        .collect::<HashSet<_>>();

    println!("13-char sequences which appear in no words:");
    for seq in (0_u16..(1 << 13)).map(n2s) {
        let needle = &seq.as_bytes()[3..]; // final 13
        if !word_sequences
            .iter()
            .any(|ws| ws.windows(13).any(|hay| needle == hay))
        {
            println!(" {}", &seq[3..]);
        }
    }

    Ok(())
}

/// Find lexicographically minimal input which produces exactly one alphabet.
///
/// Every input which produces an alphabet contains 82 dots and dashes, and
/// no other symbols. Of those, 44 are dots, and 38 are dashes.
///
/// Lexicographically, dashes are lower than dots. We can therefore restate
/// the problem as: Representing dashes as 0 and dots as 1, find the lowest
/// number containing 44 `1` bits and 38 `0` bits, which when suitably transformed
/// produces exactly one alphabet.
///
/// Because of this restatement, we can sharply reduce the number of permutations
/// we must consider, because there are efficient algorithms for generating
/// integers containing exactly N `1` bits.
fn bonus_2_2(start: Option<i128>) {
    // this is expected to be a long-running process, so we
    use smorse::input_generator::InputGenerator;
    use std::io::stdout;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    let halt = Arc::new(AtomicBool::new(false));
    let hhalt = halt.clone(); // gets moved into the handler

    ctrlc::set_handler(move || hhalt.store(true, Ordering::SeqCst))
        .expect("Error setting Ctrl-C handler");

    for (idx, input) in InputGenerator::maybe_start_at(start).enumerate() {
        // emit some output every once in a while just to demonstrate activity
        if idx & 0xfffff == 0 {
            print!(".");
            stdout().flush().expect("flushing stdout");
        }
        // check the interrupt every 256 iterations
        if idx & 0xfff == 0 && halt.load(Ordering::SeqCst) {
            println!();
            println!("Checked {} inputs; continue with", idx);
            println!("  smorse --bonus-2-2 {}", InputGenerator::i2n(&input));
            break;
        }
        if smalpha_all(&input).take(2).count() == 1 {
            println!();
            println!("{} => {}", input, smalpha(&input).unwrap());
            break;
        }
    }
}
