#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User 1: {:?}", user1);

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Black color: ({}, {}, {})", black.0, black.1, black.2);

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect1.area());
}
