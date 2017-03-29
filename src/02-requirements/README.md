# Knowledge Requirements

The main "knowledge requirement" for following along with this tutorial is to
know some Rust. To get "some" knowledge I recommend walking through the
[`Guessing Game` tutorial](https://doc.rust-lang.org/book/guessing-game.html)
and reading through some of the official [The Rust Programming Language Book].
This should give you a sufficient base of knowledge.

[The Rust Programming Language Book]: https://doc.rust-lang.org/book

# Hardware Requirements

Here are the materials we will be using:

- A [STM32 NUCLEO-F303RE] development board. Which is a simple, but powerful
  Arduino-compatible development board.

[STM32 NUCLEO-F303RE]: http://www.st.com/en/evaluation-tools/nucleo-f303re.html
  - You can purchase this board from [Mouser][0] or [Digikey][1].

[0]: http://www.mouser.com/ProductDetail/STMicroelectronics/NUCLEO-F303RE/?qs=%2fha2pyFaduhMljemrTB2TiWNA%252b8NWaDxaSD%252bvQZGxBvfwZmOimB6MjCEAFnAFVgy
[1]: https://www.digikey.com/products/en?keywords=NUCLEO-F303RE

<p align="center">
<img title="STM32 NUCLEO-F303RE" src="assets/f3.jpg" />
</p>

  - We'll refer to this board as "F3" throughout this book.
  - Specs:
      - A STM32F303RET6 microcontroller. This microcontroller has
    - A single core ARM Cortex-M4F processor with hardware support for single
      precision floating point operations and a maximum clock frequency of 72 MHz.

    - 512 KiB of "Flash" memory. (1 KiB = 10**24** bytes)

    - 80 KiB of RAM.

    - many "peripherals": timers, GPIO, I2C, SPI, USART, etc.

    - lots of "pins" that are exposed in the two lateral "headers".

    - Arduino-compatible headers.

    - **IMPORTANT** This microcontroller operates at (around) 3.3V.

    - A second microcontroller: a STM32F103CBT. This microcontroller is actually
      part of an on-board programmer and debugger named ST-LINK and is connected to
      the USB port named "USB ST-LINK".
  - [Datasheet](http://www.st.com/content/ccc/resource/technical/document/datasheet/2c/6f/d7/64/1f/a3/4f/c9/DM00118585.pdf/files/DM00118585.pdf/jcr:content/translations/en.DM00118585.pdf)
  - [Reference Manual](http://www.st.com/content/ccc/resource/technical/document/reference_manual/4a/19/6e/18/9d/92/43/32/DM00043574.pdf/files/DM00043574.pdf/jcr:content/translations/en.DM00043574.pdf)
  - [Nucleo User Manual](http://www.st.com/content/ccc/resource/technical/document/user_manual/98/2e/fa/4b/e0/82/43/b7/DM00105823.pdf/files/DM00105823.pdf/jcr:content/translations/en.DM00105823.pdf)
- One mini-B USB cables. One is required to make the STM32 NUCLEO board
  work.

<p align="center">
<img title="mini-B USB cable" src="assets/usb-cable.jpg">
</p>

> **NOTE** These are **not** the USB cables that ship with pretty much every
> Android phone; those are *micro* USB cables. Make sure you have the right
> thing!

- OPTIONAL. Bring some shields, LEDS, and wires to play around with!

> **FAQ**: Wait, why do I need this specific hardware?

It makes my life and yours much easier.

The material is much, much more approachable if we don't have to worry about
hardware differences. Trust me on this one.

> **FAQ**: Can I follow this material with a different development board?

Maybe? It depends mainly on two things: your previous experience with
microcontrollers and/or whether there already exists a high level crate, like
the [f3], for your development board somewhere.

Regardless, with a different development board, this text would lost most if not
all its beginner friendliness and "easy to follow"-ness, IMO.

If you have a different development board and you don't consider yourself a
total beginner, you are better off reading the [Copper] book which approaches
the Rust on microcontrollers topic in a bottom-up and device agnostic way. Or,
maybe even just read the source of the [f3] crate.

[Copper]: https://japaric.github.io/copper
[f3]: https://github.com/japaric/f3
