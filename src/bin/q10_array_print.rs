// Create an array of 5 integers and print each element
//NOTE: Slight modification has been made here. 

use text_io::read;
fn main() {
  let usr_choice = selection();
    if usr_choice == 1 {
        println!("Enter the name of 5 scientists.");
        string_array_print();
    }
    else if usr_choice == 2 {
        println!("Enter 5 historic years.");
        integer_array_print();
    }
    else if usr_choice == 3 {
        println!("Enter 5 universal constants.");
        float_array_print();
    }
    else {
        println!("Please enter a valid choice!!");
    }
    
}

fn selection() -> i32{
    println!("Press 1 to enter name of 5 Scientists.");
    println!("Press 2 to enter 5 historic years.");
    println!("Press 3 to enter 5 universal constants.");
    let choice:i32 = read!();
    return choice;
}

fn string_array_print() {
    let arr_str:[String; 5] = [read!(), read!(), read!(), read!(), read!()];
    println!("You entered: {:?}", arr_str);
}

fn integer_array_print() {
    let arr_int32: [i32; 5] = [read!(), read!(), read!(), read!(), read!()];
    println!("You entered: {:?}", arr_int32);
}

fn float_array_print() {
    let arr_float:[f64; 5] = [read!(), read!(), read!(), read!(), read!()];
    println!("You entered: {:?}", arr_float);
}
