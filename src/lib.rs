#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]
#![deny(unused_variables)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![allow(clippy::identity_op)]

//! # AG-LCD
//!
//! This is a rust port of the [LiquidCrystal](https://github.com/arduino-libraries/LiquidCrystal) library. LiquidCrystal
//! is a standard C++ library that allows developers to control a [HITACHI HD44780](https://pdf1.alldatasheet.com/datasheet-pdf/view/63673/HITACHI/HD44780/+435JWUEGSzDpKdlpzC.hv+/datasheet.pdf)
//! LCD screen with one or two 16-character lines. Alternatives to this library (that I've investigated) are:
//!
//! * [lcd](https://crates.io/crates/lcd)
//! * [lcd1602](https://crates.io/crates/lcd1602-rs)
//!
//! I decided to create a more comprehensive solution because existing libraries were either incomplete or somewhat
//! complicated to use. This library uses traits from [embedded-hal](https://crates.io/crates/embedded-hal) and should work
//! with any hardware abstraction layer that uses the same types. Currently this crate has only been tested with [avr-hal](https://github.com/Rahix/avr-hal)
//! and all example code and comments assume you're using avr-hal as well.
//!
//! Most features (blink, cursor, text direction etc.) can be set either through a general `set_` function that accepts
//! one or two arguments (like [set_blink][LcdDisplay::set_blink]), through specific conveniance functions ([blink_on][LcdDisplay::blink_on] rather
//! than [set_blink][LcdDisplay::set_blink]) or with a builder function (like [with_blink][LcdDisplay::with_blink]).
//!
//! If some functions are missing for a settings, its either because it doesn't make sense for that particular setting, or
//! because that feature can only be set *before* the [build][LcdDisplay::build] method is called (in which case only a `with_`
//! function is provided).
//!
//! ## Usage
//!
//! ```
//! use ag_lcd::{Display, Blink, Cursor, LcdDisplay};
//!
//! let peripherals = arduino_hal::Peripherals::take().unwrap();
//! let pins = arduino_hal::pins!(peripherals);
//! let delay = arduino_hal::Delay::new();
//!
//! let rs = pins.d12.into_output().downgrade();
//! let rw = pins.d11.into_output().downgrade();
//! let en = pins.d10.into_output().downgrade();
//! // let d0 = pins.d9.into_output().downgrade();
//! // let d1 = pins.d8.into_output().downgrade();
//! // let d2 = pins.d7.into_output().downgrade();
//! // let d3 = pins.d6.into_output().downgrade();
//! let d4 = pins.d5.into_output().downgrade();
//! let d5 = pins.d4.into_output().downgrade();
//! let d6 = pins.d3.into_output().downgrade();
//! let d7 = pins.d2.into_output().downgrade();
//!
//! let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
//!     // .with_full_bus(d0, d1, d2, d3, d4, d5, d6, d7)
//!     .with_half_bus(d4, d5, d6, d7)
//!     .with_display(Display::On)
//!     .with_blink(Blink::On)
//!     .with_cursor(Cursor::On)
//!     .with_rw(d10) // optional (set to GND if not provided)
//!     .build();
//!
//! lcd.set_cursor(Cursor::Off);
//! lcd.set_blink(Blink::Off);
//!
//! lcd.print("Test message!");
//! ```
//!

mod display;
mod errors;
#[cfg(feature = "i2c")]
#[doc(hidden)]
pub mod i2c;

pub use display::*;
pub use errors::Error;
