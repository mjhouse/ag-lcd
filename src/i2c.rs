use crate::LcdDisplay;
use core::{convert::Infallible, fmt::Debug};
use embedded_hal::{blocking::delay::DelayUs, digital::v2::OutputPin};
use port_expander::{dev::pcf8574, mode::QuasiBidirectional, I2cBus, Pcf8574, Pcf8574a, Pin};
use shared_bus::BusMutex;

/// Custom version of OldOutputPin that implements v2::OutputPin
/// Used to convert pin with fallible error to infallible
pub struct InfallibleOutputPin<T> {
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
        let _ = self.pin.set_low();
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let _ = self.pin.set_high();
        Ok(())
    }
}

impl<'a, D, M, I2C> LcdDisplay<InfallibleOutputPin<Pin<'a, QuasiBidirectional, M>>, D>
where
    D: DelayUs<u16> + Sized,
    M: BusMutex<Bus = pcf8574::Driver<I2C>>,
    I2C: I2cBus,
    <I2C as I2cBus>::BusError: Debug,
{
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
        LcdDisplay::new(
            InfallibleOutputPin::new(p0),
            InfallibleOutputPin::new(p2),
            delay,
            InfallibleOutputPin::new(p3),
        )
        .with_rw(InfallibleOutputPin::new(p1))
        .with_half_bus(
            InfallibleOutputPin::new(p4),
            InfallibleOutputPin::new(p5),
            InfallibleOutputPin::new(p6),
            InfallibleOutputPin::new(p7),
        )
    }

    /// Creates a new [`LcdDisplay`] using PCF8572A for interfacing
    #[inline]
    pub fn new_pcf8574a(expander: &'a mut Pcf8574a<M>, delay: D) -> Self {
        Self::from_parts(expander.split(), delay)
    }

    /// Creates a new [`LcdDisplay`] using PCF8572 for interfacing
    #[inline]
    pub fn new_pcf8574(expander: &'a mut Pcf8574<M>, delay: D) -> Self {
        Self::from_parts(expander.split(), delay)
    }
}
