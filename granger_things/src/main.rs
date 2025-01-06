use granger_things::swap;

fn main() {
    // Some tries
    let mut x = 32;
    let mut y = 34;
    println!("x = {x}, y = {y}");
    swap(&mut x, &mut y);
    println!("x = {x}, y = {y}");

    let mut x = "hello";
    let mut y = "world";
    println!("x = {x}, y = {y}");
    swap(&mut x, &mut y);
    println!("x = {x}, y = {y}");
    println!("Hello, world!");
}
