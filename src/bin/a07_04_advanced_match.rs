enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}


fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        _ => println!("{:?}", n),
    }
    match n {
        3 => println!("three"),
        other => println!("{:?}", other),
    }// more clean and easier to read

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("flat discount 2"),
        Discount::Percent(amount) => println!("Percent discount is {:?}", amount),
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };
    match concert {
        Ticket { price: 50, event} => println!("Event @ 50 = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}