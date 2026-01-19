fn serve_order() {
    println!("Order served!");
}

mod back_of_house {
    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();
        // TODO: Fix the following line to call `serve_order` from the parent module
        // serve_order();
    }

    fn cook_order() {
        println!("Order cooked!");
    }
}

fn main() {
    // This exercise is to fix the code inside `back_of_house`.
    // The main function just runs to prove compilation.
    println!("If this compiles, you might have fixed the path (or just commented it out :P).");
}
