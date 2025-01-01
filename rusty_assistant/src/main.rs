use std::io;

fn main() {
    println!("Hello, I am rusty, your assistant. Please enter your name:");

    let mut user_name= String::new();
    io::stdin()
            .read_line(&mut user_name)
            .expect("Failed to read line");

    println!("Hey {}, have a nice day!", user_name.trim());
}
