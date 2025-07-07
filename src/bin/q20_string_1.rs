//Create a string literal and a String object
use text_io::read;
fn main() {
    println!("Hi, please enter your name as string object: ");
    let mut  name:String = read!();
    println!("Hello, {}", name);
    
    //String operations
    
    //string_name.push() -- pushes character or string into a string
    println!("Enter the character you want to push: ");
    let character:char = read!();
    name.push(character); //pushes single character
    println!("The appended string is {}: ", name);
    
    name.push_str(" you"); //pushes &str (string literal)
    println!("The appended string is {} ", name);
    
    // insert -- inserts a character or string literal at a specific index
    println!("Enter the index and character you want to insert: ");
    let ind:usize = read!();//to represent indices use usize (unsigned integer)
    let character:char = read!();
    name.insert(ind,  character); 
    println!("The new string is {}: ", name);
    
    name.insert_str(10,"Bitch");//inserts a string
    println!("The new string is {}: ", name);
    
    //remove/replace
    let replaced = name.replace("hello", "hi"); // Returns a new String
    println!("The string is {}: ", replaced);
    
    let replaced = name.remove(0);   // Removes char at index
    println!("The string is {}: ", replaced);
    
   let replaced =  name.pop(); //removes the last character from the string
    println!("The string is {:?}: ", replaced);
    
    //trimming white space
    let replaced = name.trim();        // Removes leading/trailing whitespace
    println!("The string is {:?}: ", replaced);
    
    let replaced = name.trim_end(); // Removes only trailing
    println!("The string is {:?}: ", replaced);
    
    let replaced = name.trim_start(); // Removes only leading
    println!("The string is {}: ", replaced);
    
    //inspection
    let length = name.len();               // Returns length in bytes
    println!("The string length is {}", length);
    
    let cap = name.capacity();          // Returns allocated capacity
    println!("The string capacity is {}", cap);
    
    let answer = name.is_empty();          // true if string is empty
    println!("Is the string empty? {}", answer);
    
    let answer = name.contains("word");    // true if substring exists
    println!("Does the string contain xyz? {}", answer);
    
    let answer = name.starts_with("he");   // true if starts with "he"
    println!("Does the string start with xyz? {}", answer);
    
    let answer = name.ends_with("ld");     // true if ends with "ld"
    println!("Does the string end with ld? {}", answer);
    
    // Concatenation
    let s1 = String::from("Hello, ");
    println!("The string is {} ", s1);
    
    let s2 = String::from("world!");
    println!("The string is {} ", s2);
    
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed
    println!("The concatenated string is {} ", s3);
    
    //Some more string methods
    let xyz = name.to_uppercase();      // Returns a new String in uppercase
    println!("The string is {}: ", xyz);
    
    let xyz = name.to_lowercase();      // Returns a new String in lowercase
    println!("The string is {}: ", xyz);
    
    let xyz = name.split_whitespace();  // Iterator over words
    println!("The string is {:?}", xyz);//whitespace does not recognise {} formatting
    
    let xyz = name.split(',');   // Iterator over substrings separated by ','
    println!("The string is {:?}", xyz);
    
    let xyz = name.repeat(3);           // Repeats the string 3 times
    println!("The string is {:?}", xyz);

    //clearing the entire string
    let replaced = name.clear();  // Empties the string
    println!("The string is {:?}: ", replaced);


}