#![no_std]
#![no_main]

use ag_lcd::{Display, LcdDisplay};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let rs = pins.d12.into_output().downgrade();
    let en = pins.d10.into_output().downgrade();
    let d4 = pins.d5.into_output().downgrade();
    let d5 = pins.d4.into_output().downgrade();
    let d6 = pins.d3.into_output().downgrade();
    let d7 = pins.d2.into_output().downgrade();

    // Setting up LCD
    let delay = arduino_hal::Delay::new();
    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, delay)
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .with_lines(ag_lcd::Lines::TwoLines)
        .with_reliable_init(10000)
        .build();

    lcd.print_two_lines("Hello", "World");
    loop {}
}

trait PrintTwoLines {
    /// Clears the LCD before printing on both lines.
    fn print_two_lines(&mut self, _first_row: &str, _second_row: &str) {}
}

impl<T, D> PrintTwoLines for LcdDisplay<T, D>
where
    T: embedded_hal::digital::v2::OutputPin<Error = core::convert::Infallible> + Sized,
    D: embedded_hal::blocking::delay::DelayUs<u16> + Sized,
{
    /// Clears the LCD before printing on both lines.
    /// No need for the function to be implemented as a method, but is done for convencience and for demonstration
    fn print_two_lines(&mut self, first_row: &str, second_row: &str) {
        self.clear();
        self.set_position(0, 0);
        self.print(first_row);
        arduino_hal::delay_us(100); // A delay, even a very small one, is needed between printing and setting a new position.
        self.set_position(0, 1);
        self.print(second_row);
        self.set_position(0, 0)
    }
}
