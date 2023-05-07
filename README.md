# AG-LCD

[![ag-lcd crate](https://img.shields.io/badge/crates.io-v0.1.1-orange)](https://crates.io/crates/ag-lcd)
[![ag-lcd docs](https://img.shields.io/badge/docs-ag--lcd-blue)](https://mjhouse.github.io/ag-lcd/ag_lcd/index.html)


**This library has only been tested on an Arduino Nano (atmega328p) and an Arduino Uno (atmega328p). If you test it on other systems and find bugs
please add issues to this project so that I can fix them.**

This is a rust port of the [LiquidCrystal](https://github.com/arduino-libraries/LiquidCrystal) library. LiquidCrystal 
is a standard C++ library that allows developers to control a [HITACHI HD44780](https://pdf1.alldatasheet.com/datasheet-pdf/view/63673/HITACHI/HD44780.html) 
LCD screen with one or two 16-character lines. Alternatives to this library (that I've investigated) are:

* [lcd](https://crates.io/crates/lcd)
* [lcd1602](https://crates.io/crates/lcd1602-rs)

I decided to create a more comprehensive solution because existing libraries were either incomplete or somewhat
complicated to use. This library uses traits from [embedded-hal](https://crates.io/crates/embedded-hal) and should work
with any hardware abstraction layer that uses the same types. Currently this crate has only been tested with [avr-hal](https://github.com/Rahix/avr-hal)
and all example code and comments assume you're using avr-hal as well.

Functionality for controlling the LCD via I2C using a PCF8574 or a PCF8574A remote I/O expander is also available.

## Building

You'll need to use nightly to compile this project because avr-hal requires nightly-2021-01-07 or older.

NOTE: all examples have been updated to use `nightly-2022-07-10` and manually tested to make sure that they build. Please file an issue if you find that they don't.

## Usage

```rust
// Without I2C

use ag_lcd::{Display, Blink, Cursor, LcdDisplay};

let peripherals = arduino_hal::Peripherals::take().unwrap();
let pins = arduino_hal::pins!(peripherals);
let delay = arduino_hal::Delay::new();

let rs = pins.d12.into_output().downgrade();
let rw = pins.d11.into_output().downgrade();
let en = pins.d10.into_output().downgrade();
// let d0 = pins.d9.into_output().downgrade();
// let d1 = pins.d8.into_output().downgrade();
// let d2 = pins.d7.into_output().downgrade();
// let d3 = pins.d6.into_output().downgrade();
let d4 = pins.d5.into_output().downgrade();
let d5 = pins.d4.into_output().downgrade();
let d6 = pins.d3.into_output().downgrade();
let d7 = pins.d2.into_output().downgrade();

let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    // .with_full_bus(d0, d1, d2, d3, d4, d5, d6, d7)
    .with_half_bus(d4, d5, d6, d7)
    .with_display(Display::On)
    .with_blink(Blink::On)
    .with_cursor(Cursor::On)
    .with_rw(d10) // optional (set to GND if not provided)
    .build();

lcd.set_cursor(Cursor::Off);
lcd.set_blink(Blink::Off);

lcd.print("Test message!");
```

```rust
// With I2C

use ag_lcd::{Cursor, LcdDisplay};
use panic_halt as _;
use port_expander::dev::pcf8574::Pcf8574;


let peripherals = arduino_hal::Peripherals::take().unwrap();
let pins = arduino_hal::pins!(peripherals);
let delay = arduino_hal::Delay::new();

let sda = pins.a4.into_pull_up_input();
let scl = pins.a5.into_pull_up_input();

let i2c_bus = arduino_hal::i2c::I2c::new(peripherals.TWI, sda, scl, 50000);
let mut i2c_expander = Pcf8574::new(i2c_bus, true, true, true);

let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay)
    .with_cursor(Cursor::Off)
    .build();

lcd.print("Test message!");
```
