use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let leaf = Rc::new(RefCell::new(Node {
        value: 3,
        children: vec![],
    }));

    let branch = Rc::new(RefCell::new(Node {
        value: 5,
        children: vec![Rc::clone(&leaf)],
    }));

    println!("Original leaf value: {}", leaf.borrow().value);

    // Modify leaf value
    *leaf.borrow_mut() = Node {
        value: 10,
        children: vec![],
    };

    println!("Modified leaf value: {}", leaf.borrow().value);

    // Check via branch
    println!("Branch child value: {}", branch.borrow().children[0].borrow().value);

    assert_eq!(branch.borrow().children[0].borrow().value, 10);
}
