enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

// TODO: Implement a function `sum_list` that takes a reference to a List and returns the sum of its values.

/*
fn sum_list(list: &List) -> i32 {
    // ...
}
*/

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // println!("Sum: {}", sum_list(&list));
}
