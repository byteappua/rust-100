use std::collections::HashMap;
use std::hash::Hash;

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Clone,
    V: Clone,
{
    calculation: T,
    values: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        if self.values.contains_key(&arg) {
            self.values.get(&arg).cloned().unwrap()
        } else {
            let v = (self.calculation)(arg.clone());
            self.values.insert(arg, v.clone());
            v
        }
    }
}

fn main() {
    let mut c = Cacher::new(|x| x + 1);

    println!("First call (1): {}", c.value(1)); // Calculates and prints 2
    println!("Second call (1): {}", c.value(1)); // Returns cached 2
    println!("Third call (2): {}", c.value(2)); // Calculates and prints 3

    let mut string_cacher = Cacher::new(|x: &str| x.len());
    println!("Len of 'hello': {}", string_cacher.value("hello"));
}
