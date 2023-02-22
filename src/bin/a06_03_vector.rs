// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector



fn main() {
    //vector macro
    let numbers_1 = vec![1,2,3];
    for num in numbers_1 {
        println!("{:?}", num);
    }

    //Vec struct
    let mut numbers_2 = Vec::new();
    numbers_2.push(1);
    numbers_2.push(2);
    numbers_2.pop();
    println!("The length of numbers_2 is: {:?}", numbers_2.len());

    //Demo
    let scores = vec![
        Test { score: 90},
        Test { score: 88},
        Test { score: 93},
    ];
    for test in scores {
        println!("Score = {:?}", test.score);
    }

    //activity
    println!();
    // * Use a vector to store 4 numbers
    let mut my_vec = Vec::new();
    my_vec.push(10);   // It has to be mutable
    my_vec.push(20);
    my_vec.push(30);
    my_vec.push(40);

    //a much convenient way
    let my_vec = vec![10, 20, 30, 40];
    // * Iterate through the vector using a for..in loop
    // my_vec moved dr to this implicit call to 'into_iter()'
    for num in &my_vec {    // the owner is still 'main function'
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    println!("The length of my_vec is: {:?}", my_vec.len());
}

//Demo
struct Test {
    score: i32,
}
