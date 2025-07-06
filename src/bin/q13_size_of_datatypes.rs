//  Use `std::mem::size_of` to find the size of different data types

fn main() {
    //  i8
    let size = std::mem::size_of::<i8>();
    println!("i8 = {:?} bytes.", size);

    //  i16
    let size = std::mem::size_of::<i16>();
    println!("i16 = {:?} bytes.", size);

    //  i32
    let size = std::mem::size_of::<i32>();
    println!("i32 = {:?} bytes.", size);

    //  i64
    let size = std::mem::size_of::<i64>();
    println!("i64 = {:?} bytes.", size);

    //  u8
    let size = std::mem::size_of::<u8>();
    println!("u8 = {:?} bytes.", size);

    //  i16
    let size = std::mem::size_of::<i16>();
    println!("u16 = {:?} bytes.", size);

    //  u32
    let size = std::mem::size_of::<u32>();
    println!("u32 = {:?} bytes.", size);

    //  u64
    let size = std::mem::size_of::<u64>();
    println!("u64 = {:?} bytes.", size);

    //  f32
    let size = std::mem::size_of::<f32>();
    println!("f32 = {:?} bytes.", size);

    //  f64
    let size = std::mem::size_of::<f64>();
    println!("f64 = {:?} bytes.", size);

    //  String
    let size = std::mem::size_of::<String>();
    println!("String = {:?} bytes.", size);

    //  char
    let size = std::mem::size_of::<char>();
    println!("Char = {:?} bytes.", size);

    //  Boolean
    let size = std::mem::size_of::<bool>();
    println!("Boolean = {:?} bytes.", size);
    
    // this is a single line comment
    /*
    as usual - multi line comment
     */
}