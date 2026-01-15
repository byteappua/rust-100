use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    // 1. Serialize: Struct -> JSON String
    let p = Person {
        name: String::from("Alice"),
        age: 30,
        phones: vec![String::from("123-4567"), String::from("987-6543")],
    };

    // to_string_pretty makes the JSON easier to read
    let json_str = serde_json::to_string_pretty(&p).unwrap();
    println!("Serialized JSON:\n{}", json_str);

    // 2. Deserialize: JSON String -> Struct
    let data = r#"
        {
            "name": "Bob",
            "age": 25,
            "phones": [
                "555-1212"
            ]
        }"#;

    let p2: Person = serde_json::from_str(data).unwrap();
    println!("\nDeserialized Struct:\n{:?}", p2);
}
