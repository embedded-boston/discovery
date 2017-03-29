#![no_std]
#![no_main]

extern crate pg;
extern crate f3;
// use pg::peripheral;

use f3::peripheral;
#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let (gpioe, rcc) = unsafe { (peripheral::gpioe_mut(), peripheral::rcc_mut()) };

    rcc.ahbenr.write(|w| w.iopeen(true));

    // TODO initialize GPIOE
    gpioe.moder.write(|w| {
        w.moder8(1)
            .moder9(1)
            .moder10(1)
            .moder11(1)
            .moder12(1)
            .moder13(1)
            .moder14(1)
            .moder15(1)
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8(true)
            .odr9(true)
            .odr10(true)
            .odr11(true)
            .odr12(true)
            .odr13(true)
            .odr14(true)
            .odr15(true)
    });

    loop {}
}
