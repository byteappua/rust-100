#![no_std]

use embedded_hal::digital::OutputPin;
use embedded_hal::delay::DelayNs;

/// A simple driver that blinks an LED.
///
/// This driver is platform agnostic and can run on any device that implements
/// the `embedded-hal` traits.
pub struct Blinky<P, D> {
    led: P,
    delay: D,
}

impl<P, D> Blinky<P, D>
where
    P: OutputPin,
    D: DelayNs,
{
    /// Create a new Blinky driver.
    pub fn new(led: P, delay: D) -> Self {
        Self { led, delay }
    }

    /// Blink the LED `times` number of times, with `duration_ms` on and off.
    pub fn blink(&mut self, times: usize, duration_ms: u32) -> Result<(), P::Error> {
        for _ in 0..times {
            self.led.set_high()?;
            self.delay.delay_ms(duration_ms);
            self.led.set_low()?;
            self.delay.delay_ms(duration_ms);
        }
        Ok(())
    }
}
