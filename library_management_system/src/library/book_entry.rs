//! This module implements a book entry in a library database.
//! The entry holds the data that is need for tracing a specifc book in a library.

use std::fmt;

use crate::library::book;

/// A struct that holds all the information that is
/// needed for tracing a specifc book in a library.
#[derive(Debug, PartialEq)]
pub struct BookEntry {
    pub book: book::Book,
    curent_amount: u32,    // current amount in stock for borrowing.
    amount_of_copies: u32, // amount of copies the library holds at all.
}

impl BookEntry {
    /// When create a new book entry the initial value is 1 for the amount of book.
    const INITIAL_AMOUNT_OF_BOOK: u32 = 1;

    /// Creates a new instance of a BookEntry.
    pub fn new(book: book::Book) -> Self {
        BookEntry {
            book,
            curent_amount: BookEntry::INITIAL_AMOUNT_OF_BOOK,
            amount_of_copies: BookEntry::INITIAL_AMOUNT_OF_BOOK,
        }
    }

    /// Updates the book entry to have aditional copy of the tracing book.
    pub fn add_copy_of_book(&mut self) {
        self.curent_amount += 1;
        self.amount_of_copies += 1;
    }

    /// Updates the book entry to have one less book in the stock.
    pub fn borrow_book(&mut self) -> &book::Book {
        if self.curent_amount == 0 {
            panic!("Cannot borrow {} book since does not exist", self.book);
        }
        self.curent_amount -= 1;
        &self.book
    }

    /// Updates the book entry to have one more book in the stock.
    pub fn return_book(&mut self) {
        self.curent_amount += 1;
        if self.curent_amount > self.amount_of_copies {
            self.amount_of_copies = self.curent_amount;
        }
    }

    /// Returns if amount of copies of a book and the amount of books in stock
    /// are different. If different this means there is a copy of the book that is borrowed.
    pub fn is_borrowed(&self) -> bool {
        self.curent_amount < self.amount_of_copies
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

#[cfg(test)]
mod tests {
    use super::BookEntry;
    use crate::library::book::{Book, BookCatagory};

    #[test]
    fn test_borrow_book() {
        let mut book_entry = BookEntry::new(Book::new("try", "try", BookCatagory::Science, 2020));
        assert_eq!(book_entry.amount_of_copies, 1);
        assert_eq!(book_entry.curent_amount, 1);
        book_entry.borrow_book();
        assert_eq!(book_entry.amount_of_copies, 1);
        assert_eq!(book_entry.curent_amount, 0);
    }
}
