use simple_text_analyzer::text_analyzer::TextAnalyzer;
use std::io;

fn main() {
    let mut text_analyzer = TextAnalyzer::default();

    loop {
        println!("Please choose an operation (enter a number):");
        println!("1) Enter text for analyzing:");
        println!("2) Get a word frequency:");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse::<u8>() {
            Ok(operation_number) => match operation_number {
                1 => text_analyzer.get_text_to_analyze_from_user(),
                2 => text_analyzer.print_user_word_freq(),
                _ => println!("You enter invalid operation number, try again"),
            },
            Err(err_msg) => {
                println!("You enter invalid input {err_msg}. Please try again");
            }
        }
    }
}
