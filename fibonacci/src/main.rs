use std::io::{self, Write};

fn main() {
    print!("Input which fibonacci number you want: ");
    io::stdout().flush().expect("Couldn't flush");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Couldn't read line");

    let n: u32 = n.trim().parse().expect("Couldn't parse to u32");

    println!("nth fibonacci number with recursive: {}", get_fibonacci_recursive(n - 1));
    println!("nth fibonacci number with iterative: {}", get_fibonacci_iterative(n));

}

fn get_fibonacci_recursive(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        return get_fibonacci_recursive(n - 2) + get_fibonacci_recursive(n - 1);
    }
}


fn get_fibonacci_iterative(n: u32) -> u32 {
    let num: u32;
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    } else {
        num = n - 2
    }

    let mut left = 1;
    let mut right = 1;

    for _ in 0..num {
        let next = left + right;

        left = right;
        right = next;
    }

    right
}