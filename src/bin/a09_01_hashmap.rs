// Example

use std::collections::HashMap;

fn main() {
    let mut people = HashMap::new();
    people.insert("Jay".to_owned(),1);
    people.insert("Fan".to_owned(), 2);
    people.insert("Shuai".to_owned(),3);
    people.remove("Jay");

    match people.get("Jay") {
        Some(data) => println!("data:{:?}",data),
        None => println!("Failed"),
    }

    for (people, num) in people.iter() {
        println!("name:{:?} age:{:?}",people,num);
    }

    for name in people.keys() {
        println!("name: {:?}", name);
    }
    for num in people.values() {
        println!("num: {:?}", num);
    }
}