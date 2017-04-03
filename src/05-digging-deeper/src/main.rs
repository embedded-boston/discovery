#![no_std]
#![no_main]

extern crate pg;
extern crate f3;

use f3::peripheral;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let gpioa = unsafe { peripheral::gpioa_mut() };
    let rcc = unsafe { peripheral::rcc_mut() };

    rcc.ahbenr.modify(|_, w| w.iopaen(true));
    gpioa.moder.write(|w| w.moder5(0b01));

    loop {
        f3::delay::ms(1_000);
        gpioa.bsrr.write(|w| w.bs5(true));
        f3::delay::ms(1_000);
        gpioa.bsrr.write(|w| w.br5(true));
    }
}
