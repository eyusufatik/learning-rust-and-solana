use std::{io};

fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    println!("Array is {arr:?}");

    let two_darr = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    println!("Two dimensional array: {two_darr:?}");


    let tuple = (1, 2.3, "anan");

    println!("Tuple is {tuple:#?}");

    let mut vals = String::new();
    io::stdin()
        .read_line(&mut vals)
        .expect("Failed to read line");

    // split vals into two integers
    let mut vals: Vec<i32> = vals
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let index:usize= vals[0].try_into().unwrap();
    let value = vals[1];

    arr[index] = value;

    println!("New array is {arr:?}")
    
}
