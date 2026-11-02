use std::env;

const LOREM_TEXT: &str = include_str!("../text.txt");
const DEFAULT_WORD_COUNT: usize = 20;

fn generate_lorem(word_count: usize) -> String {
    let words: Vec<&str> = LOREM_TEXT.split_whitespace().collect();

    (0..word_count)
        .map(|index| words[index % words.len()])
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_word_count(input: &str) -> Result<usize, String> {
    let count = input
        .trim()
        .parse::<usize>()
        .map_err(|_| "Enter a positive whole number.".to_string())?;

    if count == 0 {
        return Err("The word count must be greater than 0.".to_string());
    }

    Ok(count)
}

fn run() -> Result<(), String> {
    let arguments: Vec<String> = env::args().skip(1).collect();

    if arguments.len() > 1 {
        return Err("Usage: lorem-ipsum-generator [word_count]".to_string());
    }

    let word_count = match arguments.first() {
        Some(input) => parse_word_count(input)?,
        None => DEFAULT_WORD_COUNT,
    };

    println!("{}", generate_lorem(word_count));
    Ok(())
}

fn main() {
    if let Err(message) = run() {
        eprintln!("Error: {message}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_requested_number_of_words() {
        let result = generate_lorem(25);

        assert_eq!(result.split_whitespace().count(), 25);
        assert!(result.starts_with("What is Lorem Ipsum?"));
    }

    #[test]
    fn uses_the_full_text_file_as_its_source() {
        let source_word_count = LOREM_TEXT.split_whitespace().count();
        let result = generate_lorem(source_word_count);

        assert!(source_word_count > 500);
        assert_eq!(
            result,
            LOREM_TEXT.split_whitespace().collect::<Vec<_>>().join(" ")
        );
    }

    #[test]
    fn rejects_zero() {
        assert!(parse_word_count("0").is_err());
    }

    #[test]
    fn rejects_empty_input() {
        assert!(parse_word_count("").is_err());
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(parse_word_count("ten").is_err());
        assert!(parse_word_count("-3").is_err());
    }
}
