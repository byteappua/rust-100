#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Use struct update syntax to create user2
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Comes last
    };

    println!("User 2: {:?}", user2);
    // Note: user1.username was moved to user2, so user1 is partially invalid now.
    // println!("User 1: {:?}", user1); // This would fail
}
