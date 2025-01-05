use library_management_system::library;
use library_management_system::library_cli;
use std::io;

fn main() {
    let mut library = library::Library::default();
    loop {
        println!("Please choose an operation (enter a number):");
        println!("1) Add a new book to the library:");
        println!("2) Remove a book from the library:");
        println!("3) Borrow book from library:");
        println!("4) Return book to the library:");
        println!("5) Print books in the library:");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let operation_number: u32 = user_input
            .trim()
            .parse()
            .expect("Did not enter a number of operation");

        match operation_number {
            1 => library_cli::add_new_book_from_user(&mut library),
            2 => library_cli::remove_book_user_ask(&mut library),
            3 => library_cli::borrow_book_user_ask(&mut library),
            4 => library_cli::return_book_user_ask(&mut library),
            5 => println!("{library}"),
            _ => println!("Enter invalid operation number, try again"),
        }
    }
}
