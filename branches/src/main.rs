fn main() {
    let num = 3;

    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if num % 5 == 0 {
        println!("num is divisible by 5");
    } else if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 5, 4, 3, or 2");
    }

    let other_num = if num % 3 == 0 {
        5
    } else {
        6
    };

    println!("The value of other_num is: {}", other_num);

}
