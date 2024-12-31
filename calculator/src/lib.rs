use std::io;

/// Gets a number input from user.
///
/// # Returns
///
/// A i32 number that the user enters as an input.
/// If the input is not a 'i32' number this function causes the program to be panic.
pub fn get_number_from_user() -> i32 {
    let mut number = String::new();
    println!("Please enter a number:");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    number.trim().parse().expect("You needed to enter a number")
}

/// Gets an arithmetic operation from user.
///
/// # Returns
///
/// A string that hold the arithmethic operation.
/// If the user enters wrong input this function causes the program to be panic.
pub fn get_arithmetic_operation_from_user() -> String {
    let mut operation = String::new();
    println!("Choose a arithmetic operation (+, -, *, /):");
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let operation = operation.trim();
    match operation {
        "+" | "-" | "*" | "/" => operation.to_string(),
        _ => panic!("You enter a wrong arithmetic operation"),
    }
}

/// Calculates the result of a given arithmetic operation on two given intergers.
///
/// # Arguments
///
/// * first_number - The first interger in the calculation.
/// * second_number - The second interger in the calculation.
/// * operation - The arithmetic operation to preform.
///
/// # Returns
///
/// `Some(f64)` as the result if the operation is successful and there is no overflow.
/// `None` if the operation overflows.
/// Other errors cause panic.
///
/// # Examples
///
/// ```
/// assert_eq!(calculator::calculation(i32::MAX - 1, 2, "+"), None); // Overflow case
/// assert_eq!(calculator::calculation(10, 2, "+"), Some(12.0));
/// assert_eq!(calculator::calculation(10, 2, "-"), Some(8.0));
/// assert_eq!(calculator::calculation(10, 2, "*"), Some(20.0));
/// assert_eq!(calculator::calculation(10, 2, "/"), Some(5.0));
/// ```
pub fn calculation(first_number: i32, second_number: i32, operation: &str) -> Option<f64> {
    match operation {
        "+" => first_number
            .checked_add(second_number)
            .map(|res| res as f64),
        "-" => first_number
            .checked_sub(second_number)
            .map(|res| res as f64),
        "*" => first_number
            .checked_mul(second_number)
            .map(|res| res as f64),
        "/" => {
            if second_number != 0 {
                Some(first_number as f64 / second_number as f64)
            } else {
                panic!("Cannot divide by zero!")
            }
        }
        _ => panic!("Choose a wrong arithmetic operation"),
    }
}
