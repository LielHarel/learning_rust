use simple_text_analyzer::text_analyzer::TextAnalyzer;
use std::io;

fn main() {
    let mut text_analyzer = TextAnalyzer::new();

    loop {
        println!("Please choose an operation (enter a number):");
        println!("1) Enter text for analyzing:");
        println!("2) Get a word frequency:");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let operation_number: u32 = user_input
            .trim()
            .parse()
            .expect("Did not enter a number of operation");

        match operation_number {
            1 => text_analyzer.get_text_to_analyze_from_user(),
            2 => text_analyzer.print_user_word_freq(),
            _ => println!("You enter invalid operation number, try again"),
        }
    }
}
