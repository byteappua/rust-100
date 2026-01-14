struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    // 1. Basic Iterator usage
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    println!("Iterating over v1:");
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // 2. Consuming Adaptor: sum
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("Sum of v1: {}", total);

    // 3. Iterator Adaptor: map
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 (v1 + 1): {:?}", v2);

    // 4. Iterator Adaptor: filter
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let my_shoes = shoes_in_size(shoes, 10);
    println!("Shoes of size 10: {:?}", my_shoes);

    // 5. Custom Iterator
    let counter = Counter::new();
    println!("Counter values:");
    for i in counter {
        println!("{}", i);
    }

    // Using other iterator methods on custom iterator
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("Complex iterator op result: {}", sum);
}
