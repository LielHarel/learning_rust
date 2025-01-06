use library_management_system::library_cli;

fn main() {
    let mut library_cli = library_cli::LibraryCli::default();
    loop {
        if !library_cli.menu() {
            println!("Try again ....")
        }
    }
}
