// main is the root function for the program
fn main() {
    println!("Hello World");

    let _immutable_var:i8 = 5; // Variables are immutable by default.

    let mut _mut_var:i8 = 2; // But mutable variables can be created using the mut keyword.
    _mut_var = _mut_var + 1;
    
    const _SOME_CONST:i8 = 3; // consts are always immutable, mut keyword doesn't work and type annotation must be given.

    // Shadowing
    let _spaces = "   ";
    let _spaces = _spaces.len();

    // Scalar data types
    // Integers
    let _unsigned_int: u8 = 2; // Unsigned Int which takes 8 bits of space.
    let _signed_int: i8 = -10; // Signed Int which takes 8 bits of space.

    // There are sizes of 8, 16, 32, 64, 128 and arch-isize/usize which depends on the operating system's architecture.
    // But default is 32 bit.

    // Other forms of writing ints for more readability.
    let _decimal_int = 98_222;
    let _hex_int = 0xff;
    let _octal_int = 0o77;
    let _binary_int = 0b111_000;
    let _byte_int = b'A';

    // Some stuff about Integer overflow in notes

    // Floating Point Numbers
    let _float_32: f32 = 3.2; // 32 bit floating num.
    let _float_64: f64 = 4.3; // 64 bit floating num.

    // f32 is single-precision float and f64 has double precision.

    // Boolean Type
    let _login_status: bool = false;

    // Character Type
    // Declare char type using single quotes.
    let _char_type: char = 'z';

    // Compound Types
    // Tuple
    let tup: (i8, u8, bool) = (-2, 3, false);
    let (_x, _y, _z) = tup; // Pattern matching for destructuring a tuple.
    let _tup_bool: bool = tup.2; // Accessing a tuple's element.

    // Array
    let _arr: [i8; 5] = [1, 2, 3, 4, 5];
    let _short_hand_arr: [i8; 5] = [3; 5]; // This will create an array of five 3s
}