//! This module implements a simple text analyzer

use std::{collections::HashMap, io};

/// Struct that holds the frequencies of words and analyze it.
#[derive(Debug)]
pub struct TextAnalyzer {
    freqs_of_words: HashMap<String, u32>,
}

impl TextAnalyzer {
    /// Creates a new instance of [`TextAnalyzer`] struct
    pub fn new() -> Self {
        TextAnalyzer {
            freqs_of_words: HashMap::new(),
        }
    }
    /// Updates words frequencies according to the words in the text.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_text_analyzer::text_analyzer::TextAnalyzer;
    ///
    /// let mut text_analyzer = TextAnalyzer::new();
    /// text_analyzer.add_info_from_text("Hello world\n");
    /// assert_eq!(text_analyzer.get_word_freq("hello"), 1);
    /// assert_eq!(text_analyzer.get_word_freq("world"), 1);
    /// ```
    pub fn add_info_from_text(&mut self, text: &str) {
        for word in text.replace('\n', " ").split_whitespace() {
            let freq = self.freqs_of_words.entry(word.to_lowercase()).or_insert(0);
            *freq += 1;
        }
    }

    /// Gets a text from user and analyze it in the text analyzer.
    pub fn get_text_to_analyze_from_user(&mut self) {
        println!("Please enter a text to analyze:");
        let mut user_text = String::new();
        io::stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");

        self.add_info_from_text(&user_text);
    }

    /// Gets a word frequency.
    /// If input is not a word, also return 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_text_analyzer::text_analyzer::TextAnalyzer;
    ///
    /// let mut text_analyzer = TextAnalyzer::new();
    /// text_analyzer.add_info_from_text("Hello world");
    /// assert_eq!(text_analyzer.get_word_freq("hello"), 1);
    /// assert_eq!(text_analyzer.get_word_freq("world"), 1);
    /// assert_eq!(text_analyzer.get_word_freq("hey"), 0);
    /// assert_eq!(text_analyzer.get_word_freq("hello\n"), 0);
    /// ```
    pub fn get_word_freq(&self, word: &str) -> u32 {
        if word.chars().any(|c| c.is_whitespace()) {
            // Meaning this is not just a word
            return 0;
        }
        self.freqs_of_words
            .get(&word.to_lowercase())
            .copied()
            .unwrap_or(0)
    }

    /// Gets a word from user and print its frequency.
    pub fn print_user_word_freq(&mut self) {
        println!("Please enter a word to analyze:");
        let mut user_word = String::new();
        io::stdin()
            .read_line(&mut user_word)
            .expect("Failed to read line");

        println!("Word frequency is {}", self.get_word_freq(user_word.trim()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_exist_word_freq() {
        assert_eq!(TextAnalyzer::new().get_word_freq("Hello"), 0)
    }

    #[test]
    fn test_exist_words_freq() {
        let mut text_analyzer = TextAnalyzer::new();
        text_analyzer.add_info_from_text("Hello world");
        assert_eq!(text_analyzer.get_word_freq("hello"), 1);
        assert_eq!(text_analyzer.get_word_freq("world"), 1);
    }

    #[test]
    fn test_not_a_word_returns_zero_freq() {
        let mut text_analyzer = TextAnalyzer::new();
        text_analyzer.add_info_from_text("Hello world");
        assert_eq!(text_analyzer.get_word_freq("hello\n"), 0);
        assert_eq!(text_analyzer.get_word_freq("world\n"), 0);
    }
    #[test]
    fn test_exist_words_in_multiline_text() {
        let mut text_analyzer = TextAnalyzer::new();
        text_analyzer.add_info_from_text("Hello world\n Hello \nworld");
        assert_eq!(text_analyzer.get_word_freq("hello"), 2);
        assert_eq!(text_analyzer.get_word_freq("world"), 2);
    }

    #[test]
    fn test_exist_words_in_hebrew_text() {
        let mut text_analyzer = TextAnalyzer::new();
        text_analyzer.add_info_from_text("שלום עולם");
        assert_eq!(text_analyzer.get_word_freq("שלום"), 1);
        assert_eq!(text_analyzer.get_word_freq("עולם"), 1);
    }
}
