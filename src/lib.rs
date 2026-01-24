use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::collections::HashMap;

pub struct Config {
    flags: Flags,
    files: Vec<String>
}

struct Flags {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
    top: Option<u8>
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

// impl Default for Flags {
//     fn default() -> Flags {
//         Flags {
//             lines: true,
//             words: true,
//             chars: true,
//             bytes: false,
//             top: None
//         }
//     }
// }

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
                    let n: u8 = args.next()
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
    // top: HashMap<String, usize>
}

impl Stats {
    fn new() -> Stats {
        Stats {
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0,
            // top: HashMap::new()
        }
    }

    fn print(&self, flags: Flags) {
        if flags.lines { println!("Lines: {}", self.lines); }
        if flags.words { println!("Words: {}", self.words); }
        if flags.chars { println!("Chars: {}", self.chars); }
        if flags.bytes { println!("Bytes: {}", self.bytes); }
        // if flags.top.is_some() {
        //     let n_top = flags.top.unwrap();
        //     println!("Top {} words:", n_top);
        // }
    }
}

// fn count_words(line: &String) -> usize {
//     1
// }

// fn get_top_words

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut stats = Stats::new();

    for file in config.files {
        let file = File::open(file).expect("Could not open file");
        let reader = BufReader::new(file);

        // TODO: refactor, each flag is been checked at every iteration.
        for line in reader.lines() {
            let line = line?;
            if config.flags.lines { stats.lines += 1 }
            // if config.flags.words { stats.words += count_words(&line) }
            if config.flags.chars { stats.chars += line.chars().count() }
            if config.flags.bytes { stats.bytes += line.bytes().count() }
            // if config.flags.top {  }
        }
    }

    stats.print(config.flags);
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn some_test() {
        assert_eq!(2, 2);
    }
}