// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//




// Notes:
// * Use an enum to create different flavors of drinks
enum DrinkFlavour{
    Hot,
    Cold,
}

// * Use a struct to store drink flavor and fluid ounce information
struct DrinkInfo{
    flavor: DrinkFlavour,
    fluid_ounce: f64,
}



// * Use a function to print out the drink flavor and ounces
fn print_drink(my_drink: DrinkInfo) {
    // * Use a match expression to print the drink flavor
    match my_drink.flavor {
        DrinkFlavour::Cold => println!("My drink flavor is cold"),
        DrinkFlavour::Hot => println!("My drink flavor is hot"),
    }
    println!("The ounce is {:?}",my_drink.fluid_ounce);
}


fn main() {
    let my_drink = DrinkInfo {
        flavor: DrinkFlavour::Cold,
        fluid_ounce: 1.01,
    };
    print_drink(my_drink);
}
