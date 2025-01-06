use crate::library::{self, Library};
use std::io;

pub enum Operations {
    AddNewBook,
    RemoveBook,
    BorrowBook,
    ReturnBook,
    PrintBook,
}

impl TryFrom<u32> for Operations {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Operations::AddNewBook,
            2 => Operations::RemoveBook,
            3 => Operations::BorrowBook,
            4 => Operations::ReturnBook,
            5 => Operations::PrintBook,
            _ => return Err("Value must be a number from 1 to 5".to_string()),
        })
    }
}

impl Operations {
    pub fn get_operation_from_user() -> Result<Operations, String> {
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
    pub fn menu(&mut self) -> bool {
        let user_operation = Operations::get_operation_from_user();
        match user_operation {
            Ok(operation) => {
                self.preform_library_operation(operation);
                true
            }
            Err(err) => {
                print!("{err}\n");
                false
            }
        }
    }

    /// Preforms an operation on library according to the given operation enum.
    pub fn preform_library_operation(&mut self, operation: Operations) {
        match operation {
            Operations::AddNewBook => self
                .library
                .add_new_book(library::Book::get_book_from_user()),
            Operations::RemoveBook => self.library.remove_book(
                &LibraryCli::get_book_name_from_user(),
                &LibraryCli::get_book_author_from_user(),
            ),
            Operations::BorrowBook => self.library.borrow_book(
                &LibraryCli::get_book_name_from_user(),
                &LibraryCli::get_book_author_from_user(),
            ),
            Operations::ReturnBook => self.library.return_book(
                &LibraryCli::get_book_name_from_user(),
                &LibraryCli::get_book_author_from_user(),
            ),
            Operations::PrintBook => println!("{}", self.library),
        }
    }
}
