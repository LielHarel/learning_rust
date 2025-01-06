//! This module implements a library.
use std::{collections::HashMap, fmt};

mod book;

pub mod book_entry;

pub use book::{Book, BookID};
pub use book_entry::BookEntry;

/// A struct that helps to manage a library and its books.
#[derive(Debug, Default)]
pub struct Library {
    books_list: HashMap<BookID, BookEntry>,
}

impl Library {
    /// Adds a new book to the library database.
    /// If the library has the given book, just add another copy of it.
    pub fn add_new_book(&mut self, new_book: Book) {
        match self.books_list.get_mut(&new_book.get_id()) {
            Some(book_entry) => {
                if book_entry.book != new_book {
                    panic!("New book has the same ID but it is not the same book as the exist one");
                }
                book_entry.add_copy_of_book();
            }
            None => {
                let _ = self
                    .books_list
                    .insert(new_book.get_id(), BookEntry::new(new_book));
            }
        }
    }

    /// Removes a book (identifies by its name and its author name) from the library if it is not borrowed.
    /// If the book is borrowed this function panics.
    pub fn remove_book(&mut self, book_name: &str, author_name: &str) {
        let book_id = BookID::new(book_name, author_name);
        match self.books_list.get(&book_id) {
            Some(book_entry) => {
                if book_entry.is_borrowed() {
                    panic!("Cannot remove a borrowed book");
                }
                let _ = self.books_list.remove(&book_id);
            }
            None => println!("The book already doesn't exist"),
        }
    }

    /// Borrows a book by its name and its author name.
    /// If cannot borrow it, the function panics.
    pub fn borrow_book(&mut self, book_name: &str, author_name: &str) {
        self.books_list
            .get_mut(&BookID::new(book_name, author_name))
            .expect("Try to borrow not exist book")
            .borrow_book();
    }

    /// Returns a book to the library.
    pub fn return_book(&mut self, book_name: &str, author_name: &str) {
        match self
            .books_list
            .get_mut(&BookID::new(book_name, author_name))
        {
            Some(book_entry) => book_entry.return_book(),
            None => println!("Try to return a book that does not belong to the library"),
        }
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for book in self.books_list.values() {
            write!(f, "{}\n\n", book)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::book::{Book, BookCatagory};
    use super::Library;

    #[test]
    #[should_panic]
    fn test_borrow_not_exist_book() {
        Library::default().borrow_book("Unknown", "Unknown");
    }

    #[test]
    #[should_panic]
    fn test_remove_borrowed_book() {
        let mut library = Library::default();
        library.add_new_book(Book::new("Try", "Try2", BookCatagory::Cook, 2020));

        library.borrow_book("Try", "Try2");
        library.remove_book("Try", "Try2");
    }
}
