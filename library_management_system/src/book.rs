//! This module implements a book struct that contains all the information about the book.

use std::{fmt, io, str};

const CURRENT_YEAR: u32 = 2025;

/// A book possible catagories.
#[derive(Debug, PartialEq, Eq)]
pub enum BookCatagory {
    Horror,
    Science,
    Fantasy,
    History,
    Drama,
    Cook,
}

impl fmt::Display for BookCatagory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let catagory = match self {
            BookCatagory::Horror => "horror",
            BookCatagory::Science => "science",
            BookCatagory::Fantasy => "fantasy",
            BookCatagory::History => "history",
            BookCatagory::Drama => "drama",
            BookCatagory::Cook => "cook",
        };
        write!(f, "{catagory}")
    }
}

impl str::FromStr for BookCatagory {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "horror" => Ok(BookCatagory::Horror),
            "science" => Ok(BookCatagory::Science),
            "fantasy" => Ok(BookCatagory::Fantasy),
            "history" => Ok(BookCatagory::History),
            "drama" => Ok(BookCatagory::Drama),
            "cook" => Ok(BookCatagory::Cook),
            _ => Err(format!("{input} is invalid book catagory")),
        }
    }
}

/// A struct that holds all the information about a book.
#[derive(Debug, Eq)]
pub struct Book {
    name: String,
    author: String,
    catagory: BookCatagory,
    year_of_publication: u32,
}

impl Book {
    /// Creates a new instance of a book.
    pub fn new(name: &str, autor: &str, catagory: BookCatagory, year_of_publication: u32) -> Self {
        if year_of_publication > CURRENT_YEAR {
            panic!("{year_of_publication} is not valid since the year now is just {CURRENT_YEAR}");
        }
        Book {
            name: name.to_string(),
            author: autor.to_string(),
            catagory,
            year_of_publication,
        }
    }

    /// Returns the tuple (book's name, author's name) which is the ID of a book.
    pub fn get_id(&self) -> (String, String) {
        (self.name.clone(), self.author.clone())
    }

    /// Asks full details about a book from user and returns an instance of [`Book`].
    /// If wrong input enters, the function panics/
    pub fn get_book_from_user() -> Self {
        println!("Please enter book name:");
        let mut book_name = String::new();
        io::stdin()
            .read_line(&mut book_name)
            .expect("Failed to read line");

        println!("Please enter author name:");
        let mut author_name = String::new();
        io::stdin()
            .read_line(&mut author_name)
            .expect("Failed to read line");

        println!("Please enter book's catagory:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let book_catagory: BookCatagory = user_input.trim().parse().expect("Invalid book catagory");

        println!("Please enter book's year of publication:");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let year_of_publication: u32 = user_input.trim().parse().expect("Invalid number");

        Book::new(
            book_name.to_lowercase().trim(),
            author_name.to_lowercase().trim(),
            book_catagory,
            year_of_publication,
        )
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the book '{}' by '{}' is a {} book that was published in {}",
            self.name, self.author, self.catagory, self.year_of_publication
        )
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.author == other.author
            && self.catagory == other.catagory
            && self.year_of_publication == other.year_of_publication
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_exist_catagory_string() {
        assert_eq!(
            "comedy".parse::<BookCatagory>(),
            Err(String::from("comedy is invalid book catagory"))
        );
    }

    #[test]
    #[should_panic]
    fn test_not_exist_publication_year() {
        Book::new(
            "The maze runner",
            "James Smith Dashner",
            BookCatagory::Drama,
            2026,
        );
    }

    #[test]
    fn test_equal_book() {
        let book1 = Book::new(
            &String::from("The maze runner"),
            "James Smith Dashner",
            BookCatagory::Drama,
            2009,
        );

        let book2 = Book::new(
            "The maze runner",
            "James Smith Dashner",
            BookCatagory::Drama,
            2009,
        );

        assert_eq!(book1, book2);
    }

    #[test]
    fn test_not_equal_book() {
        let book1 = Book::new(
            "The maze runner",
            "James Smith Dashner",
            BookCatagory::Drama,
            2010,
        );

        let book2 = Book::new(
            "The maze runner",
            "James Smith Dashner",
            BookCatagory::Drama,
            2009,
        );

        assert_ne!(book1, book2);
    }
}
