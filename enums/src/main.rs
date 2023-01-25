enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddrKind {
    fn print_ip_type(&self) {
        match self {
            IpAddrKind::V4(a, b, c, d) => println!("Four!, {a}:{b}:{c}:{d}"),
            IpAddrKind::V6(s) => println!("Six!, {}", s)
        };
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("don't care"));

    print_ip_type(&four);
    print_ip_type(&six);

    four.print_ip_type();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("Some number: {}", some_number.unwrap());
    println!("Some number: {}", absent_number.unwrap_or(0));
    
    println!("Some number + 1: {}", plus_one(some_number).unwrap());
    println!("Some number + 1: {}", plus_one(absent_number).unwrap_or(0));

    if let IpAddrKind::V6(_) = four {
        println!("Six detected!");
    } else {
        println!("Np!");
    }
}

fn print_ip_type(kind: &IpAddrKind) {
    match kind {
        IpAddrKind::V4(a, b, c, d) => println!("Four!, {a}:{b}:{c}:{d}"),
        IpAddrKind::V6(s) => println!("Six!, {}", s)
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(a) => Some(a + 1)
    }
}