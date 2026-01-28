use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

pub struct Config {
    flags: Flags,
    files: Vec<String>
}

struct Flags {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
    top: Option<usize>
}

impl Flags {
    fn new() -> Flags {
        Flags {
            lines: false,
            words: false,
            chars: false,
            bytes: false,
            top: None
        }
    }

    fn default() -> Flags {
        Flags {
            lines: true,
            words: true,
            chars: true,
            bytes: false,
            top: None
        }
    }
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String> + ExactSizeIterator) -> Result<Config, &'static str> {
        args.next();

        let mut flags = Flags::new();
        let mut files = Vec::new();

        let n_args = args.len();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--l" | "--lines" => flags.lines = true,
                "--w" | "--words" => flags.words = true,
                "--c" | "--chars" => flags.chars = true,
                "--b" | "--bytes" => flags.bytes = true,
                "--t" | "--top" => {
                    let n: usize = args.next()
                        .expect("Missing argument for --top")
                        .parse()
                        .expect("Invalid argument for --top");
                    flags.top = Some(n);
                }
                _ => {
                    if args.len() == n_args - 1 { flags = Flags::default() }
                    files.push(arg);
                }
            }
        }

        if files.len() == 0 { return Err("No files provided"); }

        Ok(Config {
            flags,
            files
        })
    }
}

struct Stats {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    top: HashMap<String, usize>
}

impl Stats {
    fn new() -> Stats {
        Stats {
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0,
            top: HashMap::new()
        }
    }

    fn print(&mut self, flags: Flags) {
        if flags.lines { println!("Lines: {}", self.lines); }
        if flags.words { println!("Words: {}", self.words); }
        if flags.chars { println!("Chars: {}", self.chars); }
        if flags.bytes { println!("Bytes: {}", self.bytes); }
        if flags.top.is_some() {
            let n_top = flags.top.unwrap();
            let mut heap: BinaryHeap<WordCount> = BinaryHeap::new();
            
            for (word, count) in self.top.drain() {
                heap.push(WordCount{ word, count })
            }

            println!("Top {} words:", n_top);
            for _ in 0..n_top {
                if heap.is_empty() { return }
                let wc = heap.pop().unwrap();
                println!("{} {}", wc.count, wc.word);
            }
        }
    }
}

#[derive(Eq, PartialEq)]
struct WordCount {
    word: String,
    count: usize,
}

impl Ord for WordCount {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count
            .cmp(&other.count)
            .then_with(|| self.word.cmp(&other.word))
    }
}

impl PartialOrd for WordCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_words_count(line: &String) -> usize {
    line.split_whitespace().filter(|word| word.len() > 0 || word.is_ascii()).count()
}

fn count_words(line: &String, map: &mut HashMap<String, usize>) {
    let words: Vec<&str> = line.split_whitespace().filter(|word| word.len() > 0 || word.is_ascii()).collect();
    for word in words {
        let count = map.entry(word.to_string()).or_insert(0);
        *count += 1;
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut stats = Stats::new();

    for file in config.files {
        let file = File::open(file).expect("Could not open file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if config.flags.lines { stats.lines += 1 }
            if config.flags.words { stats.words += get_words_count(&line) }
            if config.flags.chars { stats.chars += line.chars().count() }
            if config.flags.bytes { stats.bytes += line.bytes().count() }
            if config.flags.top.is_some() { count_words(&line, &mut stats.top) }
        }
    }

    stats.print(config.flags);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_words_count_zero() {
        let result = get_words_count(&String::from(""));
        assert_eq!(result, 0);
    }

    #[test]
    fn get_words_count_two() {
        let result = get_words_count(&String::from("a b"));
        assert_eq!(result, 2);
    }
}