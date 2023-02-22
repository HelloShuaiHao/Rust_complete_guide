// if an ampersand(&) isn't used then the ownership will be passed
// and the passed one will be cleaned after the "}"


// Notes:
// * Use a struct for the grocery item
struct GroceryItem{
    // * Use two i32 fields for the quantity and id number
    quantity: i32,
    id: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
impl GroceryItem {
    fn display_quantity(&self){
        println!("The quantity is {:?}", self.quantity);
    }

    fn display_id(&self){
        println!("The ID is {:?}", self.id);
    }

    fn create_new() -> Self {
        Self {
            quantity: 0,
            id: 1,
        }
    }
}

fn main() {
    let grocery_1 = GroceryItem{
        quantity: 100,
        id: 1,
    };
    grocery_1.display_quantity();
    grocery_1.display_id();

    let new_grocery = GroceryItem::create_new();
    new_grocery.display_id();
    new_grocery.display_quantity();

}
