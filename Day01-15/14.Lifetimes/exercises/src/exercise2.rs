// TODO: Add lifetime annotations to parameters and return type
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
    assert_eq!(result, "long string is long");
}
