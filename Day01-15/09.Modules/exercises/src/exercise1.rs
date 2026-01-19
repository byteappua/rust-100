mod plant {
    struct Vegetable {
        name: String,
        id: i32,
    }

    impl Vegetable {
        fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

fn main() {
    // The following code fails to compile because of visibility rules.
    // Fix the `plant` module so this code works.

    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // Note: The `id` field should remain private!
    // println!("The ID is {}", v.id);
}
