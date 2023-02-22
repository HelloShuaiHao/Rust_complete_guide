// Example
/*
enum Option<T> {
    Some(T),
    None,
}
 */



// Example 2
struct GroceryItem {
    name: String,
    quantity: i32,
}

fn find_quantity(_item: &GroceryItem, _name: &str) -> Option<i32> {
    if _item.name == _name {
        return Some(_item.quantity)
    }

    None
}


// Demo
struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}



fn main() {
    // Example 1
    struct Customer {
        age: Option<i32>,
        email: String,
    }

    //Some and None are so commonly used so we don't need to type "Option::"
    let _mark = Customer {
        age: Some(22), email: "mark@163.com".to_owned(),
    };
    let becky = Customer {
        age: None, email: "becky@163.com".to_owned(),
    };

    match becky.age {
        Some(age) => println!("Customer is {:?}", age),
        None => println!("customer's age is not provided"),
    }

    // Example 2
    let _grocery_item = vec![
        GroceryItem { name: "bananas".to_owned(), quantity: 4},
        GroceryItem { name: "eggs".to_owned(), quantity: 12},
        GroceryItem { name: "bread".to_owned(), quantity: 10},
    ];

    for item in _grocery_item {
        let quant = find_quantity(&item, "bananas");
        match  quant {
            None => (),
            _ => println!("Banana's quantity is {:?}", quant),
        }
    }


    // Demo
    let my_response = Survey {
        q1: Some(12),
        q2: Some(true),
        q3: Some("Hello".to_owned()),
    };
    match my_response.q1 {
        Some(12) => println!("Q1 is correct"),
        _ => println!("Q1 wasn't responsed"),
    }
    match my_response.q2 {
        Some(boolean) => println!("Q2: {:?}", boolean),
        None => println!("Q2: no response"),
    }


}
