// 1. Function Pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 2. Returning Closures
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// 3. Declarative Macros (macro_rules!)
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    // Function Pointers
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // Using tuple struct constructor as function pointer
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..5).map(Status::Value).collect();
    println!("Statuses: {:?}", list_of_statuses);

    // Returning Closures
    let closure = returns_closure();
    println!("Closure result: {}", closure(1));

    // Macros
    let v = my_vec![1, 2, 3];
    println!("Vector from macro: {:?}", v);

    // Testing macro with different types
    let v2 = my_vec!["a", "b"];
    println!("Vector 2 from macro: {:?}", v2);
}
