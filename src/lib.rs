#![no_std]
#![feature(external_doc)]

#![doc(include = "../README.md")]

use arduino_hal::hal::port::{
    mode::{Input, InputMode, Output},
    Pin, PB0, PB1, PB2, PB3, PB4, PD2, PD3, PD4, PD5, PD6, PD7,
};
use avr_hal_generic::port::PinOps;

macro_rules! delay {
    ( $t:expr ) => {
        arduino_hal::delay_us($t)
    };
}

macro_rules! set {
    ( $p:expr, $v:expr, $i:literal ) => {
        if let Some(pin) = $p.as_mut() {
            if ($v >> $i) & 0x01 > 0 {
                pin.set_high();
            } else {
                pin.set_low();
            }
        }
    };
    ( $p:expr, $v:expr ) => {
        set!($p, $v, 0);
    };
}

type InputPin<M, T> = Pin<Input<M>, T>;
type OutputPin<T> = Pin<Output, T>;

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
    RightToLeft = 0x00, // LCD_ENTRYRIGHT
    LeftToRight = 0x02, // LCD_ENTRYLEFT
}

/// Flag that sets the display to autoscroll
#[repr(u8)]
pub enum AutoScroll {
    On = 0x01,  // LCD_ENTRYSHIFTINCREMENT
    Off = 0x00, // LCD_ENTRYSHIFTDECREMENT
}

/// Flag that sets the display on/off
#[repr(u8)]
pub enum Display {
    On = 0x04,  // LCD_DISPLAYON
    Off = 0x00, // LCD_DISPLAYOFF
}

/// Flag that sets the cursor on/off
#[repr(u8)]
pub enum Cursor {
    On = 0x02,  // LCD_CURSORON
    Off = 0x00, // LCD_CURSOROFF
}

/// Flag that sets cursor background to blink
#[repr(u8)]
pub enum Blink {
    On = 0x01,  // LCD_BLINKON
    Off = 0x00, // LCD_BLINKOFF
}

/// Flag used to indicate direction for display scrolling
#[repr(u8)]
pub enum Scroll {
    Right = 0x04, // LCD_MOVERIGHT
    Left = 0x00,  // LCD_MOVELEFT
}

/// Flag for the bus mode of the display
#[repr(u8)]
pub enum Mode {
    EightBits = 0x10, // LCD_8BITMODE
    FourBits = 0x00,  // LCD_4BITMODE
}

/// Flag for the number of lines in the display 
#[repr(u8)]
pub enum Lines {
    TwoLines = 0x08, // LCD_2LINE
    OneLine = 0x00,  // LCD_1LINE
}

/// Flag for the character size of the display
#[repr(u8)]
pub enum Size {
    Dots5x10 = 0x04, // LCD_5x10DOTS
    Dots5x8 = 0x00,  // LCD_5x8DOTS
}

const DEFAULT_DISPLAY_FUNC: u8 = Mode::FourBits as u8 | Lines::OneLine as u8 | Size::Dots5x8 as u8;
const DEFAULT_DISPLAY_CTRL: u8 = Display::On as u8 | Cursor::Off as u8 | Blink::Off as u8;
const DEFAULT_DISPLAY_MODE: u8 = Layout::LeftToRight as u8 | AutoScroll::Off as u8;

const CMD_DELAY: u32 = 2900;
const CHR_DELAY: u32 = 320;

/// The LCD display
///
/// Methods called on this struct will fail silently if the system or screen is
/// misconfigured. Should never panic.
pub struct LcdDisplay<
    Rs = PB4, // d12
    En = PB3, // d11
    Rw = PB2, // d10
    D1 = PD2, // d2
    D2 = PD3, // d3
    D3 = PD4, // d4
    D4 = PD5, // d5
    D5 = PD6, // d6
    D6 = PD7, // d7
    D7 = PB0, // d8
    D8 = PB1, // d9
> {
    rs: Option<OutputPin<Rs>>,
    en: Option<OutputPin<En>>,
    rw: Option<OutputPin<Rw>>,
    d1: Option<OutputPin<D1>>,
    d2: Option<OutputPin<D2>>,
    d3: Option<OutputPin<D3>>,
    d4: Option<OutputPin<D4>>,
    d5: Option<OutputPin<D5>>,
    d6: Option<OutputPin<D6>>,
    d7: Option<OutputPin<D7>>,
    d8: Option<OutputPin<D8>>,
    display_func: u8,
    display_mode: u8,
    display_ctrl: u8,
    offsets: [u8; 4],
}

impl<Rs, En, Rw, D1, D2, D3, D4, D5, D6, D7, D8> LcdDisplay<Rs, En, Rw, D1, D2, D3, D4, D5, D6, D7, D8>
where
    Rs: PinOps,
    En: PinOps,
    Rw: PinOps,
    D1: PinOps,
    D2: PinOps,
    D3: PinOps,
    D4: PinOps,
    D5: PinOps,
    D6: PinOps,
    D7: PinOps,
    D8: PinOps,
{

    /// Create a new instance of the LcdDisplay
    ///
    /// # Examples
    ///
    /// ```
    /// let peripherals = arduino_hal::Peripherals::take().unwrap();
    /// let pins = arduino_hal::pins!(peripherals);
    ///
    /// let d12 = pins.d12;
    /// let d11 = pins.d11;
    ///
    /// let d2 = pins.d2;
    /// let d3 = pins.d3;
    /// let d4 = pins.d4;
    /// let d5 = pins.d5;
    ///
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
    ///     .with_blink(Blink::On)
    ///     .with_cursor(Cursor::Off)
    ///     .with_rw(d10)
    ///     .build();
    /// ```
    pub fn new<M>(rs: InputPin<M, Rs>, en: InputPin<M, En>) -> Self
    where
        M: InputMode,
    {
        Self {
            rs: Some(rs.into_output()),
            en: Some(en.into_output()),
            rw: None,
            d1: None,
            d2: None,
            d3: None,
            d4: None,
            d5: None,
            d6: None,
            d7: None,
            d8: None,
            display_func: DEFAULT_DISPLAY_FUNC,
            display_mode: DEFAULT_DISPLAY_MODE,
            display_ctrl: DEFAULT_DISPLAY_CTRL,
            offsets: [0, 0, 0, 0],
        }
    }

    /// Set four pins that connect to the lcd screen and configure the display for four-pin mode.
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
    ///     .build();
    /// ```
    pub fn with_half_bus<M>(
        mut self,
        d1: InputPin<M, D1>,
        d2: InputPin<M, D2>,
        d3: InputPin<M, D3>,
        d4: InputPin<M, D4>,
    ) -> Self
    where
        M: InputMode,
    {
        // set to four-bit bus mode and assign pins
        self.display_func &= !(Mode::EightBits as u8);
        self.d1 = Some(d1.into_output());
        self.d2 = Some(d2.into_output());
        self.d3 = Some(d3.into_output());
        self.d4 = Some(d4.into_output());
        self
    }

    /// Set eight pins that connect to the lcd screen and configure the display for eight-pin mode.
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_full_bus(d2, d3, d4, d5, d6, d7, d8, d9)
    ///     .build();
    /// ```
    pub fn with_full_bus<M>(
        mut self,
        d1: InputPin<M, D1>,
        d2: InputPin<M, D2>,
        d3: InputPin<M, D3>,
        d4: InputPin<M, D4>,
        d5: InputPin<M, D5>,
        d6: InputPin<M, D6>,
        d7: InputPin<M, D7>,
        d8: InputPin<M, D8>,
    ) -> Self
    where
        M: InputMode,
    {
        // set to eight-bit bus mode and assign pins
        self.display_func |= Mode::EightBits as u8;
        self.d1 = Some(d1.into_output());
        self.d2 = Some(d2.into_output());
        self.d3 = Some(d3.into_output());
        self.d4 = Some(d4.into_output());
        self.d5 = Some(d5.into_output());
        self.d6 = Some(d6.into_output());
        self.d7 = Some(d7.into_output());
        self.d8 = Some(d8.into_output());
        self
    }

    /// Set an RW (Read/Write) pin to use (This is optional and can normally be connected directly
    /// to GND, leaving the display permanently in Write mode)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
    ///     .with_rw(d10)
    ///     .build();
    /// ```
    pub fn with_rw<M>(mut self, rw: InputPin<M, Rw>) -> Self 
    where 
        M: InputMode,
    {
        self.rw = Some(rw.into_output());
        self
    }

    /// Set the character size of the LCD display. (Defaults to Size::Dots5x8)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
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
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
    ///     .with_lines(Lines::OneLine)
    ///     .build();
    /// ```
    pub fn with_lines(mut self, value: Lines) -> Self {
        match value {
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
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
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
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
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
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
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
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
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

    /// Set autoscroll on or off. (Default is AutoScroll::Off)
    ///
    /// # Examples
    ///
    /// ```
    /// ...
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     .with_half_bus(d2, d3, d4, d5)
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
    ///
    /// let d12 = pins.d12;
    /// let d11 = pins.d11;
    /// let d10 = pins.d10;
    ///
    /// let d2 = pins.d2;
    /// let d3 = pins.d3;
    /// let d4 = pins.d4;
    /// let d5 = pins.d5;
    ///
    /// // Capital 'D<N>' pins are on the LCD, lowercase are 
    /// // on the arduino. 
    ///
    /// // d12 = RS, d11 = Enable
    /// let mut lcd: LcdDisplay = LcdDisplay::new(d12, d11)
    ///     // d2 = D7, d3 = D6, d4 = D5, d5 = D4
    ///     .with_half_bus(d2, d3, d4, d5)
    ///     .with_display(Display::On)
    ///     .with_blink(Blink::On)
    ///     .with_cursor(Cursor::On)
    ///     .with_rw(d10)
    ///     .build();
    ///
    /// lcd.print("Test message!");
    /// ```
    pub fn build(mut self) -> Self {
        delay!(50000);
        set!(self.rs, 0);
        set!(self.en, 0);
        set!(self.rw, 0);

        let cols: u8 = 16;

        self.offsets[0] = 0x00;
        self.offsets[1] = 0x40;
        self.offsets[2] = 0x00 + cols;
        self.offsets[3] = 0x40 + cols;

        match self.mode() {
            Mode::FourBits => {
                // display function is four bit
                self.update(0x03);
                delay!(4500);

                self.update(0x03);
                delay!(4500);

                self.update(0x03);
                delay!(150);

                self.update(0x02);
            }
            Mode::EightBits => {
                // display function is eight bit
                self.command(Command::SetDisplayFunc as u8 | self.display_func);
                delay!(4500);

                self.command(Command::SetDisplayFunc as u8 | self.display_func);
                delay!(150);

                self.command(Command::SetDisplayFunc as u8 | self.display_func);
            }
        }

        self.command(Command::SetDisplayFunc as u8 | self.display_func);
        delay!(CMD_DELAY);

        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        delay!(CMD_DELAY);

        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        delay!(CMD_DELAY);

        self.clear();
        self.home();

        self
    }

    /// Set the position of the cursor.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
    ///
    /// let row = 0;
    /// let col = 2;
    ///
    /// lcd.set_position(col,row);
    /// ```
    pub fn set_position(&mut self, col: u8, mut row: u8) {
        let max_lines = 4;

        let num_lines = match self.lines() {
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
        delay!(CMD_DELAY);
    }

    /// Scroll the display right or left.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
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
            delay!(CMD_DELAY);
        }
    }

    /// Set the text direction layout.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
    ///
    /// lcd.set_layout(Layout::LeftToRight);
    /// ```
    pub fn set_layout(&mut self, layout: Layout) {
        match layout {
            Layout::LeftToRight => self.display_mode |= Layout::LeftToRight as u8,
            Layout::RightToLeft => self.display_mode &= !(Layout::LeftToRight as u8),
        }
        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        delay!(CMD_DELAY);
    }

    /// Turn the display on or off.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
    ///
    /// lcd.set_display(Display::Off);
    /// ```
    pub fn set_display(&mut self, display: Display) {
        match display {
            Display::On => self.display_ctrl |= Display::On as u8,
            Display::Off => self.display_ctrl &= !(Display::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        delay!(CMD_DELAY);
    }

    /// Turn the cursor on or off.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
    ///
    /// lcd.set_cursor(Cursor::On);
    /// ```
    pub fn set_cursor(&mut self, cursor: Cursor) {
        match cursor {
            Cursor::On => self.display_ctrl |= Cursor::On as u8,
            Cursor::Off => self.display_ctrl &= !(Cursor::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        delay!(CMD_DELAY);
    }

    /// Make the background of the cursor blink or stop blinking.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
    ///
    /// lcd.set_blink(Blink::On);
    /// ```
    pub fn set_blink(&mut self, blink: Blink) {
        match blink {
            Blink::On => self.display_ctrl |= Blink::On as u8,
            Blink::Off => self.display_ctrl &= !(Blink::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        delay!(CMD_DELAY);
    }

    /// Turn auto scroll on or off.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
    ///
    /// lcd.set_autoscroll(AutoScroll::On);
    /// ```
    pub fn set_autoscroll(&mut self, scroll: AutoScroll) {
        match scroll {
            AutoScroll::On => self.display_mode |= AutoScroll::On as u8,
            AutoScroll::Off => self.display_mode &= !(AutoScroll::On as u8),
        }
        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        delay!(CMD_DELAY);
    }

    /// Add a new character map to the LCD memory (CGRAM) at a particular location.
    /// There are eight locations available at positions 0-7, and location values
    /// outside of this range will be bitwise masked to fall within the range, possibly
    /// overwriting an existing custom character.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
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
    /// lcd.write(0u8);
    /// ```
    pub fn set_character(&mut self, mut location: u8, map: [u8;8]) {
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
    /// let mut lcd: LcdDisplay = ...;
    /// lcd.clear();
    /// ```
    pub fn clear(&mut self) {
        self.command(Command::ClearDisplay as u8);
        delay!(CMD_DELAY);
    }

    /// Move the cursor to the home position.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
    /// lcd.home(); // cursor should be top-left
    /// ```
    pub fn home(&mut self) {
        self.command(Command::ReturnHome as u8);
        delay!(CMD_DELAY);
    }

    /// Scroll the display to the right. (See [set_scroll][LcdDisplay::set_scroll])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
    /// lcd.blink_off();
    /// ```
    pub fn blink_off(&mut self) {
        self.set_blink(Blink::Off);
    }

    /// Turn autoscroll on. (See [set_autoscroll][LcdDisplay::set_autoscroll])
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
    /// let lines = lcd.lines();
    /// ```
    pub fn lines(&self) -> Lines {
        if (self.display_func & Lines::TwoLines as u8) == 0 {
            Lines::OneLine
        } else {
            Lines::TwoLines
        }
    }

    /// Print a message to the LCD display.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut lcd: LcdDisplay = ...;
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
    /// let mut lcd: LcdDisplay = ...;
    /// lcd.write('A' as u8);
    /// ```
    pub fn write(&mut self, value: u8) {
        delay!(CHR_DELAY);
        self.send(value, 1);
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
        self.send(value, 0);
    }

    /// Send bytes to the LCD display with the RS pin set either high (for commands)
    /// or low (to write to memory)
    ///
    /// # Examples
    ///
    /// ```
    /// self.send(value, 1);
    /// ```
    fn send(&mut self, byte: u8, mode: u8) {
        set!(self.rs, mode);
        set!(self.rw,0);
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
        set!(self.en, 0);
        match self.mode() {
            Mode::FourBits => {
                set!(self.d4, byte, 0);
                set!(self.d3, byte, 1);
                set!(self.d2, byte, 2);
                set!(self.d1, byte, 3);
            }
            Mode::EightBits => {
                set!(self.d4, byte, 0);
                set!(self.d3, byte, 1);
                set!(self.d2, byte, 2);
                set!(self.d1, byte, 3);
                set!(self.d8, byte, 4);
                set!(self.d7, byte, 5);
                set!(self.d6, byte, 6);
                set!(self.d5, byte, 7);
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
        set!(self.en, 1);
        set!(self.en, 0);
    }
}