/*
 * enum is a type that can represent one item at a time
 *  each item is called a variant
 * enum is not limited to just plain variants
 *  each variant can optionally contain additional data
 */

enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32,i32),
}

enum PromotionDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promotion(PromotionDiscount),
    Custom(String),
}
fn main() {}