#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message received: {:?}", self);
    }
}

fn main() {
    let m1 = Message::Write(String::from("hello"));
    m1.call();

    let m2 = Message::Move { x: 10, y: 20 };
    m2.call();

    let m3 = Message::ChangeColor(255, 0, 0);
    m3.call();

    let m4 = Message::Quit;
    m4.call();
}
