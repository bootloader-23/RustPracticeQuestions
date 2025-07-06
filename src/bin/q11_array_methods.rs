//Create a slice from an array and print it
fn main() {
    let arr:[i32;5] = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);
    
    //slicing an array - sometimes some functions may require sliced arrays
    let arr_sliced = arr.as_slice();
    println!("arr_sliced = {:?}", arr_sliced);
    
    //finding the length of an array
    let len =  arr.len();
    println!("len = {}", len);
    
    //checking whether a give value is present in an array or not.
    let ans = arr.contains(&3);
    println!("ans = {}", ans);
    
    //checking in which value is at a  given index in an array.
    let ind = 3;
    println!("ind = {:?}", arr[ind]); // manual method
    
    let  ind = arr.get(1);//using arrayName.get(index)
    println!("ind = {:?}", ind); // always use {:?} in placeholders for array and string methods
    
    //checking what is the value at a given index of an array
    println!("Value at the 4th index = {}", arr[4]);
    
    //making a new array assigning values from old arrays
    let x = [arr[2], arr[3]];
    println!("x = {:?}", x);
    // array_name[index] can be uses simply like any other variable in RUST
    // and most other languages.
    
    
        
    
   
}

