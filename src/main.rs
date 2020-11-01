// main is the root function for the program
fn main() {
    println!("Hello World");

    let immut_var:i8 = 5; // Variables are immutable by default.
    println!("Value of immut_var is {}", immut_var);

    let mut mut_var:i8 = 2; // But mutable variables can be created using the mut keyword.
    mut_var = mut_var + 1;
    println!("Value of mut_var is {}", mut_var);
    
    const SOME_CONST:i8 = 3; // consts are always immutable, mut keyword doesn't work and type annotation must be given.

    // Shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces = {}", spaces);
}