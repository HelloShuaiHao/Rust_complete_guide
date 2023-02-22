// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//


// Notes:
// * Use a struct to encapsulate the box characteristics

// * Use an enum for the box color
enum Color {
    Red,
    Blue,
    Green,
}

//impl on Color
impl Color {
    fn print_color(&self) {
        match self {
            Color::Red => println!("The color is red"),
            Color::Blue => println!("The color is blue"),
            Color::Green => println!("The color is green"),
        }
    }
}

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}
impl Dimensions {
    fn print_dimensions(&self) {
        println!("height: {:?}",self.height);
        println!("width: {:?}",self.width);
        println!("depth: {:?}",self.depth);
    }
}


// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
struct ShippingBox {
    dimensions: Dimensions,
    color: Color,
    weight: f64,
}
impl ShippingBox {
    fn new(dimensions: Dimensions, color: Color, weight: f64) -> Self{
        Self {
            dimensions,
            color,
            weight,
        }
    }

    fn print(&self) {
        self.dimensions.print_dimensions();
        self.color.print_color();
        println!("weight: {:?}", self.weight);
    }
}




fn main() {
    let small_dimensions = Dimensions {
        height: 10.0,
        depth: 10.0,
        width: 10.0,
    };

    let small_shipping_box = ShippingBox::new(small_dimensions,Color::Red, 1000.0);
    small_shipping_box.print();
}
