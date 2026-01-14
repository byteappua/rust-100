use std::fmt::Display;

// 1. Define Trait
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // Default implementation
    }
}

// 2. Implement Trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Struct using default implementation
pub struct BlogPost {
    pub content: String,
}

impl Summary for BlogPost {}

// 3. Trait as parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news (bound)! {}", item.summarize());
}

// Returning impl Trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Conditional implementation
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let post = BlogPost {
        content: String::from("This is a blog post"),
    };
    println!("New blog post: {}", post.summarize()); // Uses default impl

    notify(&article);
    notify_bound(&tweet);

    let t = returns_summarizable();
    println!("Returned tweet: {}", t.summarize());

    let pair = Pair::new(10, 20);
    pair.cmp_display();
}
