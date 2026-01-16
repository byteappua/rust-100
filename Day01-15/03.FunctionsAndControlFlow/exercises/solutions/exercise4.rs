fn main() {
    let c = 32.0;
    let f = 89.6;

    println!("{}°C = {:.2}°F", c, celsius_to_fahrenheit(c));
    println!("{}°F = {:.2}°C", f, fahrenheit_to_celsius(f));
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}
