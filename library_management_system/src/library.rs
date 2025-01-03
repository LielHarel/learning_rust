//! This module implements a library.
use crate::book;
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Eq)]
struct BookEntry {
    book: book::Book,
    curent_amount: u32,
    amount_of_copies: u32,
}

impl BookEntry {
    fn new(book: book::Book) -> Self {
        BookEntry {
            book,
            curent_amount: 1,
            amount_of_copies: 1,
        }
    }

    fn add_copy_of_book(&mut self) {
        self.curent_amount += 1;
        self.amount_of_copies += 1;
    }

    fn borrow_book(&mut self) -> &book::Book {
        if self.curent_amount <= 0 {
            panic!("Cannot borrow {} book since does not exist", self.book);
        }
        self.curent_amount -= 1;
        &self.book
    }

    fn return_book(&mut self) {
        self.curent_amount += 1;
        if self.curent_amount > self.amount_of_copies {
            self.add_copy_of_book();
        }
    }

    fn is_borrow(&self) -> bool {
        return self.curent_amount != self.amount_of_copies;
    }
}

impl fmt::Display for BookEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\nIt has {} copies in the library and currently {} available for borrowing",
            self.book, self.amount_of_copies, self.curent_amount
        )
    }
}

#[derive(Debug)]
pub struct Library {
    books_list: HashMap<(String, String), BookEntry>,
}

impl Library {
    /// Creates a new instance of a library
    pub fn new() -> Self {
        Library {
            books_list: HashMap::new(),
        }
    }

    /// Adds a new book to the library database.
    /// If the library has the given book, just add another copy of it.
    pub fn add_new_book(&mut self, new_book: book::Book) {
        match self.books_list.get_mut(&new_book.get_id()) {
            Some(book_entry) => {
                if book_entry.book != new_book {
                    panic!("New book has the same ID but it is not the same book as the exist one");
                } else {
                    book_entry.add_copy_of_book();
                }
            }
            None => {
                let _ = self
                    .books_list
                    .insert(new_book.get_id(), BookEntry::new(new_book));
            }
        }
    }

    /// Removes a book (identifies by its name and its author name) from the library if it is not borrowed.
    /// If the booked is borrowed this function panics.
    pub fn remove_book(&mut self, book_name: &str, author_name: &str) {
        let book_id = (book_name.to_string(), author_name.to_string());
        match self.books_list.get(&book_id) {
            Some(book_entry) => {
                if book_entry.is_borrow() {
                    panic!("Cannot remove a borrowed book");
                } else {
                    let _ = self.books_list.remove(&book_id);
                }
            }
            None => println!("The book already doesn't exist"),
        }
    }

    /// Borrows a book by its name and its author name.
    /// If cannot borrow it, the function panics.
    pub fn borrow_book(&mut self, book_name: &str, author_name: &str) -> &book::Book {
        match self
            .books_list
            .get_mut(&(book_name.to_string(), author_name.to_string()))
        {
            Some(book_entry) => book_entry.borrow_book(),
            None => panic!("Try to borrow not exist book"),
        }
    }

    /// Returns a book to the library.
    pub fn return_book(&mut self, book_name: &str, author_name: &str) {
        match self
            .books_list
            .get_mut(&(book_name.to_string(), author_name.to_string()))
        {
            Some(book_entry) => book_entry.return_book(),
            None => println!("Try to return a book that does not belong to the library"),
        }
    }
}

impl fmt::Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for book in self.books_list.values() {
            let _ = write!(f, "{}\n\n", book);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_borrow_not_exist_book() {
        Library::new().borrow_book("Unknown", "Unknown");
    }

    #[test]
    fn test_borrowing_book() {
        let mut library = Library::new();
        library.add_new_book(book::Book::new(
            "Try",
            "Try2",
            book::BookCatagory::Cook,
            2020,
        ));
        library.add_new_book(book::Book::new(
            "Try",
            "Try2",
            book::BookCatagory::Cook,
            2020,
        ));
        assert_ne!(
            library
                .books_list
                .get(&("Try".to_string(), "Try2".to_string())),
            None
        );

        let book = library.borrow_book("Try", "Try2");
        assert_eq!(
            *book,
            book::Book::new("Try", "Try2", book::BookCatagory::Cook, 2020)
        );
    }
}
