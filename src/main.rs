use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    
    for line in handle.lines() {
        match line {
            Ok(line) => {
                println!("{}", title_case(&line));
            },
            Err(err) => {
                eprintln!("Error reading standard input: {}", err);
                break;
            }
        }
    }
}

fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case_single_word() {
        assert_eq!(title_case("hello"), "Hello");
        assert_eq!(title_case("rust"), "Rust");
    }

    #[test]
    fn test_title_case_multiple_words() {
        assert_eq!(title_case("hello world"), "Hello World");
        assert_eq!(title_case("rust programming language"), "Rust Programming Language");
    }

    #[test]
    fn test_title_case_mixed_case_words() {
        assert_eq!(title_case("hElLo WoRLd"), "Hello World");
        assert_eq!(title_case("RuSt PrOgRaMmInG LaNgUaGe"), "Rust Programming Language");
    }

    #[test]
    fn test_title_case_empty_string() {
        assert_eq!(title_case(""), "");
    }

    #[test]
    fn test_title_case_with_punctuation() {
        assert_eq!(title_case("hello, world!"), "Hello, World!");
        assert_eq!(title_case("rust: programming language"), "Rust: Programming Language");
    }
}
