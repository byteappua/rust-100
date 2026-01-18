use std::cell::RefCell;

trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    // TODO: Use RefCell to allow mutating sent_messages inside the immutable send method.
    // sent_messages: ...
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

// TODO: Implement Messenger for MockMessenger.
/*
impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        // ...
    }
}
*/

fn main() {
    let mock = MockMessenger::new();
    mock.send("Hello");
    mock.send("World");

    // assert_eq!(mock.sent_messages.borrow().len(), 2);
}
