use crate::library::{self, Library};
use std::io;

const ADD_NEW_BOOK: u32 = 1;
const REMOVE_BOOK: u32 = 2;
const BORROW_BOOK: u32 = 3;
const RETURN_BOOK: u32 = 4;
const PRINT_BOOKS: u32 = 5;

/// Enum that represents a possible library operations
pub enum LibraryOperations {
    AddNewBook,
    RemoveBook,
    BorrowBook,
    ReturnBook,
    PrintBook,
}

impl TryFrom<u32> for LibraryOperations {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            ADD_NEW_BOOK => LibraryOperations::AddNewBook,
            REMOVE_BOOK => LibraryOperations::RemoveBook,
            BORROW_BOOK => LibraryOperations::BorrowBook,
            RETURN_BOOK => LibraryOperations::ReturnBook,
            PRINT_BOOKS => LibraryOperations::PrintBook,
            _ => return Err("Value must be a number from 1 to 5".to_string()),
        })
    }
}

impl LibraryOperations {
    /// Gets a library operation from a user that he want to preform
    /// and returns it if the input was correct, otherwise return error message.
    pub fn get_operation_from_user() -> Result<LibraryOperations, String> {
        println!("Please choose an operation (enter a number):");
        println!("{ADD_NEW_BOOK}) Add a new book to the library:");
        println!("{REMOVE_BOOK}) Remove a book from the library:");
        println!("{BORROW_BOOK}) Borrow book from library:");
        println!("{RETURN_BOOK}) Return book to the library:");
        println!("{PRINT_BOOKS}) Print books in the library:");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let operation_numer = match user_input.trim().parse::<u32>() {
            Ok(number) => number,
            Err(err) => return Err(err.to_string()),
        };
        operation_numer.try_into()
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
            LibraryOperations::AddNewBook => self
                .library
                .add_new_book(library::Book::get_book_from_user()),
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
            LibraryOperations::PrintBook => println!("{}", self.library),
        }
    }
}
