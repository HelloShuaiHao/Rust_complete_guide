// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:

fn main() {
// * Use a variable set to any integer
    let var = 2;
// * Use a match expression to determine which message to display
    match var {
        1 => println!("The value is {:?}",var),
        2 => println!("The value is {:?}",var),
        3 => println!("The value is {:?}",var),
        _ => println!("The value is others"),
    }
// * Use an underscore (_) to match on any value
}
