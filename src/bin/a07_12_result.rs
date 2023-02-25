// Example
/*
enum Result<T,E> {
    Ok(T),
    Err(E),
}*/

#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice,String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Unable to find the instruction".to_owned()),
    }
}



fn pick_choice(input: &str) -> Result<(),String> {
    let choice = get_choice(input)?;
    println!("Choice: {:?}", choice);
    Ok(())
}


// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
struct Customer {
    name: String,
    age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

fn check_customer_age(cus: &Customer) -> Result<i32,String> {
    match cus.age >= 21 {
        true => Ok(cus.age),
        false => Err("Its age is under 21".to_owned()),
    }
}


fn main() {
    // Example
    let input = "mainmenu";
    /*match get_choice(input) {
        Ok(inner_choice) => println!("Success: {:?}", inner_choice),
        Err(e) => println!("Err:{:?}", e),
    }*/
    pick_choice(input);

    // Activity Result
    let cus_1 = Customer {
        name: "Jay".to_owned(),
        age: 21,
    };
    let check = check_customer_age(&cus_1);
    match check {
        Ok(age) => println!("Its age is {:?}", age),
        Err(e) => println!("{:?}",e),
    }


    // Activity Result ant Question Mark
}