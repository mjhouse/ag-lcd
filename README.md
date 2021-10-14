# AG-LCD

[![ag-lcd docs](https://img.shields.io/badge/docs-ag--lcd-blue)](https://mjhouse.github.io/ag-lcd/ag_lcd/index.html)

**This library has only been tested on an Arduino Nano. If you test it on other systems and find bugs
please add issues to this project so that I can fix them.**

This is a rust port of the [LiquidCrystal](https://github.com/arduino-libraries/LiquidCrystal) library. LiquidCrystal 
is a standard C++ library that allows developers to control a [HITACHI HD44780](https://pdf1.alldatasheet.com/datasheet-pdf/view/63673/HITACHI/HD44780/+435JWUEGSzDpKdlpzC.hv+/datasheet.pdf) 
LCD screen with one or two 16-character lines. Alternatives to this library (that I've investigated) are:

* [lcd](https://crates.io/crates/lcd)
* [lcd1602](https://crates.io/crates/lcd1602-rs)

I decided to create a more comprehensive solution because existing libraries were either incomplete or somewhat
complicated to use. This library relies on [avr-hal](https://github.com/Rahix/avr-hal) as a dependency and expects 
that downstream projects will also be using avr-hal.

## Building

You'll need to use nightly to compile this project- current there is an issue ([#124](https://github.com/Rahix/avr-hal/issues/124)) in avr-hal that requires nightly-2021-01-07 or older.

## Usage

```
use ag_lcd::{Display, Blink, Cursor, LcdDisplay};

let peripherals = arduino_hal::Peripherals::take().unwrap();
let pins = arduino_hal::pins!(peripherals);

let d12 = pins.d12;
let d11 = pins.d11;
let d10 = pins.d10;

let d2 = pins.d2;
let d3 = pins.d3;
let d4 = pins.d4;
let d5 = pins.d5;

// must provide type for the variable so that rustc
// can deduce default type values for pins.
let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    .with_half_bus(d2, d3, d4, d5)
    .with_display(Display::On)
    .with_blink(Blink::On)
    .with_cursor(Cursor::On)
    .with_rw(d10)
    .build();

lcd.set_cursor(Cursor::Off);
lcd.set_blink(Blink::Off);

lcd.print("Test message!");
```

## Examples

You can find examples [here](examples/README.md).