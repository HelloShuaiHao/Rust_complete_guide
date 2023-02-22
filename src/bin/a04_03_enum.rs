// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//

//use std::intrinsics::prefetch_read_instruction;

// * Use an enum with color names as variants
enum ColorName {
    Green,
    Blue,
    Red,
}


// * Use a function to print the color name
// * The function must use the enum as a parameter
fn color_print(color: ColorName) {
    // * Use a match expression to determine which color to print
    match color {
        ColorName::Red => println!("The color is red"),
        ColorName::Green => println!("The color is green"),
        ColorName::Blue => println!("The color is blue"),
    }
}

fn main() {
    let color = ColorName::Red;
    color_print(color);
}
