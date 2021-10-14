#![no_std]
#![no_main]

use ag_lcd::{Layout, LcdDisplay};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let d2 = pins.d2;
    let d3 = pins.d3;
    let d4 = pins.d4;
    let d5 = pins.d5;

    // let d10 = pins.d10;
    let d11 = pins.d11;
    let d12 = pins.d12;

    let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
        .with_half_bus(d2, d3, d4, d5)
        // .with_full_bus(d2, d3, d4, d5, d6, d7, d8, d9)
        .with_layout(Layout::LeftToRight)
        // .with_rw(d10)
        .build();

    let message = "TEST";
    let mut flag = false;

    loop {
        arduino_hal::delay_ms(2000);
        lcd.clear();
        lcd.set_position(8, 0);

        if flag {
            lcd.set_layout(Layout::RightToLeft);
            lcd.print(message);
            flag = false;
        } else {
            lcd.set_layout(Layout::LeftToRight);
            lcd.print(message);
            flag = true;
        }
    }
}
