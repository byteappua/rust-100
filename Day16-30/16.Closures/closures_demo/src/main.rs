use std::thread;
use std::time::Duration;

// A simulated expensive calculation
#[allow(dead_code)]
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

#[allow(dead_code)]
fn generate_workout(intensity: u32, random_number: u32) {
    // Define a closure
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly (closure)...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// Cacher implementation (Memoization)
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // 1. Basic Closure
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
    println!("Closure check passed.");

    // 2. Move Closure
    let v = vec![1, 2, 3];
    let equal_to_v = move |z: Vec<i32>| z == v;
    // println!("v: {:?}", v); // Error: value moved
    assert!(equal_to_v(vec![1, 2, 3]));
    println!("Move closure check passed.");

    // 3. Simulated Workout
    // generate_workout(10, 7); // Uncomment to run slow simulation

    // 4. Cacher
    let mut c = Cacher::new(|num| {
        println!("calculating inside cacher...");
        num
    });

    let v1 = c.value(1);
    let v2 = c.value(1); // Should not print "calculating..."

    println!("Cacher values: {}, {}", v1, v2);
}
