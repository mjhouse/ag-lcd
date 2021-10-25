#![no_std]
#![no_main]

use ag_lcd::{Blink, LcdDisplay, Lines};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
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
        .with_half_bus(d4, d5, d6, d7)
        // .with_full_bus(d0, d1, d2, d3, d4, d5, d6, d7)
        .with_blink(Blink::On)
        .with_lines(Lines::TwoLines)
        .with_rw(rw)
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
