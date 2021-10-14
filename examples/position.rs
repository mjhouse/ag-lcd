#![no_std]
#![no_main]

use ag_lcd::{Blink, LcdDisplay, Lines};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let d2 = pins.d2;
    let d3 = pins.d3;
    let d4 = pins.d4;
    let d5 = pins.d5;
    // let d6 = pins.d6;
    // let d7 = pins.d7;
    // let d8 = pins.d8;
    // let d9 = pins.d9;

    // let d10 = pins.d10;
    let d11 = pins.d11;
    let d12 = pins.d12;

    let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
        .with_half_bus(d2, d3, d4, d5)
        // .with_full_bus(d2, d3, d4, d5, d6, d7, d8, d9)
        .with_blink(Blink::On)
        .with_lines(Lines::TwoLines)
        // .with_rw(d10)
        .build();

    let data: [[&str; 16]; 2] = [
        [
            "T", "E", "S", "T", "I", "N", "G", "A", "C", "O", "M", "P", "L", "E", "T", "E",
        ],
        [
            "M", "E", "S", "S", "A", "G", "E", "I", "N", "P", "I", "E", "C", "E", "S", "!",
        ],
    ];

    let cols = 16;
    let rows = 2;

    for i in 0..cols {
        for j in 0..rows {
            arduino_hal::delay_ms(1000);
            lcd.set_position(i, j);
            lcd.print(data[j as usize][i as usize]);
        }
    }

    loop {
        arduino_hal::delay_ms(1000);
    }
}
