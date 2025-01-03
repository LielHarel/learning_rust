pub mod book;
pub mod library;

use std::io;

/// Gets from user information about a new book and adds it to the given library.
pub fn add_new_book_from_user(library: &mut library::Library) {
    let new_book = book::Book::get_book_from_user();
    library.add_new_book(new_book);
}

/// Gets from user book name and author name which is the ID of a book.
fn get_book_id_from_user() -> (String, String) {
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

    (
        book_name.trim().to_lowercase(),
        author_name.trim().to_lowercase(),
    )
}

/// Removes a book the user asks from the library.
pub fn remove_book_user_ask(library: &mut library::Library) {
    let book_id = get_book_id_from_user();
    library.remove_book(&book_id.0, &book_id.1);
}

/// Borrows a book the user asks from the library.
pub fn borrow_book_user_ask(library: &mut library::Library) {
    let book_id = get_book_id_from_user();
    let book = library.borrow_book(&book_id.0, &book_id.1);
    println!("The borrowed book is ==> {book}");
}

/// Returns a book the user asks to the library.
pub fn return_book_user_ask(library: &mut library::Library) {
    let book_id = get_book_id_from_user();
    library.return_book(&book_id.0, &book_id.1);
}
