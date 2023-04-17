use crate::LcdDisplay;
use core::{
    convert::Infallible,
    fmt::Debug,
    ops::{Deref, DerefMut},
};
use embedded_hal::{
    blocking::delay::DelayUs,
    digital::{v1_compat::OldOutputPin, v2::OutputPin},
};
use port_expander::{dev::pcf8574, mode::QuasiBidirectional, I2cBus, Pcf8574a, Pin};
use shared_bus::{BusMutex, NullMutex};

/// Custom version of OldOutputPin that implements v2::OutputPin
/// Used to convert pin with fallible error to infallible
struct InfallibleOutputPin<T> {
    pin: T,
}

impl<T, E> InfallibleOutputPin<T>
where
    T: OutputPin<Error = E>,
    E: Debug,
{
    fn new(pin: T) -> Self {
        Self { pin }
    }
}

impl<T, E> OutputPin for InfallibleOutputPin<T>
where
    T: OutputPin<Error = E>,
    E: Debug,
{
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self.pin.set_low().unwrap())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self.pin.set_high().unwrap())
    }
}

/// Uses port expander, like PCF8574, to communicate with display
pub struct I2CLcdDisplay<T, D, E>
where
    T: OutputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
{
    lcd: LcdDisplay<T, D>,
    expander: E,
}

impl<D, M, I2C> I2CLcdDisplay<InfallibleOutputPin<Pin<'_, QuasiBidirectional, M>>, D, Pcf8574a<M>>
where
    D: DelayUs<u16> + Sized,
    M: BusMutex<Bus = pcf8574::Driver<I2C>>,
    I2C: I2cBus,
    <I2C as I2cBus>::BusError: Debug,
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
        let lcd = LcdDisplay::new(
            InfallibleOutputPin::new(p0),
            InfallibleOutputPin::new(p2),
            delay,
        )
        .with_half_bus(
            InfallibleOutputPin::new(p4),
            InfallibleOutputPin::new(p5),
            InfallibleOutputPin::new(p6),
            InfallibleOutputPin::new(p7),
        )
        .build();
        Self { lcd, expander }
    }
}

impl<D, I2C>
    I2CLcdDisplay<
        InfallibleOutputPin<Pin<'_, QuasiBidirectional, NullMutex<pcf8574::Driver<I2C>>>>,
        D,
        Pcf8574a<NullMutex<pcf8574::Driver<I2C>>>,
    >
where
    D: DelayUs<u16> + Sized,
    I2C: I2cBus,
    <I2C as I2cBus>::BusError: Debug,
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
        let lcd = LcdDisplay::new(
            InfallibleOutputPin::new(p0),
            InfallibleOutputPin::new(p2),
            delay,
        )
        .with_half_bus(
            InfallibleOutputPin::new(p4),
            InfallibleOutputPin::new(p5),
            InfallibleOutputPin::new(p6),
            InfallibleOutputPin::new(p7),
        )
        .build();
        Self { lcd, expander }
    }
}

impl<T, D, E> Deref for I2CLcdDisplay<T, D, E>
where
    T: OutputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
{
    type Target = LcdDisplay<T, D>;

    fn deref(&self) -> &Self::Target {
        &self.lcd
    }
}

impl<T, D, E> DerefMut for I2CLcdDisplay<T, D, E>
where
    T: OutputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lcd
    }
}
