use library_management_system::library_cli::LibraryCli;

fn main() {
    let mut library_cli = LibraryCli::default();
    loop {
        if !library_cli.handle() {
            println!("Try again ....");
        }
    }
}
