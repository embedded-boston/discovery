# Reading Button Inputs

So we can turn an LED on and off, but what about read a button? Luckily the
STM32 Nucleo board has a built in button for us to use on Port C pin 13.
This is the starter code.

``` rust
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
```

If you run the starter code, you will see that you can now press the button and
then the LED will turn on. Now you have IO capabilities. There is nothing more
complicated here under the covers than in the LED only examples earlier. I
encourage you to look in the `pg` directory and grab the [STM32F3 Reference
Manual] and understand what is happening. It may also be useful to look at the
docs for f3, cortex-m and svd2rust. To generate documetnation for all local
crates you can run:

```
$ cargo doc
```

[STM32F3 Reference Manual]: http://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf
