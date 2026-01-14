use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
#[allow(dead_code)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
#[allow(dead_code)]
enum MutableList {
    Cons(Rc<RefCell<i32>>, Rc<MutableList>),
    Nil,
}

fn main() {
    // 1. Rc<T> Example
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let _b = List::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // 2. RefCell<T> Example
    let value = RefCell::new(5);
    *value.borrow_mut() += 10;
    println!("RefCell value: {:?}", value.borrow());

    // 3. Combining Rc and RefCell
    let val = Rc::new(RefCell::new(5));

    let list_a = Rc::new(MutableList::Cons(Rc::clone(&val), Rc::new(MutableList::Nil)));
    let list_b = MutableList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&list_a));
    let list_c = MutableList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&list_a));

    println!("Value before update: {:?}", val);

    *val.borrow_mut() += 10;

    println!("Value after update: {:?}", val);
    println!("List A: {:?}", list_a);
    println!("List B: {:?}", list_b);
    println!("List C: {:?}", list_c);
}
