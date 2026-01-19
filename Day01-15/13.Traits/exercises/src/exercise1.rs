trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement AppendBar for String

// TODO: Implement AppendBar for Vec<String>

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s); // Should print FooBar
    // assert_eq!(s, "FooBar");

    let v = vec![String::from("Foo")];
    let v = v.append_bar();
    println!("v: {:?}", v); // Should print ["Foo", "Bar"]
    // assert_eq!(v, vec![String::from("Foo"), String::from("Bar")]);
}
