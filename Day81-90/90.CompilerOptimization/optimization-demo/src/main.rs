use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

fn main() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("Serialized: {}", json);

    let deserialized: User = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
