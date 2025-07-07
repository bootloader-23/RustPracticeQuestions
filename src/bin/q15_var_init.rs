//Create a variable without initializing it, then assign a value late
use text_io::read;
fn main() {
    let num1:i32;
    let num2:i32;
    
    println!("Enter the first number: ");
    num1 = read!();
    println!("Enter the second number: ");
    num2 = read!();
    
    let sum = num1 + num2;
    println!("The sum is: {}", sum);
    
}