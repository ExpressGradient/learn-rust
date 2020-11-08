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
}