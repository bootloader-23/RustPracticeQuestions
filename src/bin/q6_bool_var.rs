//Create a boolean variable and print its value

use text_io::read;
fn main() {
    println!("Enter boolean value: ");
    let bool_var : bool = read!();
    println!("{}", bool_var);
}