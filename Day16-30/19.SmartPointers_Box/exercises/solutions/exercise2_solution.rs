enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn sum_list(list: &List) -> i32 {
    match list {
        Cons(value, next) => value + sum_list(next),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let sum = sum_list(&list);
    println!("Sum: {}", sum);
    assert_eq!(sum, 6);
}
