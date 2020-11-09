// Enums allow you to define a type by enumerating its possible variants.
#[derive(Debug)]
enum IpAddrKind<String> {
    V4(String),
    V6(String)
}

// impl block can also be on enums too
impl IpAddrKind<String> {
    fn show(&self) {
        println!("{:?}", self);
    }
}

struct IpAddr {
    kind: IpAddrKind<String>,
    address: String
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String)
}

// Match is like a coin sorting machine.
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This is from {} state", state);
            25
        }
    }
}

// Placeholder value _ which will match for anything and return unit value.
fn match_something(input_num: u32) {
    match input_num {
        10 => println!("its 10"),
        _ => ()
    }
}

// Matches are exhaustive, we must exhaust every last possibility in order for the code to be valid.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    // Variants of the enum are namespaced under its identifier.
    let v4 = IpAddrKind::V4(String::from("127.0.0.1"));

    let v6 = IpAddrKind::V6(String::from("::1"));
    v6.show();

    // Option<T> is an enum which has Some(T) and None as it's variants.
    // Its even included in the prelude so that you don't have to bring it into the scope.
    let _some_num = Some(2);

    // If we want to use None, you need to tell that it may have this(u32 in this case) type for Some.
    let _absent_num: Option<u32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let quarter_type = Coin::Quarter(String::from("Alabama"));
    let quarter = value_in_cents(quarter_type);
    println!("Alabama quarter: {}", quarter);

    // if let, use when the logic is too verbose with match.
    let some_value: Option<u32> = Option::Some(3);
    if let Some(3) = some_value {
        println!("Its 3");
    } else {
        println!("Not 3");
    }
}