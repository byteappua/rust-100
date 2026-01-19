mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("Clarinet sound");
        }
    }
}

// TODO: Use `use` to bring `clarinet` or `instrument` into scope
// so we can call it directly or with a shorter path.

fn main() {
    // Original calls:
    sound::instrument::clarinet();
    sound::instrument::clarinet();
    sound::instrument::clarinet();
}
