// 1. Generic Lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 2. Struct with Lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 3. Static Lifetime
fn static_example() {
    let s: &'static str = "I have a static lifetime.";
    println!("Static: {}", s);
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Lifetime validity example
    let string3 = String::from("long string is long");
    let result;
    {
        let string4 = String::from("xyz");
        result = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    } // string4 goes out of scope here.

    // println!("The longest string is {}", result); // Error if uncommented: `result` borrows `string4`

    // Struct example
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Important Excerpt: {}", i.part);
    println!("Level: {}", i.level());
    println!("Part announced: {}", i.announce_and_return_part("Start!"));

    static_example();
}
