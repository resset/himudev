//! On-board user LEDs
//!
//! - Red = Pin 20
//! - Green = Pin 19
//! - Blue = Pin 21
//! - Yellow = Pin 22
use embedded_hal::digital::v2::OutputPin;
use e310x_hal::gpio::gpio0::{Pin19, Pin20, Pin21, Pin22};
use e310x_hal::gpio::{Output, Regular, Invert};

/// Red LED
pub type RED = Pin20<Output<Regular<Invert>>>;

/// Green LED
pub type GREEN = Pin19<Output<Regular<Invert>>>;

/// Blue LED
pub type BLUE = Pin21<Output<Regular<Invert>>>;

/// Blue LED
pub type YELLOW = Pin22<Output<Regular<Invert>>>;

/// Returns RED, GREEN and BLUE LEDs.
pub fn rgb<X, Y, Z>(
    red: Pin20<X>, green: Pin19<Y>, blue: Pin21<Z>
) -> (RED, GREEN, BLUE)
{
    let red: RED = red.into_inverted_output();
    let green: GREEN = green.into_inverted_output();
    let blue: BLUE = blue.into_inverted_output();
    (red, green, blue)
}

/// Returns all LEDs.
pub fn all<A, B, C, D>(
    red: Pin20<A>, green: Pin19<B>, blue: Pin21<C>, yellow: Pin22<D>
) -> (RED, GREEN, BLUE, YELLOW)
{
    let red: RED = red.into_inverted_output();
    let green: GREEN = green.into_inverted_output();
    let blue: BLUE = blue.into_inverted_output();
    let yellow: YELLOW = yellow.into_inverted_output();
    (red, green, blue, yellow)
}

/// Generic LED
pub trait Led {
    /// Turns the LED off
    fn off(&mut self);

    /// Turns the LED on
    fn on(&mut self);
}

impl Led for RED {
    fn off(&mut self) {
        self.set_low().unwrap();
    }

    fn on(&mut self) {
        self.set_high().unwrap();
    }
}

impl Led for GREEN {
    fn off(&mut self) {
        self.set_low().unwrap();
    }

    fn on(&mut self) {
        self.set_high().unwrap();
    }
}

impl Led for BLUE {
    fn off(&mut self) {
        self.set_low().unwrap();
    }

    fn on(&mut self) {
        self.set_high().unwrap();
    }
}

impl Led for YELLOW {
    fn off(&mut self) {
        self.set_low().unwrap();
    }

    fn on(&mut self) {
        self.set_high().unwrap();
    }
}
