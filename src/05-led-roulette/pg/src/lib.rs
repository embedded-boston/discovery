//! Playground

#![feature(asm)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![no_std]

#[macro_reexport(bkpt, iprint, iprintln)]
#[macro_use]
extern crate f3;

#[doc(hidden)]
pub use f3::delay;

#[doc(hidden)]
pub use f3::itm;

/// LEDs
pub mod led {
    pub use f3::led::{LEDS, Led};
}

#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::delay::init();
    f3::led::init();
    f3::itm::init();
}

#[doc(hidden)]
#[export_name = "_default_exception_handler"]
pub unsafe extern "C" fn exception_handler() {
    bkpt!();

    loop {}
}

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt() {
    bkpt!();

    loop {}
}
