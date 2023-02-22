// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use an if..else if..else block to determine what to print




// * Use a function that returns a tuple
fn coordinate_return () -> (i32, i32) {
    (3, 4)
}


fn main() {
// * Destructure the return value into two variables
    let (_x,y) = coordinate_return();
    if y > 5 {
        println!("Y is greater than 5");
    }else if y == 5 {
        println!("Y is equal to 5");
    }else {
        println!("Y is less than 5");
    }
}
