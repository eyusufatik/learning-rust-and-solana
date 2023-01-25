struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8);

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

}

fn build_user(email: &str, username: &str) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1
    }
}