struct User {
    active: bool,
    age: u32,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        age: 31,
        email: String::from("borginer"),
        sign_in_count: 1,
    };

    user1.age = 32;
    
    println!("user1\nactive: {}\nage: {}\nemail: {}\nsign_in_count: {}\n",
    user1.active, user1.age, user1.email, user1.sign_in_count);

    let user2 = build_user(21, String::from("lalala"));
    println!("user2 email: {}", user2.email);
    
    let user3 = User {
        active: false,
        ..user2
    };
    println!("user3 active: {}", user3.active);
}

fn build_user(age: u32, email: String) -> User {
    User {
        active: true,
        age,
        email,
        sign_in_count: 1,
    }
}
