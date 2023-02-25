// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//

// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn to_upper(item: String) -> String{
    item.to_uppercase()
}
fn to_lower(item: String) -> String{
    item.to_lowercase()
}


fn main() {
    let s = "hello world";
    println!("to lower: {:?}", to_lower(s.to_owned()));
    println!("to upper: {:?}", to_upper(s.to_owned()));

}
