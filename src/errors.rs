/// Error type for [LcdDisplay][crate::display::LcdDisplay], returned by [LcdDisplay::error][crate::display::LcdDisplay::error]
///
/// LcdDisplay uses an internal error code rather than the standard rust
/// Result pattern because there are only two places in LcdDisplay where
/// an error is returned. Unfortunately, *every* public function invokes
/// one of those places (which has an [Infallible][core::convert::Infallible] error type, no less) and
/// would be forced to return a result or call unwrap/expect.
///
/// This led to a cluttered API in which users had to handle error conditions
/// when calling functions like [clear][crate::display::LcdDisplay::clear] and [home][crate::display::LcdDisplay::home].
/// An internal error code which could mostly be ignored except when debugging seemed like a better option.
#[repr(u8)]
#[derive(Clone, Eq, PartialEq)]
pub enum Error {
    /// No pin RS
    NoPinRS = 0,
    /// No pin EN
    NoPinEN = 1,
    /// No pin RW
    NoPinRW = 2,
    /// No pin D0
    NoPinD0 = 3,
    /// No pin D1
    NoPinD1 = 4,
    /// No pin D2
    NoPinD2 = 5,
    /// No pin D3
    NoPinD3 = 6,
    /// No pin D4
    NoPinD4 = 7,
    /// No pin D5
    NoPinD5 = 8,
    /// No pin D6
    NoPinD6 = 9,
    /// No pin D7
    NoPinD7 = 10,
    /// No error
    None = 11,
    /// [Bus mode][crate::display::Mode] is invalid or not set
    InvalidMode = 12,
    /// Invalid conversion from u8 to Error
    InvalidCode = 13,
}

impl From<u8> for Error {
    fn from(v: u8) -> Self {
        match v {
            0 => Error::NoPinRS,
            1 => Error::NoPinEN,
            2 => Error::NoPinRW,
            3 => Error::NoPinD0,
            4 => Error::NoPinD1,
            5 => Error::NoPinD2,
            6 => Error::NoPinD3,
            7 => Error::NoPinD4,
            8 => Error::NoPinD5,
            9 => Error::NoPinD6,
            10 => Error::NoPinD7,
            11 => Error::None,
            12 => Error::InvalidMode,
            _ => Error::InvalidCode,
        }
    }
}
