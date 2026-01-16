use embedded_demo::Blinky;
use embedded_hal::digital::{OutputPin, ErrorType};
use embedded_hal::delay::DelayNs;
use std::thread;
use std::time::Duration;

// Mock implementation of an Output Pin
struct MockPin {
    name: String,
    state: bool,
}

impl MockPin {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: false,
        }
    }
}

impl ErrorType for MockPin {
    type Error = core::convert::Infallible;
}

impl OutputPin for MockPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        println!("[{}] LED OFF (Low)", self.name);
        self.state = false;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        println!("[{}] LED ON (High)", self.name);
        self.state = true;
        Ok(())
    }
}

// Mock implementation of Delay
struct MockDelay;

impl DelayNs for MockDelay {
    fn delay_ns(&mut self, ns: u32) {
        // Convert nanoseconds to duration.
        // For simulation purposes, we might want to speed it up or keep it real time.
        // Here we keep it real time but maybe print a message if it's long.
        let duration = Duration::from_nanos(ns.into());
        if duration.as_millis() > 100 {
             println!("[Delay] Sleeping for {:?}", duration);
        }
        thread::sleep(duration);
    }
}

fn main() {
    println!("=== Embedded Rust Demo (Host Simulation) ===");
    println!("This program simulates an embedded device on your host machine.");
    println!("It uses the same driver code that would run on a microcontroller.");
    println!();

    let pin = MockPin::new("GPIO_13");
    let delay = MockDelay;

    let mut blinky = Blinky::new(pin, delay);

    println!("Starting blink sequence...");
    // Blink 5 times, 200ms on/off
    match blinky.blink(5, 200) {
        Ok(_) => println!("Blink sequence completed successfully."),
        Err(_) => println!("Error during blink sequence."),
    }
}
