//Write a program to input the user's name, age and phone number and print the same.

use text_io::read;
use std::string::String;
fn main() {
  
    println!("Hello, Please enter your first  name: ");
    let  name : String = read!(); //reads only till the first space
    
    println!("Please enter your age: ");
    let  age : i32 = read!();
    
    println!("Now, enter your phone no. ");
    let  phone : i64 = read!();
    
    println!("Welcome Mr. {}, you are {} years old and your number is {}.", name, age, phone);
    

}