// Create floating point variables (f32, f64) and perform basic arithmetic

use text_io::read;
fn main() {
    
    println!("Enter two floating point values: ");
    let  fl_val1 : f32 = read!();
    let fl_val2 : f64 = read!();
    
    //carrying out basic arithmetic operations
    let  fl_val1 = fl_val1 as f64; //explicit type casting + variable shadowing
    // RUST does not allow operations between f32 and f64 as no implicit type casting is allowed.
    
    let sum : f64 = fl_val1 + fl_val2;
    let product = fl_val1 * fl_val2;
    let quotient = fl_val1 / fl_val2;
    let remainder = fl_val1 % fl_val2;
    let difference = fl_val1 - fl_val2;
    
    println!("Sum = {} \nDifference = {} \nQuotient = {} \n", sum, difference, quotient);
    println!("Product = {} \nRemainder = {} \n", product, remainder);
    
}
