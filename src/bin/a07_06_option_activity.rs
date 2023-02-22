// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>



struct Locker {
    name: String,
    assignment: Option<i32>,
}
fn main() {
    let lockers = vec![
        Locker { name:"Jay".to_owned(), assignment: None},
        Locker { name:"Kate".to_owned(), assignment: Some(12)},
        Locker { name:"Jay".to_owned(), assignment: Some(16)},
    ];

    for locker in &lockers {
        println!("name:{:?}", locker.name);
        match locker.assignment {
            Some(num) => println!("Locker number:{:?}", num),
            None => println!("No locker"),
        }
    }
}
