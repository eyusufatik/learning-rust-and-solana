fn main() {
    another_function(squared(five()));


    let y = {
        let x = 8;
        x + 1
    };

    another_function(y);

}

fn another_function(x: i32) {
    println!("Value of var is {x}");
}

fn five() -> i32 {
    return 5;
}

fn squared(x: i32) -> i32 {
    x * x
}