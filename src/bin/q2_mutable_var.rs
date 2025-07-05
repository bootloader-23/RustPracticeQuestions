// Take a mutable variable and change its value and print each value.

use text_io::read;
fn main(){
    println!("Please enter the value: ");
    let mut val : i32 = read!();
    println!("The value without mutation is: {}", val);
    
    val += 2;
    println!("After the first mutation, value is {}", val);  
    
    
}