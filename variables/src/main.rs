fn main() {
    let mut x = 6;
    println!("Value of x is {x}");
    x = 7;
    println!("Value of x is {x}");

    const SECONDS_IN_A_DAY: u32 = 60 * 60 * 24;
    println!("There are {SECONDS_IN_A_DAY} seconds in a day");

    let x = 6;
    {
        let x = x * 3;
        println!("Value of x is {x}");
    }
    println!("Value of x is {x}");

    let spaces = "   ";
    println!("Value of spaces is {spaces}");
    let spaces = spaces.len();
    println!("Value of spaces is {spaces}");
}
