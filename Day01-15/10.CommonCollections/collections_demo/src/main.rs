use std::collections::HashMap;

fn main() {
    let v = vec![1, 2, 3];
    println!("Vector: {:?}", v);

    let s = String::from("Hello");
    println!("String: {}", s);

    let mut map = HashMap::new();
    map.insert("Blue", 10);
    println!("Map: {:?}", map);
}
