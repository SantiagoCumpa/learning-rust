// Variables and Mutability in Rust

fn main() {

    // let x = 5;  -> Immutable variable

    let mut x = 5; // Mutable variable
    println!("The value of x is: {x}");

    // with inmutability this would cause an error
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing variable names
    let y = 5;      // initial declaration of y
    let y = y + 1;  // Y is shadowed here

    // 
    { 
        let y = y * 2; // Y is shadowed again in this inner scope
        println!("The value of y in the inner scope is: {y}");
    }

    // Y is back to the previous value here
    println!("The value of y is: {y}");

    // Different types with shadowing
    let spaces = "   ";         // string with spaces
    let spaces = spaces.len();  // shadowed as integer 

    // Uncommenting the following lines would cause an error
    // let spaces = "   ";         // string with spaces
    // spaces = spaces.len();  // shadowed as integer 

}