mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("Clarinet sound");
        }
    }
}

use crate::sound::instrument::clarinet;

fn main() {
    clarinet();
    clarinet();
    clarinet();
}
