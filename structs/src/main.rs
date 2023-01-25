struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn info() -> String {
        return String::from("This is a rectangle struct!");
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        other_rect.height  <= self.height && other_rect.width <= self.width
    }
}

fn main() {
    let esad = User {
        email: String::from("eyusufatik@hotmail.com"),
        username: String::from("esad"),
        active: true,
        sign_in_count: 1
    };

    println!("Esad: {}", esad.active);

    let orkun = build_user("orkunmkilic@gmail.com", "0xorkun");

    let mut murat = build_user(&String::from("muratkun@gmail.com"), &String::from("murat"));

    println!("Murat: {}, Orkun: {}", murat.email, orkun.email);


    murat = User {
        email: String::from("murat@chainway.xyz"),
        ..murat
    };

    println!("Murat2's email: {}, Murat2's name: {}", murat.email, murat.username);

    let red = Color(255, 0, 0);
    let black = Color(0, 0, 0);

    println!("Black: {}, Red: {}", black.0, red.2);

    let square = Rectangle {
        width: 10,
        height: 10
    };

    println!("Area of square is: {}", area(&square));
    println!("The square: {:?}", square);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("Area of square with struct method is: {}", square.area());
    println!("What we call static methods in other languages: {}", Rectangle::info());
    println!("Area of square with sturct method in a different way: {}", Rectangle::area(&square));

    let rect2 = Rectangle {
        width: 5,
        height: 8
    };
    let rect3 = Rectangle {
        width: 12,
        height: 20
    };

    println!("Can square hold rect2: {}", square.can_hold(&rect2));
    println!("Can square hold rect3: {}", square.can_hold(&rect3));




}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}