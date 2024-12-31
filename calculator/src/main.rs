/// Executes a calculator program that gets inputs from user.
fn main() {
    println!("********** My RUST calculator **********");

    let first_number = calculator::get_number_from_user();
    let second_number = calculator::get_number_from_user();
    let operation = calculator::get_arithmetic_operation_from_user();

    match calculator::calculation(first_number, second_number, operation) {
        Some(value) => println!("Result: {value}"),
        None => panic!("An overflow happens or divide by zero"),
    };
}
