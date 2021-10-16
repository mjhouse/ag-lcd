#![no_std]
#![no_main]

use ag_lcd::{LcdDisplay};
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

    let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en,delay)
        .with_half_bus(d4, d5, d6, d7)
        // .with_full_bus(d0, d1, d2, d3, d4, d5, d6, d7)
        .with_rw(rw)
        .build();

    lcd.display_on();
    arduino_hal::delay_ms(1000);

    lcd.print("ScrollMe");

    let mut count = 0;
    let mut flag = true;

    loop {
        if count == 8 {
            flag = false;
        }
        if count == 0 {
            flag = true;
        }

        if flag {
            arduino_hal::delay_ms(1000);
            lcd.scroll_right(2);
            count += 2;
        } else {
            arduino_hal::delay_ms(1000);
            lcd.scroll_left(2);
            count -= 2;
        }
    }
}
