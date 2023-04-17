use crate::LcdDisplay;
use core::convert::Infallible;
use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};
use port_expander::{dev::pcf8574, I2cBus, Pcf8574a};
use shared_bus::{BusMutex, NullMutex};

/// Uses port expander, like PCF8574, to communicate with display
pub struct I2CLcdDisplay<T, D, E>
where
    T: OutputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
{
    lcd: LcdDisplay<T, D>,
    expander: E,
}

impl<T, D, M, I2C> I2CLcdDisplay<T, D, Pcf8574a<M>>
where
    T: OutputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
    M: BusMutex<Bus = pcf8574::Driver<I2C>>,
    I2C: I2cBus,
{
    pub fn new_pcf8574a_with_mutex(i2c: I2C, a0: bool, a1: bool, a2: bool, delay: D) -> Self {
        let mut expander = Pcf8574a::with_mutex(i2c, a0, a1, a2);
        let pcf8574::Parts {
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
        } = expander.split();
        let lcd = LcdDisplay::new(p0, p2, delay)
            .with_half_bus(p4, p5, p6, p7)
            .build();
        Self { lcd, expander }
    }
}

impl<T, D, I2C> I2CLcdDisplay<T, D, Pcf8574a<NullMutex<pcf8574::Driver<I2C>>>>
where
    T: OutputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
    I2C: I2cBus,
{
    pub fn new_pcf8574a(i2c: I2C, a0: bool, a1: bool, a2: bool, delay: D) -> Self {
        let mut expander = Pcf8574a::new(i2c, a0, a1, a2);
        let pcf8574::Parts {
            p0,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
        } = expander.split();
        let lcd = LcdDisplay::new(p0, p2, delay)
            .with_half_bus(p4, p5, p6, p7)
            .build();
        Self { lcd, expander }
    }
}
