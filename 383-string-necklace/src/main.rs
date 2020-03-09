use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use string_necklace::StringNecklace;
use structopt::StructOpt;

pub fn matches_in<Iter, Str>(words: Iter) -> HashMap<StringNecklace, usize>
where
    Iter: IntoIterator<Item = Str>,
    Str: AsRef<str>,
{
    let mut h = HashMap::new();
    for word in words.into_iter() {
        let necklace = StringNecklace::new(word);
        *h.entry(necklace).or_default() += 1;
    }
    h
}

#[derive(Debug, StructOpt)]
#[structopt(
    about = "Given a wordlist, get those items in the wordlist which have N matches as string necklaces."
)]
struct Opt {
    /// Path to wordlist
    #[structopt(parse(from_os_str))]
    wordlist: PathBuf,

    /// How many equivalents we should look for
    n: usize,
}

fn main() -> Result<()> {
    let Opt { wordlist, n } = Opt::from_args();
    let file = File::open(wordlist)?;
    let words: Vec<String> = io::BufReader::new(file)
        .lines()
        .collect::<std::result::Result<Vec<String>, _>>()?;

    let mut match_map = matches_in(&words);
    match_map.retain(|_, count| *count == n);
    for sn in match_map.keys() {
        for word in &words {
            if &StringNecklace::new(word) == sn {
                println!("{}", word);
            }
        }
        println!();
    }
    Ok(())
}
