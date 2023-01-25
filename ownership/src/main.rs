fn main() {
    // let s = "hello";

    // let mut s1 = String::new();
    // s1.push_str("hey ");

    // s1.push_str(s);

    // println!("{}", s1);

    let mut s1 = String::from("Hello, World!");
    let mut s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    s1.insert(2, 'k');

    println!("s1 = {}, s2 = {}", s1, s2);

    takes_ownership(s1);

    // println!("After giving owenership s1: {}", s1);

    s2 = takes_and_gives_ownership(s2);

    println!("After giving and taking ownership s2: {}", s2);

    let mut len: usize;
    (s2, len) = calc_len(s2);
    println!("Length of s2: {len}");

    len = calc_len_reference(&s2);
    println!("Length of s2: {len}");

    change_ref(&mut s2);

    println!("After change_ref is called s2: {}", s2);

    let r1 = &s2;

    println!("r1: {}", r1);

    let r2 = &mut s2;
    println!("r2: {}", r2);

    // let ret = dangle();


}

fn takes_ownership(s: String) {
    println!("I came upon some string: {}", s);
}

fn takes_and_gives_ownership(mut s: String) -> String {
    println!("Took and will give back {} after modifying", s);
    s.replace_range(0..1, "M");
    s
}

fn calc_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calc_len_reference(s: &String) -> usize {
    s.len()
}

fn change_ref(s: &mut String) {
    s.push_str("yoooo");
    another_mutable_ref(s);
}

fn another_mutable_ref(s: &mut String) {
    println!("Placeholder s: {}", s);
}

// fn dangle() -> &String {
//     let s = String::from("Hi there!");

//     &s
// }