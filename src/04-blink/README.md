# Blinking An LED
> The "Hello, World" of Embedded Systems Programming.

Alright, let's start by building the following application:

<p align="center">
<img src="http://i.imgur.com/41n82mp.gif">
</p>

We are going to start off with a high-level API to implement this very simple
program, but we will get into some of the details in the next section. The
main goal is to get familiar with building a Rust program, loading it and
debugging it.

Throughout this text we'll be using the starter code that's in the [tutorial-bring-and-blink-rust]
repository. Make sure you always have the latest version of the master branch
because this website tracks that branch. You can get using git with the
following command:

```
$ git clone https://github.com/embedded-boston/tutorial-bring-and-blink-rust.git
```

The starter code is in the `src` directory of that repository. Inside that
directory there more directories named after each chapter of this book. Most of
those directories are starter Cargo projects.

[tutorial-bring-and-blink-rust]: https://embedded-boston.github.io/tutorial-bring-and-blink-rust/

Now, jump into the `src/04-blink` directory. Check the `src/main.rs`
file:

``` rust
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate pg;

use pg::delay;
use pg::gpio;

#[inline(never)]
#[no_mangle]
pub fn main() -> ! {
    let led: gpio::Gpio = gpio::Gpio::new(5, gpio::GpioBank::A, gpio::GpioDirection::Out);
    let mut state: bool = false;

    loop {
        delay::ms(1_000);
        led.write(state);
        state = !state;
    }
}
```

There's some unusual stuff in it: `#![no_main]`, `#[no_mangle]` and `main` is
both `pub` and has signature `fn() -> !`. For now, why those are the way they
are doesn't matter. The only practical implication of all this is that you can't
return from the `main` function.

If you are a careful observer, you'll also notice there is a `.cargo` directory
in the Cargo project as well. This specifies some build time arguments, including
the linker file we will be using.

Alright, let's start by building this program.
