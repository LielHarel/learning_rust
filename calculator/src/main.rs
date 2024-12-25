use std::io;

fn get_number_from_user() -> f64
{
    let mut number = String::new();
    println!("Please enter a number:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    number.trim().parse().expect("You needed to enter a number")
}

fn main() {
    println!("********** My RUST calculator **********");

    let first_number: f64 = get_number_from_user();
    let second_number: f64 = get_number_from_user();

    let mut operation = String::new();

    println!("Choose an operation (+, -, *, /):");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation = operation.trim();

    let result = match operation {
        "+" => Ok(first_number + second_number),
        "-" => Ok(first_number - second_number),
        "*" => Ok(first_number * second_number),
        "/" => {
            if second_number != 0.0 {
                Ok(first_number / second_number)
            }
            else {
                Err("Cannot divide by zero!")
            }
        },
        _ => Err("Choose invalid operation"),
    };

    match result {
        Ok(result) => println!("Result: {result}"),
        Err(error_msg) => println!("{error_msg}")
    };
}
