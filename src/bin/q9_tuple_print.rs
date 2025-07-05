//Create a tuple with different data types and access its elements

use text_io::read;
fn main() {
    
    let  tup:(String, i32, u64) = ("Atharva".to_string(), 18, 6202147475);
    println!("Name: {}\nAge: {}\nNumber: {}", tup.0, tup.1, tup.2);
    
    let tuple:(String, i32, u64) = (read!(), read!(), read!()); //reading user input in tuple
    println!("Father's Name: {}\nFather's Age: {}\nFather's Number: {}", tuple.0, tuple.1, tuple.2);
    
    let (x, y, z) = tuple; //destructuring the tuple
    println!("{}, {}, {}", x, y, z);
    
}