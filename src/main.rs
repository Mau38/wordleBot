use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static WORDS_PATH: String = String::from("./words.txt");

fn main() {}

fn read_words<P>() -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(WORDS_PATH)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_words<P: std::convert::AsRef<std::path::Path>>() -> Vec<&'static String> {
    let mut words: Vec<&String> = Vec::new();
    if let Ok(lines) = read_words::<P>() {
        for line in lines {
            if let Ok(word) = line {
                words.push(&word);
            }
        }
    }
    words
}

fn solve(
    words: Vec<&String>,
    greens: HashMap<char, Vec<u8>>,
    yellows: HashMap<Vec<u8>, char>,
    greys: Vec<char>,
) -> Vec<&String> {
}

fn check_for_greys(words: Vec<String>, greys: Vec<char>) -> Vec<&'static String> {
    words
        .to_owned()
        .iter()
        .filter(|word| greys.iter().any(|grey| word.contains(*grey)))
        .collect()
}
