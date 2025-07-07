//  Create a reference to a variable using `&`

use text_io::read;
fn main() {
    println!("Enter a number: ");
    let num:i32 = read!();
    
    let y = &num;
    println!("This is the referenced value: {}", y);
    
    println!("The number you entered is: {}", &num);
    
}