//! Playground

#![feature(asm)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![no_std]

pub mod gpio;

#[macro_reexport(bkpt, iprint, iprintln)]
#[macro_use]
extern crate f3;

#[doc(hidden)]
pub use f3::delay;

#[doc(hidden)]
pub use f3::itm;


#[doc(hidden)]
#[export_name = "_init"]
pub unsafe fn init() {
    f3::delay::init();
    f3::led::init();
}
