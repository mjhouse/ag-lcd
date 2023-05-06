use crate::Error;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::digital::v2::OutputPin;

#[repr(u8)]
#[allow(dead_code)]
enum Command {
    ClearDisplay = 0x01,   // LCD_CLEARDISPLAY
    ReturnHome = 0x02,     // LCD_RETURNHOME
    SetDisplayMode = 0x04, // LCD_ENTRYMODESET
    SetDisplayCtrl = 0x08, // LCD_DISPLAYCONTROL
    CursorShift = 0x10,    // LCD_CURSORSHIFT
    SetDisplayFunc = 0x20, // LCD_FUNCTIONSET
    SetCGramAddr = 0x40,   // LCD_SETCGRAMADDR
    SetDDRAMAddr = 0x80,   // LCD_SETDDRAMADDR
}

#[repr(u8)]
#[allow(dead_code)]
enum Move {
    Display = 0x08, // LCD_DISPLAYMOVE
    Cursor = 0x00,  // LCD_CURSORMOVE
}

/// Flag that controls text direction
#[repr(u8)]
pub enum Layout {
    /// Text runs from right to left
    RightToLeft = 0x00, // LCD_ENTRYRIGHT

    /// Text runs from left to right (default)
    LeftToRight = 0x02, // LCD_ENTRYLEFT
}

/// Flag that sets the display to autoscroll
#[repr(u8)]
pub enum AutoScroll {
    /// Turn AutoScroll on
    On = 0x01, // LCD_ENTRYSHIFTINCREMENT

    /// Turn AutoScroll off (default)
    Off = 0x00, // LCD_ENTRYSHIFTDECREMENT
}

/// Flag that sets the display on/off
#[repr(u8)]
pub enum Display {
    /// Turn Display on (default)
    On = 0x04, // LCD_DISPLAYON

    /// Turn Display off
    Off = 0x00, // LCD_DISPLAYOFF
}

/// Flag that sets the cursor on/off
#[repr(u8)]
pub enum Cursor {
    /// Turn Cursor on
    On = 0x02, // LCD_CURSORON

    /// Turn Cursor off
    Off = 0x00, // LCD_CURSOROFF
}

/// Flag that sets cursor background to blink
#[repr(u8)]
pub enum Blink {
    /// Turn Blink on
    On = 0x01, // LCD_BLINKON

    /// Turn Blink off (default)
    Off = 0x00, // LCD_BLINKOFF
}

/// Flag that sets backlight state
pub enum Backlight {
    /// Turn Backlight on (default)
    On,

    /// Turn Backlight off
    Off,
}

/// Flag used to indicate direction for display scrolling
#[repr(u8)]
pub enum Scroll {
    /// Scroll display right
    Right = 0x04, // LCD_MOVERIGHT

    /// Scroll display left
    Left = 0x00, // LCD_MOVELEFT
}

/// Flag for the bus mode of the display
#[repr(u8)]
pub enum Mode {
    /// Use eight-bit bus (Set by [with_full_bus][LcdDisplay::with_full_bus])
    EightBits = 0x10, // LCD_8BITMODE

    /// Use four-bit bus (Set by [with_half_bus][LcdDisplay::with_half_bus])
    FourBits = 0x00, // LCD_4BITMODE
}

/// Flag for the number of lines in the display
#[repr(u8)]
pub enum Lines {
    /// Use four lines if available
    ///
    /// ## Notes
    /// Since HD44780 doesn't support 4-line LCDs, 4-line display is used like a 2-line display,
    /// but half of the characters were moved below the top part. Since the interface only allows
    /// two states for amount of lines: two and one, a way to differentiate between four line and
    /// two line mode is needed. According to HHD44780 documentation, when two-line display mode is
    /// used, the bit that specifies font size is ignored. Because of that, we can use it to
    /// differentiate between four line mode and two line mode.
    FourLines = 0x0C,

    /// Use two lines if available
    TwoLines = 0x08, // LCD_2LINE

    /// Use one line (default)
    OneLine = 0x00, // LCD_1LINE
}

/// Flag for the character size of the display
#[repr(u8)]
pub enum Size {
    /// Use display with 5x10 characters
    Dots5x10 = 0x04, // LCD_5x10DOTS

    /// Use display with 5x8 characters (default)
    Dots5x8 = 0x00, // LCD_5x8DOTS
}

/// One of the most popular sizes for this kind of LCD is 16x2
const DEFAULT_COLS: u8 = 16;

const DEFAULT_DISPLAY_FUNC: u8 = Mode::FourBits as u8 | Lines::OneLine as u8 | Size::Dots5x8 as u8;
const DEFAULT_DISPLAY_CTRL: u8 = Display::On as u8 | Cursor::Off as u8 | Blink::Off as u8;
const DEFAULT_DISPLAY_MODE: u8 = Layout::LeftToRight as u8 | AutoScroll::Off as u8;

const CMD_DELAY: u16 = 3500;
const CHR_DELAY: u16 = 450;

const RS: u8 = 0;
const EN: u8 = 1;
const RW: u8 = 2;
const D0: u8 = 3;
const D1: u8 = 4;
const D2: u8 = 5;
const D3: u8 = 6;
const D4: u8 = 7;
const D5: u8 = 8;
const D6: u8 = 9;
const D7: u8 = 10;
const A: u8 = 11;

/// The LCD display
///
/// Methods called on this struct will fail silently if the system or screen is
/// misconfigured.
pub struct LcdDisplay<T, D>
where
    T: OutputPin + Sized,
    D: DelayUs<u16> + Sized,
{
    pins: [Option<T>; 12],
    display_func: u8,
    display_mode: u8,
    display_ctrl: u8,
    offsets: [u8; 4],
    delay: D,
    code: Error,
}

impl<T, D> LcdDisplay<T, D>
where
    T: OutputPin + Sized,
    D: DelayUs<u16> + Sized,
{
    /// Create a new instance of the LcdDisplay
    ///
    /// # Examples
    ///
    /// ```
    /// let peripherals = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(peripherals);
    /// let delay = arduino_hal::Delay::new();
    ///
    /// let rs = pins.d12.into_output().downgrade();
    /// let rw = pins.d11.into_output().downgrade();
    /// let en = pins.d10.into_output().downgrade();
    /// let d4 = pins.d5.into_output().downgrade();
    /// let d5 = pins.d4.into_output().downgrade();
    /// let d6 = pins.d3.into_output().downgrade();
    /// let d7 = pins.d2.into_output().downgrade();
    ///
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_blink(Blink::On)
    ///     .with_cursor(Cursor::Off)
    ///     .with_rw(d10) // optional (set lcd pin to GND if not provided)
    ///     .build();
    /// ```
    pub fn new(rs: T, en: T, delay: D) -> Self {
        Self {
            pins: [
                Some(rs),
                Some(en),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            display_func: DEFAULT_DISPLAY_FUNC,
            display_mode: DEFAULT_DISPLAY_MODE,
            display_ctrl: DEFAULT_DISPLAY_CTRL,
            offsets: [0x00, 0x40, 0x00 + DEFAULT_COLS, 0x40 + DEFAULT_COLS],
            delay: delay,
            code: Error::None,
        }
    }

    /// Set amount of columns this lcd has
    pub fn with_cols(mut self, mut cols: u8) -> Self {
        cols = cols.clamp(0, 31);
        // First two bytes skipped because they are always the same
        self.offsets[2] = 0x00 + cols;
        self.offsets[3] = 0x40 + cols;
        self
    }

    /// Set four pins that connect to the lcd screen and configure the display for four-pin mode.
    ///
    /// The parameters below (d4-d7) are labeled in the order that you should see on the LCD
    /// itself. Regardless of how the display is connected to the arduino, 'D4' on the LCD should
    /// map to 'd4' when calling this function.
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .build();
    /// ```
    pub fn with_half_bus(mut self, d4: T, d5: T, d6: T, d7: T) -> Self {
        // set to four-bit bus mode and assign pins
        self.display_func &= !(Mode::EightBits as u8);
        self.pins[D4 as usize] = Some(d4);
        self.pins[D5 as usize] = Some(d5);
        self.pins[D6 as usize] = Some(d6);
        self.pins[D7 as usize] = Some(d7);
        self
    }

    /// Set eight pins that connect to the lcd screen and configure the display for eight-pin mode.
    ///
    /// The parameters below (d0-d7) are labeled in the order that you should see on the LCD
    /// itself. Regardless of how the display is connected to the arduino, 'D4' on the LCD should
    /// map to 'd4' when calling this function.
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_full_bus(d0, d1, d4, d5, d6, d7, d6, d7)
    ///     .build();
    /// ```
    pub fn with_full_bus(mut self, d0: T, d1: T, d2: T, d3: T, d4: T, d5: T, d6: T, d7: T) -> Self {
        // set to eight-bit bus mode and assign pins
        self.display_func |= Mode::EightBits as u8;
        self.pins[D0 as usize] = Some(d0);
        self.pins[D1 as usize] = Some(d1);
        self.pins[D2 as usize] = Some(d2);
        self.pins[D3 as usize] = Some(d3);
        self.pins[D4 as usize] = Some(d4);
        self.pins[D5 as usize] = Some(d5);
        self.pins[D6 as usize] = Some(d6);
        self.pins[D7 as usize] = Some(d7);
        self
    }

    /// Set an RW (Read/Write) pin to use (This is optional and can normally be connected directly
    /// to GND, leaving the display permanently in Write mode)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_rw(d10)
    ///     .build();
    /// ```
    pub fn with_rw(mut self, rw: T) -> Self {
        self.pins[RW as usize] = Some(rw);
        self
    }

    /// Set the character size of the LCD display. (Defaults to Size::Dots5x8)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_size(Size::Dots5x8)
    ///     .build();
    /// ```
    pub fn with_size(mut self, value: Size) -> Self {
        match value {
            Size::Dots5x10 => self.display_func |= Size::Dots5x10 as u8,
            Size::Dots5x8 => self.display_func &= !(Size::Dots5x10 as u8),
        }
        self
    }

    /// Set the number of lines on the LCD display. (Default is Lines::OneLine)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_lines(Lines::OneLine)
    ///     .build();
    /// ```
    pub fn with_lines(mut self, value: Lines) -> Self {
        match value {
            Lines::FourLines => self.display_func |= Lines::FourLines as u8,
            Lines::TwoLines => self.display_func |= Lines::TwoLines as u8,
            Lines::OneLine => self.display_func &= !(Lines::TwoLines as u8),
        }
        self
    }

    /// Set the text direction layout of the LCD display. (Default is Layout::LeftToRight)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_layout(Layout::LeftToRight)
    ///     .build();
    /// ```
    pub fn with_layout(mut self, value: Layout) -> Self {
        match value {
            Layout::LeftToRight => self.display_mode |= Layout::LeftToRight as u8,
            Layout::RightToLeft => self.display_mode &= !(Layout::LeftToRight as u8),
        }
        self
    }

    /// Set the LCD display on or off initially. (Default is Display::On)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_display(Display::On)
    ///     .build();
    /// ```
    pub fn with_display(mut self, value: Display) -> Self {
        match value {
            Display::On => self.display_ctrl |= Display::On as u8,
            Display::Off => self.display_ctrl &= !(Display::On as u8),
        }
        self
    }

    /// Set the cursor on or off initially. (Default is Cursor::Off)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_cursor(Cursor::Off)
    ///     .build();
    /// ```
    pub fn with_cursor(mut self, value: Cursor) -> Self {
        match value {
            Cursor::On => self.display_ctrl |= Cursor::On as u8,
            Cursor::Off => self.display_ctrl &= !(Cursor::On as u8),
        }
        self
    }

    /// Set the cursor background to blink on and off. (Default is Blink::Off)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_blink(Blink::Off)
    ///     .build();
    /// ```
    pub fn with_blink(mut self, value: Blink) -> Self {
        match value {
            Blink::On => self.display_ctrl |= Blink::On as u8,
            Blink::Off => self.display_ctrl &= !(Blink::On as u8),
        }
        self
    }

    /// Set a pin for controlling backlight state
    pub fn with_backlight(mut self, backlight_pin: T) -> Self {
        self.pins[A as usize] = Some(backlight_pin);
        self
    }

    /// Set autoscroll on or off. (Default is AutoScroll::Off)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_autoscroll(AutoScroll::Off)
    ///     .build();
    /// ```
    pub fn with_autoscroll(mut self, value: AutoScroll) -> Self {
        match value {
            AutoScroll::On => self.display_mode |= AutoScroll::On as u8,
            AutoScroll::Off => self.display_mode &= !(AutoScroll::On as u8),
        }
        self
    }

    /// Increase reliability of initialization of LCD.
    ///
    /// Some users experience unreliable initialization of the LCD, where
    /// the LCD sometimes is unable to display symbols after running
    /// `.build()`. This method toggles the LCD off and on with some
    /// delay in between, 3 times. A higher `delay_toggle` tends to make
    /// this method more reliable, and a value of `10 000` is recommended.
    /// Note that this method should be run as close as possible to
    /// `.build()`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_reliable_init(10000)
    ///     .build();
    /// ```
    pub fn with_reliable_init(mut self, delay_toggle: u16) -> Self {
        if self.display_ctrl == Display::On as u8 {
            for _ in 0..3 {
                self.delay.delay_us(delay_toggle);
                self.display_off();
                self.delay.delay_us(delay_toggle);
                self.display_on();
            }
        } else {
            for _ in 0..3 {
                self.delay.delay_us(delay_toggle);
                self.display_on();
                self.delay.delay_us(delay_toggle);
                self.display_off();
            }
        }

        self
    }

    /// Finish construction of the LcdDisplay and initialized the
    /// display to the provided settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use ag_lcd::{Display, Blink, Cursor, LcdDisplay};
    ///
    /// let peripherals = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(peripherals);
    /// let delay = arduino_hal::Delay::new();
    ///
    /// let rs = pins.d12.into_output().downgrade();
    /// let rw = pins.d11.into_output().downgrade();
    /// let en = pins.d10.into_output().downgrade();
    ///
    /// // left-side names refer to lcd pinout (e.g. 'd4' = D4 on lcd)
    /// let d4 = pins.d5.into_output().downgrade();
    /// let d5 = pins.d4.into_output().downgrade();
    /// let d6 = pins.d3.into_output().downgrade();
    /// let d7 = pins.d2.into_output().downgrade();
    ///
    /// let mut lcd: LcdDisplay<_,_> = LcdDisplay::new(rs, en, delay)
    ///     .with_half_bus(d4, d5, d6, d7)
    ///     .with_display(Display::On)
    ///     .with_blink(Blink::On)
    ///     .with_cursor(Cursor::On)
    ///     .with_rw(rw) // optional (set lcd pin to GND if not provided)
    ///     .build();
    ///
    /// lcd.print("Test message!");
    /// ```
    pub fn build(mut self) -> Self {
        self.delay.delay_us(50000);

        self.set(RS, false);
        self.set(EN, false);

        if self.exists(RW) {
            self.set(RW, false);
        }

        match self.mode() {
            Mode::FourBits => {
                // display function is four bit
                self.update(0x03);
                self.delay.delay_us(4500);

                self.update(0x03);
                self.delay.delay_us(4500);

                self.update(0x03);
                self.delay.delay_us(150);

                self.update(0x02);
            }
            Mode::EightBits => {
                // display function is eight bit
                self.command(Command::SetDisplayFunc as u8 | self.display_func);
                self.delay.delay_us(4500);

                self.command(Command::SetDisplayFunc as u8 | self.display_func);
                self.delay.delay_us(150);

                self.command(Command::SetDisplayFunc as u8 | self.display_func);
            }
        }

        self.command(Command::SetDisplayFunc as u8 | self.display_func);
        self.delay.delay_us(CMD_DELAY);

        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        self.delay.delay_us(CMD_DELAY);

        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        self.delay.delay_us(CMD_DELAY);

        self.clear();
        self.home();

        // set an error code display is misconfigured
        self.validate();
        self
    }

    /// Set the position of the cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// let row = 0;
    /// let col = 2;
    ///
    /// lcd.set_position(col,row);
    /// ```
    pub fn set_position(&mut self, col: u8, mut row: u8) {
        let max_lines = 4;

        let num_lines = match self.lines() {
            Lines::FourLines => 4,
            Lines::TwoLines => 2,
            Lines::OneLine => 1,
        };

        let mut pos = col;

        if row >= max_lines {
            row = max_lines.saturating_sub(1);
        }

        if row >= num_lines {
            row = num_lines.saturating_sub(1);
        }

        pos += self.offsets[row as usize];
        self.command(Command::SetDDRAMAddr as u8 | pos);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Scroll the display right or left.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// let direction = Scroll::Left;
    /// let distance = 2;
    ///
    /// lcd.set_scroll(direction,distance);
    /// ```
    pub fn set_scroll(&mut self, direction: Scroll, distance: u8) {
        let command = Command::CursorShift as u8 | Move::Display as u8 | direction as u8;
        for _ in 0..distance {
            self.command(command);
            self.delay.delay_us(CMD_DELAY);
        }
    }

    /// Set the text direction layout.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// lcd.set_layout(Layout::LeftToRight);
    /// ```
    pub fn set_layout(&mut self, layout: Layout) {
        match layout {
            Layout::LeftToRight => self.display_mode |= Layout::LeftToRight as u8,
            Layout::RightToLeft => self.display_mode &= !(Layout::LeftToRight as u8),
        }
        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Turn the display on or off.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// lcd.set_display(Display::Off);
    /// ```
    pub fn set_display(&mut self, display: Display) {
        match display {
            Display::On => self.display_ctrl |= Display::On as u8,
            Display::Off => self.display_ctrl &= !(Display::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Turn the cursor on or off.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// lcd.set_cursor(Cursor::On);
    /// ```
    pub fn set_cursor(&mut self, cursor: Cursor) {
        match cursor {
            Cursor::On => self.display_ctrl |= Cursor::On as u8,
            Cursor::Off => self.display_ctrl &= !(Cursor::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Make the background of the cursor blink or stop blinking.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// lcd.set_blink(Blink::On);
    /// ```
    pub fn set_blink(&mut self, blink: Blink) {
        match blink {
            Blink::On => self.display_ctrl |= Blink::On as u8,
            Blink::Off => self.display_ctrl &= !(Blink::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Enable or disable LCD backlight
    pub fn set_backlight(&mut self, backlight: Backlight) {
        match backlight {
            Backlight::On => self.backlight_on(),
            Backlight::Off => self.backlight_off(),
        }
    }

    /// Turn auto scroll on or off.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// lcd.set_autoscroll(AutoScroll::On);
    /// ```
    pub fn set_autoscroll(&mut self, scroll: AutoScroll) {
        match scroll {
            AutoScroll::On => self.display_mode |= AutoScroll::On as u8,
            AutoScroll::Off => self.display_mode &= !(AutoScroll::On as u8),
        }
        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Add a new character map to the LCD memory (CGRAM) at a particular location.
    /// There are eight locations available at positions 0-7, and location values
    /// outside of this range will be bitwise masked to fall within the range, possibly
    /// overwriting an existing custom character.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    ///
    /// // set a sideways smiley face in CGRAM at location 0.
    /// lcd.set_character(0u8,[
    ///     0b00110,
    ///     0b00001,
    ///     0b11001,
    ///     0b00001,
    ///     0b00001,
    ///     0b11001,
    ///     0b00001,
    ///     0b00110
    /// ]);
    ///
    /// // write the character code for the custom character.
    /// lcd.home();
    /// lcd.write(0u8);
    /// ```
    pub fn set_character(&mut self, mut location: u8, map: [u8; 8]) {
        location &= 0x7; // limit to locations 0-7
        self.command(Command::SetCGramAddr as u8 | (location << 3));
        for ch in map.iter() {
            self.write(*ch);
        }
    }

    /// Clear the display.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.clear();
    /// ```
    pub fn clear(&mut self) {
        self.command(Command::ClearDisplay as u8);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Move the cursor to the home position.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.home(); // cursor should be top-left
    /// ```
    pub fn home(&mut self) {
        self.command(Command::ReturnHome as u8);
        self.delay.delay_us(CMD_DELAY);
    }

    /// Scroll the display to the right. (See [set_scroll][LcdDisplay::set_scroll])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.scroll_right(2); // display scrolls 2 positions to the right.
    /// ```
    pub fn scroll_right(&mut self, value: u8) {
        self.set_scroll(Scroll::Right, value);
    }

    /// Scroll the display to the left. (See [set_scroll][LcdDisplay::set_scroll])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.scroll_left(2); // display scrolls 2 positions to the left.
    /// ```
    pub fn scroll_left(&mut self, value: u8) {
        self.set_scroll(Scroll::Left, value);
    }

    /// Set the text direction layout left-to-right. (See [set_layout][LcdDisplay::set_layout])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.layout_left_to_right();
    /// ```
    pub fn layout_left_to_right(&mut self) {
        self.set_layout(Layout::LeftToRight);
    }

    /// Set the text direction layout right-to-left. (See [set_layout][LcdDisplay::set_layout])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.layout_right_to_left();
    /// ```
    pub fn layout_right_to_left(&mut self) {
        self.set_layout(Layout::RightToLeft);
    }

    /// Turn the display on. (See [set_display][LcdDisplay::set_display])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.display_on();
    /// ```
    pub fn display_on(&mut self) {
        self.set_display(Display::On);
    }

    /// Turn the display off. (See [set_display][LcdDisplay::set_display])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.display_off();
    /// ```
    pub fn display_off(&mut self) {
        self.set_display(Display::Off);
    }

    /// Turn the cursor on. (See [set_cursor][LcdDisplay::set_cursor])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.cursor_on();
    /// ```
    pub fn cursor_on(&mut self) {
        self.set_cursor(Cursor::On);
    }

    /// Turn the cursor off. (See [set_cursor][LcdDisplay::set_cursor])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.cursor_off();
    /// ```
    pub fn cursor_off(&mut self) {
        self.set_cursor(Cursor::Off);
    }

    /// Set the background of the cursor to blink. (See [set_blink][LcdDisplay::set_blink])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.blink_on();
    /// ```
    pub fn blink_on(&mut self) {
        self.set_blink(Blink::On);
    }

    /// Set the background of the cursor to stop blinking. (See [set_blink][LcdDisplay::set_blink])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.blink_off();
    /// ```
    pub fn blink_off(&mut self) {
        self.set_blink(Blink::Off);
    }

    /// Turn backlight on
    pub fn backlight_on(&mut self) {
        if let Some(backlight_pin) = &mut self.pins[A as usize] {
            let _ = backlight_pin.set_high();
        }
    }

    /// Turn backlight off
    pub fn backlight_off(&mut self) {
        if let Some(backlight_pin) = &mut self.pins[A as usize] {
            let _ = backlight_pin.set_low();
        }
    }

    /// Turn autoscroll on. (See [set_autoscroll][LcdDisplay::set_autoscroll])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.autoscroll_on();
    /// ```
    pub fn autoscroll_on(&mut self) {
        self.set_autoscroll(AutoScroll::On);
    }

    /// Turn autoscroll off. (See [set_autoscroll][LcdDisplay::set_autoscroll])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.autoscroll_off();
    /// ```
    pub fn autoscroll_off(&mut self) {
        self.set_autoscroll(AutoScroll::Off);
    }

    /// Get the current bus mode. (See [with_half_bus][LcdDisplay::with_half_bus] and [with_full_bus][LcdDisplay::with_full_bus])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let mode = lcd.mode();
    /// ```
    pub fn mode(&self) -> Mode {
        if (self.display_func & Mode::EightBits as u8) == 0 {
            Mode::FourBits
        } else {
            Mode::EightBits
        }
    }

    /// Get the current text direction layout. (See [set_layout][LcdDisplay::set_layout])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let layout = lcd.layout();
    /// ```
    pub fn layout(&self) -> Layout {
        if (self.display_mode & Layout::LeftToRight as u8) == 0 {
            Layout::RightToLeft
        } else {
            Layout::LeftToRight
        }
    }

    /// Get the current state of the display (on or off). (See [set_display][LcdDisplay::set_display])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let display = lcd.display();
    /// ```
    pub fn display(&self) -> Display {
        if (self.display_ctrl & Display::On as u8) == 0 {
            Display::Off
        } else {
            Display::On
        }
    }

    /// Get the current cursor state (on or off). (See [set_cursor][LcdDisplay::set_cursor])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let cursor = lcd.cursor();
    /// ```
    pub fn cursor(&self) -> Cursor {
        if (self.display_ctrl & Cursor::On as u8) == 0 {
            Cursor::Off
        } else {
            Cursor::On
        }
    }

    /// Get the current blink state (on or off). (See [set_blink][LcdDisplay::set_blink])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let blink = lcd.blink();
    /// ```
    pub fn blink(&self) -> Blink {
        if (self.display_ctrl & Blink::On as u8) == 0 {
            Blink::Off
        } else {
            Blink::On
        }
    }

    /// Get the current autoscroll state (on or off). (See [set_autoscroll][LcdDisplay::set_autoscroll])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let autoscroll = lcd.autoscroll();
    /// ```
    pub fn autoscroll(&self) -> AutoScroll {
        if (self.display_mode & AutoScroll::On as u8) == 0 {
            AutoScroll::Off
        } else {
            AutoScroll::On
        }
    }

    /// Get the number of lines. (See [with_lines][LcdDisplay::with_lines])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let lines = lcd.lines();
    /// ```
    pub fn lines(&self) -> Lines {
        let flag_bits: u8 = self.display_func & 0x0C;
        if flag_bits == Lines::FourLines as u8 {
            Lines::FourLines
        } else if flag_bits == Lines::TwoLines as u8 {
            Lines::TwoLines
        } else {
            Lines::OneLine
        }
    }

    /// Get the current error code. If an error occurs, the internal code will be
    /// set to a value other than [Error::None][Error::None] (11u8).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// let code = lcd.error();
    /// ```
    pub fn error(&self) -> Error {
        self.code.clone()
    }

    /// Print a message to the LCD display.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.print("TEST MESSAGE");
    /// ```
    pub fn print(&mut self, text: &str) {
        for ch in text.chars() {
            self.write(ch as u8);
        }
    }

    /// Write a single character to the LCD display.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay<_,_> = ...;
    /// lcd.write('A' as u8);
    /// ```
    pub fn write(&mut self, value: u8) {
        self.delay.delay_us(CHR_DELAY);
        self.send(value, true);
    }

    /// Execute a command on the LCD display, usually by using bitwise OR to combine
    /// flags in various ways.
    ///
    /// # Examples
    ///
    /// ```
    /// self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
    /// ```
    fn command(&mut self, value: u8) {
        self.send(value, false);
    }

    /// Send bytes to the LCD display with the RS pin set either high (for commands)
    /// or low (to write to memory)
    ///
    /// # Examples
    ///
    /// ```
    /// self.send(value, true);
    /// ```
    fn send(&mut self, byte: u8, mode: bool) {
        self.set(RS, mode);

        if self.exists(RW) {
            self.set(RW, false);
        }

        match self.mode() {
            Mode::FourBits => {
                self.update(byte >> 4);
                self.update(byte);
            }
            Mode::EightBits => {
                self.update(byte);
            }
        }
    }

    /// Update the on-device memory by sending either the bottom nibble (in
    /// four-bit mode) or a whole byte (in eight-bit) and then pulsing the enable pin.
    ///
    /// # Examples
    ///
    /// ```
    /// self.update(byte);
    /// ```
    fn update(&mut self, byte: u8) {
        self.set(EN, false);
        match self.mode() {
            Mode::FourBits => {
                self.set(D7, (byte >> 3) & 1 > 0);
                self.set(D6, (byte >> 2) & 1 > 0);
                self.set(D5, (byte >> 1) & 1 > 0);
                self.set(D4, (byte >> 0) & 1 > 0);
            }
            Mode::EightBits => {
                self.set(D7, (byte >> 7) & 1 > 0);
                self.set(D6, (byte >> 6) & 1 > 0);
                self.set(D5, (byte >> 5) & 1 > 0);
                self.set(D4, (byte >> 4) & 1 > 0);
                self.set(D3, (byte >> 3) & 1 > 0);
                self.set(D2, (byte >> 2) & 1 > 0);
                self.set(D1, (byte >> 1) & 1 > 0);
                self.set(D0, (byte >> 0) & 1 > 0);
            }
        };
        self.pulse();
    }

    /// Set the enable pin high and then low to make the LCD accept the most
    /// recently transmitted data.
    ///
    /// # Examples
    ///
    /// ```
    /// self.pulse();
    /// ```
    fn pulse(&mut self) {
        self.set(EN, true);
        self.set(EN, false);
    }

    /// Set a pin at position `index` to a particular value
    ///
    /// # Examples
    ///
    /// ```
    /// self.set(RS, true);
    /// ```
    fn set(&mut self, index: u8, value: bool) {
        if self.pins[index as usize]
            .as_mut()
            .map(|p| match value {
                true => p.set_high().ok(),
                false => p.set_low().ok(),
            })
            .flatten()
            .is_none()
        {
            self.code = index.into();
        }
    }

    /// Check that a pin exists
    ///
    /// # Examples
    ///
    /// ```
    /// if self.exists(RS) {
    ///     ...
    /// }
    /// ```
    fn exists(&self, index: u8) -> bool {
        self.pins[index as usize].is_some()
    }

    /// Set an error code if display is misconfigured. Currently
    /// only validates the number of pins for the given bus width.
    fn validate(&mut self) {
        if match self.mode() {
            Mode::FourBits => {
                self.exists(D4) || self.exists(D5) || self.exists(D6) || self.exists(D7)
            }
            Mode::EightBits => {
                self.exists(D0)
                    || self.exists(D1)
                    || self.exists(D2)
                    || self.exists(D3)
                    || self.exists(D4)
                    || self.exists(D5)
                    || self.exists(D6)
                    || self.exists(D7)
            }
        } {
            self.code = Error::InvalidMode;
        }
    }
}

/// Implementation of ufmt::uWrite
///
/// This trait allows us to use the uwrite/uwriteln macros from ufmt
/// to format arbitrary arguments (that have the appropriate uDisplay or uDebug traits
/// implemented) into a string to display on the lcd screen.
///
/// # Examples
///
/// ```
/// let mut lcd: LcdDisplay<_,_> = ...;
///
/// let count = 3;
/// uwriteln!(&mut lcd, "COUNT IS: {}",count);
/// ```
///
#[cfg(feature = "ufmt")]
impl<T, D> ufmt::uWrite for LcdDisplay<T, D>
where
    T: OutputPin<Error = Infallible> + Sized,
    D: DelayUs<u16> + Sized,
{
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.print(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        self.write(c as u8);
        Ok(())
    }
}
