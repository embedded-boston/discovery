#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::delay;
use pg::led::LEDS;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    iprintln!("Hello, world!");

    loop {
        delay::ms(1_000);
        LEDS[0].on();
        delay::ms(1_000);
        LEDS[0].off();
    }
}
