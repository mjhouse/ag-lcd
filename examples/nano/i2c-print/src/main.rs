#![no_std]
#![no_main]

use ag_lcd::{Cursor, LcdDisplay};
use panic_halt as _;
use port_expander::dev::pcf8574::Pcf8574;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let delay = arduino_hal::Delay::new();

    let sda = pins.a4.into_pull_up_input();
    let scl = pins.a5.into_pull_up_input();

    let i2c_bus = arduino_hal::i2c::I2c::new(dp.TWI, sda, scl, 50000);
    let mut i2c_expander = Pcf8574::new(i2c_bus, true, true, true);

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new_pcf8574(&mut i2c_expander, delay)
        .with_cursor(Cursor::Off)
        .build();

    lcd.print("Hello, World");

    loop {}
}
