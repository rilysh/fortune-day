use std::{
    borrow::Cow,
    env,
    fs::{self, File},
    io::{BufReader, Read},
    process,
};

use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use walkdir::WalkDir;

use crate::{
    colors::{color, Colors},
    randnum::get_rand_value,
};

const FORTUNE_DIR: &str = "fortunes";

fn get_fortunes_dir<'a>() -> Cow<'a, str> {
    if let Ok(fortunes) = env::var("FORTUNES_DIR") {
        fortunes.into()
    } else {
        Cow::Borrowed(FORTUNE_DIR)
    }
}

fn get_rand_file() -> Result<String, ()> {
    if let Ok(dir) = fs::read_dir(&*get_fortunes_dir()) {
        let mut files = Vec::new();
        for file in dir {
            files.push(file.unwrap().path().to_str().unwrap().to_string());
        }

        if files.len() == 0 {
            eprintln!("error: fortunes directory is empty!");
            return Err(());
        }

        Ok(files[get_rand_value(files.len())].to_string())
    } else {
        eprintln!("error: no fortunes directory was found.");
        return Err(());
    }
}

fn read_fortune_file(name: &str) -> String {
    let file = File::open(name).unwrap();

    // Re-encode with UTF-8 character encoding.
    // See: https://en.wikipedia.org/wiki/UTF-8
    let mut reader = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(UTF_8))
            .build(file),
    );

    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    content
}

pub fn get_rand_fortune() -> String {
    let file = match get_rand_file() {
        Ok(file) => file,
        Err(()) => process::exit(1),
    };

    let content = read_fortune_file(&file);
    let quotes = content.split("\n%\n").collect::<Vec<&str>>();
    let quote = quotes[get_rand_value(quotes.len())];

    // If the quote is empty (e.g. end of line with only newlines)
    if quote.len() <= 1 {
        let mut i = 1;
        while i < quotes.len() {
            if quotes[quotes.len() - i].len() > 1 {
                return quotes[quotes.len() - i].to_string();
            }

            i += 1;
        }

        // No string literal even in
        quotes[0].to_string()
    } else {
        quote.to_string()
    }
}

pub fn get_rand_fortune_from_all() -> Result<String, ()> {
    let mut quotes = Vec::new();
    let walkdir = WalkDir::new(&*get_fortunes_dir());

    for file in walkdir {
        if let Ok(file) = file {
            if !file.path().is_dir() {
                let content = read_fortune_file(file.path().to_str().unwrap());
                quotes.push(content);
            }
        } else {
            eprintln!("no fortunes directory was found.");
            return Err(());
        }
    }

    let mut new_quotes = Vec::new();
    quotes
        .iter()
        .for_each(|e| e.split("\n%\n").for_each(|e| new_quotes.push(e)));

    // Move the local value (a clone)
    let quote = new_quotes[get_rand_value(new_quotes.len())];
    if quote.len() <= 1 {
        let mut i = 1;
        while i < quotes.len() {
            if quotes[quotes.len() - i].len() > 1 {
                return Ok(quotes[quotes.len() - i].to_string());
            }
            i += 1;
        }

        if quotes[0].len() <= 1 {
            Ok("Oops, no quote. Are fortune cookies empty?".to_string())
        } else {
            Ok(quotes[0].to_string())
        }
    } else {
        Ok(quote.to_string())
    }
}

pub fn search_fortune(pattern: &str, insensitive: bool, show_colors: bool, only_one: bool) {
    let mut contents = Vec::new();

    if let Ok(dir) = fs::read_dir(&*get_fortunes_dir()) {
        for file in dir {
            let content = read_fortune_file(file.unwrap().path().to_str().unwrap());
            contents.push(content);
        }
    }

    let pattern_mod = if insensitive {
        &pattern.to_lowercase()
    } else {
        pattern
    };

    'do_loop: for content in contents {
        for entry in content.split("\n%\n") {
            let entry_mod = if insensitive {
                &entry.to_lowercase()
            } else {
                entry
            };

            if entry_mod.contains(pattern) {
                let pattern_pos = entry_mod.find(&pattern_mod).unwrap();
                let pat = &entry[pattern_pos..pattern_pos + pattern.len()];

                let mut new_line = String::new();
                if show_colors {
                    new_line.push_str(&entry[0..pattern_pos]);
                    new_line.push_str(&color(Colors::Red));
                    new_line.push_str(pat);
                    new_line.push_str(&color(Colors::End));
                    new_line.push_str(&entry[pattern_pos + pattern.len()..]);
                    println!("{}\n%", new_line);
                } else {
                    println!("{}\n%", entry);
                }

                // Break the loop as soon as we found a match
                if only_one {
                    break 'do_loop;
                }
            }
        }
    }
}

pub fn search_fortune_regex(pattern: &str, insensitive: bool, only_one: bool) {
    let mut contents = Vec::new();

    if let Ok(dir) = fs::read_dir(&*get_fortunes_dir()) {
        for file in dir {
            let content = read_fortune_file(file.unwrap().path().to_str().unwrap());
            contents.push(content);
        }
    }

    let pattern_mod = if insensitive {
        &pattern.to_lowercase()
    } else {
        pattern
    };

    let re = Regex::new(pattern_mod).unwrap();

    'do_loop: for content in contents {
        for entry in content.split("\n%\n") {
            let entry_mod = if insensitive {
                &entry.to_lowercase()
            } else {
                entry
            };

            if re.is_match(entry_mod) {
                println!("{}\n%", entry);

                // Break the loop as soon as we found a match
                if only_one {
                    break 'do_loop;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fortune_dir() {
        assert_eq!(get_fortunes_dir().len() > 0, true);
    }

    #[test]
    fn test_get_rand_file() {
        assert_eq!(get_rand_file().is_ok(), true)
    }

    #[test]
    fn test_read_fortune_file() {
        if let Ok(file) = get_rand_file() {
            let fortune = read_fortune_file(&file);
            assert_eq!(fortune.len() > 0, true)
        } else {
            panic!("test_read_fortune_file() -> get_rand_file()")
        }
    }

    #[test]
    fn test_get_rand_fortune() {
        let fortune = get_rand_fortune();
        assert_eq!(fortune.len() > 0, true)
    }

    #[test]
    fn test_get_rand_fortune_from_all() {
        assert_eq!(get_rand_fortune_from_all().is_ok(), true)
    }

    #[test]
    fn test_search_fortune() {
        let strings = ["", "a", "aaaaaaabbbbb", "hello", "x"];
        strings
            .iter()
            .for_each(|e| search_fortune(e, true, true, true));
        strings
            .iter()
            .for_each(|e| search_fortune(e, true, true, false));
        strings
            .iter()
            .for_each(|e| search_fortune(e, true, false, false));
        strings
            .iter()
            .for_each(|e| search_fortune(e, false, false, false));
        strings
            .iter()
            .for_each(|e| search_fortune(e, false, true, true));
        strings
            .iter()
            .for_each(|e| search_fortune(e, false, false, true));
    }

    #[test]
    fn test_search_fortune_regex() {
        let strings = [
            "",
            "a",
            "aaaaaaabbbbb",
            "hello",
            "x",
            ".",
            "a.",
            "A.",
            "[a-z]",
            "[A-Z]",
        ];
        strings
            .iter()
            .for_each(|e| search_fortune_regex(e, true, true));
        strings
            .iter()
            .for_each(|e| search_fortune_regex(e, true, false));
        strings
            .iter()
            .for_each(|e| search_fortune_regex(e, false, false));
        strings
            .iter()
            .for_each(|e| search_fortune_regex(e, false, true));
    }
}
