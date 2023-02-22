// Two commonly used types of strings: String(owned) and &str(borrowed string slice)
// Must use an owned String to store in a struct
// Use &str when passing to a function



//Example print_it
fn print_it(data: &str) {
    println!("{:?}", data);
}

// Example Works,
// we can't store a string slice in this manner, cuz it will be cleaned after the struct
struct Employee {
    // name: &str,
    name: String,
}


// Recap
// Strings are automatically borrowed
// Use .to_owned() or String::from() to create an owned copy of a string slice
// Use an owned String when storing in a struct

// Demo
struct LineItem {
    name: String,
    count: i32,
}
fn print_name(name: &str) {
    println!("Name:{:?}", name);
}


// Activity
// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Personality {
    age: u32,
    name: String,
    color: String,
}
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
fn print_info(name: &str, color: &str) {
    println!("{:?}", name);
    println!("{:?}", color);
}

fn main() {
    // Example - Pass to a function
    println!("Example: print_it");
    print_it("A string slice");
    //Two ways to define an owned string
    let owned_string = "owned string".to_owned();
    let another_owned_string = String::from("another_owned_string");
    //When we use an owned string, we have to borrow it
    print_it(&owned_string);
    print_it(&another_owned_string);



    //Example - Works
    let _employee_name = "ShuaiHao".to_owned();
    let _employee_name = String::from("ShuaiHao");
    let _employee = Employee {
        name: _employee_name,
    };


    //Demo
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 1,
        },
        LineItem {
            name: String::from("fruit"),
            count: 3,
        },
    ];
    for item in &receipt {
        print_name(&item.name);
        println!("count: {:?}", item.count);
    }



    // Activity
    // * Create and store at least 3 people in a vector
    // * Iterate through the vector using a for..in loop
    let _persons = vec![
        Personality {
            age: 10,
            name: "Jay".to_owned(),
            color: "blue".to_owned(),
        },
        Personality {
            age: 11,
            name: String::from("Kite"),
            color: String::from("green"),
        },
        Personality {
            age: 12,
            name: "Ley".to_owned(),
            color: "red".to_owned(),
        },
    ];
    for _person in &_persons{
        if  _person.age <= 10 {
            print_info(&_person.name, &_person.color);
        }
    }
}