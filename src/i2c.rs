//! Allows interacting  with an lcd display via I2C using a digital port expander

use crate::LcdDisplay;
use core::fmt::Debug;
use embedded_hal::blocking::delay::DelayUs;
use port_expander::{dev::pcf8574, mode::QuasiBidirectional, I2cBus, Pcf8574, Pcf8574a, Pin};
use shared_bus::BusMutex;

impl<'a, D, M, I2C> LcdDisplay<Pin<'a, QuasiBidirectional, M>, D>
where
    D: DelayUs<u16> + Sized,
    M: BusMutex<Bus = pcf8574::Driver<I2C>>,
    I2C: I2cBus,
    <I2C as I2cBus>::BusError: Debug,
{
    /// Descructs pin collection from port expander and constructs LcdDisplay using pins that are
    /// available. For example usage see [`new_pcf8574`] or [`new_pcf8574a`].
    fn from_parts(parts: pcf8574::Parts<'a, I2C, M>, delay: D) -> Self {
        let pcf8574::Parts {
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
        } = parts;
        LcdDisplay::new(p0, p2, delay)
            .with_backlight(p3)
            .with_rw(p1)
            .with_half_bus(p4, p5, p6, p7)
    }

    /// Creates a new [`LcdDisplay`] using PCF8572A for interfacing
    ///
    /// Refer to [Pcf8574a docs] from crate `port-expander` for more information about setup of the
    /// port expander
    ///
    /// This method is only available if the `i2c` feature is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// let peripherals = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(peripherals);
    /// let delay = arduino_hal::Delay::new();
    ///
    /// let sda = pins.a4.into_pull_up_input();
    /// let scl = pins.a5.into_pull_up_input();
    ///
    /// let i2c_bus = arduino_hal::i2c::I2c::new(peripherals.TWI, sda, scl, 50000);
    /// let mut i2c_expander = Pcf8574a::new(i2c_bus, true, true, true);
    ///
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new_pcf8574a(&mut i2c_expander, delay)
    ///     .with_blink(Blink::On)
    ///     .with_cursor(Cursor::Off)
    ///     .build();
    /// ```
    ///
    /// [Pcf8574a docs]: https://docs.rs/port-expander/latest/port_expander/dev/pcf8574/struct.Pcf8574a.html
    #[inline]
    pub fn new_pcf8574a(expander: &'a mut Pcf8574a<M>, delay: D) -> Self {
        Self::from_parts(expander.split(), delay)
    }

    /// Creates a new [`LcdDisplay`] using PCF8572 for interfacing
    ///
    /// Refer to [Pcf8574a docs] from crate `port-expander` for more information about setup of the
    /// port expander
    ///
    /// This method is only available if the `i2c` feature is enabled.
    ///
    /// # Examples
    ///
    /// ```
    /// let peripherals = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(peripherals);
    /// let delay = arduino_hal::Delay::new();
    ///
    /// let sda = pins.a4.into_pull_up_input();
    /// let scl = pins.a5.into_pull_up_input();
    ///
    /// let i2c_bus = arduino_hal::i2c::I2c::new(peripherals.TWI, sda, scl, 50000);
    /// let mut i2c_expander = Pcf8574::new(i2c_bus, true, true, true);
    ///
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new_pcf8574a(&mut i2c_expander, delay)
    ///     .with_blink(Blink::On)
    ///     .with_cursor(Cursor::Off)
    ///     .build();
    /// ```
    ///
    /// [Pcf8574a docs]: https://docs.rs/port-expander/latest/port_expander/dev/pcf8574/struct.Pcf8574a.html
    #[inline]
    pub fn new_pcf8574(expander: &'a mut Pcf8574<M>, delay: D) -> Self {
        Self::from_parts(expander.split(), delay)
    }
}
