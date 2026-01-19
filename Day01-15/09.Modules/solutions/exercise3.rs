fn serve_order() {
    println!("Order served!");
}

mod back_of_house {
    #[allow(dead_code)]
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {
        println!("Order cooked!");
    }
}

fn main() {
    println!("Solution compiled successfully!");
}
