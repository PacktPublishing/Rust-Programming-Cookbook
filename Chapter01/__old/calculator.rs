
pub struct ArithmeticResults{
    sum: i32,
    difference: i32,
    product: i32,
    quotient: f32,
}

pub fn print_basic_arithmetics(a: i32, b: i32) -> ArithmeticResults {
    ArithmeticResults {
        sum: a + b,
        difference: a - b,
        product: a * b,
        quotient: a / b
    }
}