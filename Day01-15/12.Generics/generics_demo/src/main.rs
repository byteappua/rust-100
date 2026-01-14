fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // 1. Function Generics
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 2. Struct Generics
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("Integer Point: {:?}", integer);
    println!("Float Point: {:?}", float);
    println!("integer.x = {}", integer.x());

    // 3. Method with specific type
    println!("Distance from origin: {}", float.distance_from_origin());

    // 4. Mixed Types
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("Mixed point: p3.x = {}, p3.y = {}", p3.x, p3.y);
}
