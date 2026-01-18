use std::cell::RefCell;

trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // self is immutable, but RefCell allows interior mutability
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

fn main() {
    let mock = MockMessenger::new();
    mock.send("Hello");
    mock.send("World");

    assert_eq!(mock.sent_messages.borrow().len(), 2);
    assert_eq!(mock.sent_messages.borrow()[0], "Hello");
    println!("MockMessenger works!");
}
