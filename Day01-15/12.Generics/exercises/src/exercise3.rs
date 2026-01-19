struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // TODO: Implement mixup.
    // It takes `self` and `other: Point<V, W>`.
    // It returns Point<T, W>.
    /*
    fn mixup...
    */
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    // let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // assert_eq!(p3.x, 5);
    // assert_eq!(p3.y, 'c');
}
