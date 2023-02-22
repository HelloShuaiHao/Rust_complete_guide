
// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant

#[derive(Debug)]
enum Ticket {
    Backstage(String,i32),
    Vip(String,i32),
    Standard(i32),
}

// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
fn main() {
    let tickets = vec![
        Ticket::Backstage("Jay".to_owned(), 50),
        Ticket::Vip("Kate".to_owned(), 100),
        Ticket::Standard(10),
    ];
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("Backstage=> name:{:?}, price:{:?}", name, price)
            },
            Ticket::Vip(name, price) => {
                println!("Vip      => name:{:?}, price:{:?}", name, price)
            },
            Ticket::Standard( price) => {
                println!("Standard => price:{:?}", price)
            },
        };
    }
}