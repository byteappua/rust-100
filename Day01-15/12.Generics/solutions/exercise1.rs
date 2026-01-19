struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}

fn main() {
    let int_container = Container::new(42);
    let str_container = Container::new(String::from("Hello"));

    println!("Int: {}", int_container.get_value());
    println!("Str: {}", str_container.get_value());

    assert_eq!(*int_container.get_value(), 42);
}
