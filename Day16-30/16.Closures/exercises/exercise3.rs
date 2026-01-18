use std::collections::HashMap;
use std::hash::Hash;

// TODO: Implement a `Cacher` struct that can cache results of a closure.
// It should support generic argument types (K) and return types (V).
// Use a HashMap to store cached values.

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Clone,
    V: Clone,
{
    // calculation: ...
    // values: ...
    marker: std::marker::PhantomData<(T, K, V)>, // Remove this when implementing
}

/*
impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        // ...
    }

    fn value(&mut self, arg: K) -> V {
        // Check if value exists in map.
        // If yes, return it.
        // If no, calculate it, insert into map, and return it.
    }
}
*/

fn main() {
    // let mut c = Cacher::new(|x| x + 1);
    // println!("{}", c.value(1)); // Calculates
    // println!("{}", c.value(1)); // Returns cached
    // println!("{}", c.value(2)); // Calculates
}
