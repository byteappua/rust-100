#[cfg(target_os = "linux")]
fn platform_message() -> &'static str {
    "Linux"
}

#[cfg(not(target_os = "linux"))]
fn platform_message() -> &'static str {
    "Other"
}

fn main() {
    println!("Running on: {}", platform_message());
}
