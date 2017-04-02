#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate pg;

use pg::delay;
use pg::gpio;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    // Let's create a Gpio object, this will make to Port A Pin 5 where the User LED of the
    // Nucleo is.
    let led: gpio::Gpio = gpio::Gpio::new(5, gpio::GpioBank::A, gpio::GpioDirection::Out);

    // Let us get some mutable state to toggle to our hearts content.
    // false is off, true is on.
    let mut state: bool = false;

    loop {
        // Every 1000 ms display the state and toggle.
        delay::ms(1_000);
        led.write(state);
        state = !state;
    }
}
