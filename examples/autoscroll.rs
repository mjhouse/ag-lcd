#![no_std]
#![no_main]

use ag_lcd::{AutoScroll, Blink, LcdDisplay};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);
    let delay = arduino_hal::Delay::new();

    let rs = pins.d12.into_output().downgrade();
    // let rw = pins.d11.into_output().downgrade();
    let en = pins.d10.into_output().downgrade();
    // let d0 = pins.d9.into_output().downgrade();
    // let d1 = pins.d8.into_output().downgrade();
    // let d2 = pins.d7.into_output().downgrade();
    // let d3 = pins.d6.into_output().downgrade();
    let d4 = pins.d5.into_output().downgrade();
    let d5 = pins.d4.into_output().downgrade();
    let d6 = pins.d3.into_output().downgrade();
    let d7 = pins.d2.into_output().downgrade();

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        // .with_full_bus(d0, d1, d2, d3, d4, d5, d6, d7)
        .with_autoscroll(AutoScroll::Off)
        .with_blink(Blink::On)
        // .with_rw(rw)
        .build();

    let mut flag = false;

    loop {
        arduino_hal::delay_ms(1000);
        lcd.clear();
        lcd.set_position(8, 0);

        if flag {
            lcd.set_autoscroll(AutoScroll::On);
            lcd.print("AutoOn");
            flag = false;
        } else {
            lcd.set_autoscroll(AutoScroll::Off);
            lcd.print("AutoOff");
            flag = true;
        }
    }
}
