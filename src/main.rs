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
    let arr: [i8; 5] = [1, 2, 3, 4, 5];
    let _short_hand_arr: [i8; 5] = [3; 5]; // This will create an array of five 3s.

    // Functions
    greet();
    let _five: i8 = return_int(3);

    // Control Flow
    // If statement
    let _if_num: i32 = if false { 5 } else { 6 };

    // Loop
    let mut counter: i32 = 0;
    let _loop_num: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    // While Loop
    counter = 5;
    while counter != 0 {
        println!("T-{} seconds", counter);
        counter -= 1;
    }
    println!("LIFTOFF!!!");

    // For Loop
    for element in arr.iter() {
        println!("{}", element);
    }

    // For loop with range
    for num in (1..4).into_iter() {
        println!("{}", num);
    }


    // Ownership: It is a solution to manage heap data.

    // Some stuff about stack and heap in notes.

    /*
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
    */

    // Ownership scope
    {
        {
            let mut _scope_var: i32 = 2; // scope_var's scope starts from here
            _scope_var += 1; // stuff with scope_var
        } // scope_var goes out of scope

        // String Type
        let mut some_str: String = String::from("Hello, "); // from() from String asks for memory from memory allocator at runtime.
        some_str.push_str("World");
        // Memory is automatically returned when the owner goes out of scope.

        // Move
        let s1: String = String::from("some string");
        let _s2: String = s1; // s1 is no longer valid.

        // Clone - Heap data gets copied, but very expensive.
        let s1: String = String::from("some string");
        let _s2: String = s1.clone(); // Both s1 and s2 are valid.

        // Copy
        let x: i32 = 2;
        let _y: i32 = x; // Both x and y are valid.

        // This happens because integer types have sizes known at compile time, so they are stored on stack and easy to make copies.
        // Types which implement the Copy trait, older vars are still usable after assignment.
    }

    // References
    {
        let s1 = String::from("Hello, World!");
        let len: usize = calculate_length(&s1); // Reference of s1 passed as an arg to calculate_length.

        println!("{}, {}", s1, len); // s1 is perfectly valid.
    }
    // Some stuff about dangling pointer in notes.
}

fn greet() {
    println!("Hello User!");
}

fn return_int(input_int: i8) -> i8 {
    input_int // You can omit the return keyword by doing this.
}

// This process of having refs as params to a fn is called borrowing.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Takes in mutable reference. But you can have only one mutable reference to a piece of code in a scope.
// This to mainly prevent data races.
// We also cannot have mutable ref when there is an immutable ref in the scope.
fn add_pls(input_str: &mut String) {
    input_str.push_str(" pls");
}