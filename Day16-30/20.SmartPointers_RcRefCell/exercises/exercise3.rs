use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    // TODO: Define children as a vector of shared, mutable nodes.
    // children: ...
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

    // TODO: Modify the value of `leaf` to be 10.
    // ...

    // Verify that `branch` sees the new value of its child.
    // println!("branch child value: {}", branch.borrow().children[0].borrow().value);
}
