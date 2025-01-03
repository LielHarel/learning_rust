use library_management_system::book;
use library_management_system::library;

fn main() {
    let mut library = library::Library::new();
    library.add_new_book(book::Book::new(
        "Liel",
        "Harel",
        book::BookCatagory::Science,
        2022,
    ));
    library.add_new_book(book::Book::new(
        "Liel",
        "Harel",
        book::BookCatagory::Science,
        2022,
    ));
    library.add_new_book(book::Book::new(
        "KK",
        "Harel",
        book::BookCatagory::Science,
        2022,
    ));

    println!("{library}");

    library.borrow_book("Liel", "Harel");
    println!("{library}");
}
