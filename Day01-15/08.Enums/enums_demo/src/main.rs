#[derive(Debug)]
#[allow(dead_code)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called: {:?}", self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    println!("Home: {:?}", home);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    if let Some(n) = some_number {
        println!("Number: {}", n);
    }
}
