trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(mut self) -> Self {
        self.push_str("Bar");
        self
    }
}

impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
    assert_eq!(s, "FooBar");

    let v = vec![String::from("Foo")];
    let v = v.append_bar();
    println!("v: {:?}", v);
    assert_eq!(v, vec![String::from("Foo"), String::from("Bar")]);
}
