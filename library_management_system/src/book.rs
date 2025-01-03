//! This module implements a book struct that contains all the information about the book.

use std::fmt;
use std::str;

const CURRENT_YEAR: u32 = 2025;

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
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "The book '{}' by {} is a {} book that was published in {}",
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
