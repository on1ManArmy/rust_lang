struct User {
    username: String,
    age: u32,
    active: bool,
}

pub fn run() {
    let user = User {
        username: String::from("Abhishek"),
        age: 32,
        active: true,
    };

    println!("{} {} {}", user.username, user.active, user.age)
}
