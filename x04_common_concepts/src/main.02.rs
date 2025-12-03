// Data types in Rust

fn main () {
    // Without type annotation, Rust throw warning
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");

    // Scalar types -> single value -> integers, floating-point numbers, Booleans, characters

    // Integer 
    
    //           8bit   16bit   32bit   64bit   128bit   arch dep  
    // Signed:    i8     i16     i32     i64     i128     isize
    // Unsigned:  u8     u16     u32     u64     u128     usize   -> always positive
    let a: i8 = -100;
    
    


}