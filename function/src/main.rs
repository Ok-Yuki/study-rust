fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        println!("\nError! Division by zero is not allowed.");
        // To prevent division by zero, halt execution and return to the caller
        return false;
    } else if dividend % divisor > 0 {
        println!("\n{} % {} has a remainder of {}.", dividend, divisor, ())
    }
}



fn is_zero(input: u8) -> bool {
    if input == 0 {
        return true;
    }
    false
}

fn main() {
    if is_zero(0) {
        println!("The value is zero.");
    }
}