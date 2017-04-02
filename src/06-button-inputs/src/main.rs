#![no_std]
#![no_main]
extern crate pg;

use pg::gpio::{Gpio, GpioBank, GpioDirection};

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let led = Gpio::new(5, GpioBank::A, GpioDirection::Out);
    let btn = Gpio::new(13, GpioBank::C, GpioDirection::In);

    loop {
        led.write(!btn.read());
    }
}
