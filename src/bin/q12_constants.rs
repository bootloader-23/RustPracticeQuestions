use text_io::read;
const G : f32 = 9.8; //recommended to use uppercase names for constants.
// constants can be declared globally, inside functions, or inside specific blocks of code.
fn main() {
    println!("Enter your mass in KGs: ");
    let mass:i32 = read!();
    
    let weight = mass as f32 * G;
    
    println!("Your weight in kgf is {weight}.");
    
}