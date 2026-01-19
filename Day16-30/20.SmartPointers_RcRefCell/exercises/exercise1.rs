use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // TODO: Create a list `a` using Rc.
    // let a = ...
    // println!("count after creating a = {}", Rc::strong_count(&a));

    // TODO: Create `b` that shares `a`.
    // let b = ...
    // println!("count after creating b = {}", Rc::strong_count(&a));

    // TODO: Create `c` that shares `a`.
    // let c = ...
    // println!("count after creating c = {}", Rc::strong_count(&a));

    // Check final counts
}
