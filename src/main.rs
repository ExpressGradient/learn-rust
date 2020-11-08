#[derive(Debug)] // Add this annotation explicitly to use Debug trait.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32
}

// Methods on Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.breadth
    }

    fn perimeter(&self) -> u32 {
        2 * (self.length + self.breadth)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.breadth >= other.breadth
    }
}

// We can use multiple impl blocks on a single struct.
impl Rectangle {
    // associated function that doesn't take &self as param.
    // associated functions are like new constructors to the struct to return a different instance.
    fn make_square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            breadth: size
        }
    }
}

fn build_user(username: String, email: String) -> User {
    // Field Init Shorthand when variables and fields have same name.
    User {
        username,
        email,
        sign_in_count: 0,
        active: true
    }
}

// Tuple Structs with no named fields.
struct Color(u32, u32, u32);

fn main() {
    // Instance of the user.
    let mut gradient = User {
        username: String::from("Express Gradient"),
        email: String::from("express.gradient@something.com"),
        sign_in_count: 23,
        active: true
    };

    // To change the field, it's entire instance should be mutable.
    gradient.email = String::from("eg@something.com");

    let elliot = User {
        username: String::from("Elliot Alderson"),
        email: String::from("elliot.alderson@protonmail.com"),
        ..gradient
    };
    println!("Elliot Struct: {:?}", elliot); // Use an struct output formatter called Debug.

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    let rectangle = Rectangle {
        length: 10,
        breadth: 20
    };
    println!("Rectangle's area: {}", rectangle.area());
    println!("Rectangle's perimeter: {}", rectangle.perimeter());

    let second_rectangle = Rectangle {
        length: 5,
        breadth: 7
    };
    println!("Can the rectangle hold the second_rectangle: {}", rectangle.can_hold(&second_rectangle));
}