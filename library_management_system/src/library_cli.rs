use crate::library::{Book, Library};
use num::FromPrimitive;
use num::ToPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use std::io;

/// Enum that represents a possible library operations
#[derive(FromPrimitive, ToPrimitive, Debug)]
#[repr(u8)]
pub enum LibraryOperations {
    AddNewBook = 1,
    RemoveBook,
    BorrowBook,
    ReturnBook,
    PrintBooks,
}

impl LibraryOperations {
    /// Gets a library operation from a user that he want to preform
    /// and returns it if the input was correct, otherwise return error message.
    pub fn get_operation_from_user() -> Result<LibraryOperations, String> {
        println!("Please choose an operation (enter a number):");
        println!(
            "{}) Add a new book to the library:",
            LibraryOperations::AddNewBook
                .to_u8()
                .expect("should not happen")
        );
        println!(
            "{}) Remove a book from the library:",
            LibraryOperations::RemoveBook
                .to_u8()
                .expect("should not happen")
        );
        println!(
            "{}) Borrow book from library:",
            LibraryOperations::BorrowBook
                .to_u8()
                .expect("should not happen")
        );
        println!(
            "{}) Return book to the library:",
            LibraryOperations::ReturnBook
                .to_u8()
                .expect("should not happen")
        );
        println!(
            "{}) Print books in the library:",
            LibraryOperations::PrintBooks
                .to_u8()
                .expect("should not happen")
        );

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let operation_numer = match user_input.trim().parse::<u8>() {
            Ok(number) => number,
            Err(err) => return Err(err.to_string()),
        };
        LibraryOperations::from_u8(operation_numer)
            .ok_or(format!("{operation_numer} invalid number operation"))
    }
}

#[derive(Debug, Default)]
pub struct LibraryCli {
    library: Library,
}

impl LibraryCli {
    fn get_book_name_from_user() -> String {
        println!("Please enter book name:");
        let mut book_name = String::new();
        io::stdin()
            .read_line(&mut book_name)
            .expect("Failed to read line");

        book_name.trim().to_lowercase()
    }

    fn get_book_author_from_user() -> String {
        println!("Please enter author name:");
        let mut author_name = String::new();
        io::stdin()
            .read_line(&mut author_name)
            .expect("Failed to read line");

        author_name.trim().to_lowercase()
    }

    /// Asks from user operation on library and preform it.
    /// This function returns true on success, otherwise false.
    pub fn menu_runner(&mut self) -> bool {
        let user_operation = LibraryOperations::get_operation_from_user();
        match user_operation {
            Ok(operation) => {
                self.preform_library_operation(operation);
                true
            }
            Err(err) => {
                println!("{err}");
                false
            }
        }
    }

    /// Preforms an operation on library according to the given operation enum.
    pub fn preform_library_operation(&mut self, operation: LibraryOperations) {
        match operation {
            LibraryOperations::AddNewBook => self.library.add_new_book(Book::from_user()),
            LibraryOperations::RemoveBook => self.library.remove_book(
                &LibraryCli::get_book_name_from_user(),
                &LibraryCli::get_book_author_from_user(),
            ),
            LibraryOperations::BorrowBook => self.library.borrow_book(
                &LibraryCli::get_book_name_from_user(),
                &LibraryCli::get_book_author_from_user(),
            ),
            LibraryOperations::ReturnBook => self.library.return_book(
                &LibraryCli::get_book_name_from_user(),
                &LibraryCli::get_book_author_from_user(),
            ),
            LibraryOperations::PrintBooks => println!("{}", self.library),
        }
    }
}
