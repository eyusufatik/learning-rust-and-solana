fn main() {
    let mut counter: i32 = 0;

    let count = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The count is: {}", count);


    let mut number = 3;
    while number != 0 {
        println!("Number is {number}");

        number -= 1;
    }

    println!("Finished while loop. Number is {number}");

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("The value is: {}", element);
    }

    for index in (0..arr.len()).rev() {
        println!("{}!", arr[index]);
    }
    let slice = &arr[0..2];
    println!("I want to try something: {slice:?}")
}
