#![no_std]

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

#[repr(u8)]
pub enum Layout {
    RightToLeft = 0x00, // LCD_ENTRYRIGHT
    LeftToRight = 0x02, // LCD_ENTRYLEFT
}

#[repr(u8)]
pub enum AutoScroll {
    On = 0x01,  // LCD_ENTRYSHIFTINCREMENT
    Off = 0x00, // LCD_ENTRYSHIFTDECREMENT
}

#[repr(u8)]
pub enum Display {
    On = 0x04,  // LCD_DISPLAYON
    Off = 0x00, // LCD_DISPLAYOFF
}

#[repr(u8)]
pub enum Cursor {
    On = 0x02,  // LCD_CURSORON
    Off = 0x00, // LCD_CURSOROFF
}

#[repr(u8)]
pub enum Blink {
    On = 0x01,  // LCD_BLINKON
    Off = 0x00, // LCD_BLINKOFF
}

#[repr(u8)]
pub enum Scroll {
    Right = 0x04, // LCD_MOVERIGHT
    Left = 0x00,  // LCD_MOVELEFT
}

#[repr(u8)]
pub enum Mode {
    EightBits = 0x10, // LCD_8BITMODE
    FourBits = 0x00,  // LCD_4BITMODE
}

#[repr(u8)]
pub enum Lines {
    TwoLines = 0x08, // LCD_2LINE
    OneLine = 0x00,  // LCD_1LINE
}

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

    pub fn with_rw<M>(mut self, rw: InputPin<M, Rw>) -> Self 
    where 
        M: InputMode,
    {
        self.rw = Some(rw.into_output());
        self
    }

    pub fn with_size(mut self, value: Size) -> Self {
        match value {
            Size::Dots5x10 => self.display_func |= Size::Dots5x10 as u8,
            Size::Dots5x8 => self.display_func &= !(Size::Dots5x10 as u8),
        }
        self
    }

    pub fn with_lines(mut self, value: Lines) -> Self {
        match value {
            Lines::TwoLines => self.display_func |= Lines::TwoLines as u8,
            Lines::OneLine => self.display_func &= !(Lines::TwoLines as u8),
        }
        self
    }

    pub fn with_layout(mut self, value: Layout) -> Self {
        match value {
            Layout::LeftToRight => self.display_mode |= Layout::LeftToRight as u8,
            Layout::RightToLeft => self.display_mode &= !(Layout::LeftToRight as u8),
        }
        self
    }

    pub fn with_display(mut self, value: Display) -> Self {
        match value {
            Display::On => self.display_ctrl |= Display::On as u8,
            Display::Off => self.display_ctrl &= !(Display::On as u8),
        }
        self
    }

    pub fn with_cursor(mut self, value: Cursor) -> Self {
        match value {
            Cursor::On => self.display_ctrl |= Cursor::On as u8,
            Cursor::Off => self.display_ctrl &= !(Cursor::On as u8),
        }
        self
    }

    pub fn with_blink(mut self, value: Blink) -> Self {
        match value {
            Blink::On => self.display_ctrl |= Blink::On as u8,
            Blink::Off => self.display_ctrl &= !(Blink::On as u8),
        }
        self
    }

    pub fn with_autoscroll(mut self, value: AutoScroll) -> Self {
        match value {
            AutoScroll::On => self.display_mode |= AutoScroll::On as u8,
            AutoScroll::Off => self.display_mode &= !(AutoScroll::On as u8),
        }
        self
    }

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

    // ========================================================================
    // GENERAL ACTIONS
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

    pub fn set_scroll(&mut self, direction: Scroll, distance: u8) {
        let command = Command::CursorShift as u8 | Move::Display as u8 | direction as u8;
        for _ in 0..distance {
            self.command(command);
            delay!(CMD_DELAY);
        }
    }

    pub fn set_layout(&mut self, layout: Layout) {
        match layout {
            Layout::LeftToRight => self.display_mode |= Layout::LeftToRight as u8,
            Layout::RightToLeft => self.display_mode &= !(Layout::LeftToRight as u8),
        }
        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        delay!(CMD_DELAY);
    }

    pub fn set_display(&mut self, display: Display) {
        match display {
            Display::On => self.display_ctrl |= Display::On as u8,
            Display::Off => self.display_ctrl &= !(Display::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        delay!(CMD_DELAY);
    }

    pub fn set_cursor(&mut self, cursor: Cursor) {
        match cursor {
            Cursor::On => self.display_ctrl |= Cursor::On as u8,
            Cursor::Off => self.display_ctrl &= !(Cursor::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        delay!(CMD_DELAY);
    }

    pub fn set_blink(&mut self, blink: Blink) {
        match blink {
            Blink::On => self.display_ctrl |= Blink::On as u8,
            Blink::Off => self.display_ctrl &= !(Blink::On as u8),
        }
        self.command(Command::SetDisplayCtrl as u8 | self.display_ctrl);
        delay!(CMD_DELAY);
    }

    pub fn set_autoscroll(&mut self, scroll: AutoScroll) {
        match scroll {
            AutoScroll::On => self.display_mode |= AutoScroll::On as u8,
            AutoScroll::Off => self.display_mode &= !(AutoScroll::On as u8),
        }
        self.command(Command::SetDisplayMode as u8 | self.display_mode);
        delay!(CMD_DELAY);
    }

    pub fn set_character(&mut self, mut location: u8, map: [u8;8]) {
        location &= 0x7; // limit to locations 0-7
        self.command(Command::SetCGramAddr as u8 | (location << 3));
        for ch in map.iter() {
            delay!(CHR_DELAY);
            self.write(*ch);
        }
    }

    // ========================================================================
    // SPECIFIC ACTIONS
    pub fn clear(&mut self) {
        self.command(Command::ClearDisplay as u8);
        delay!(CMD_DELAY);
    }

    pub fn home(&mut self) {
        self.command(Command::ReturnHome as u8);
        delay!(CMD_DELAY);
    }

    pub fn scroll_right(&mut self, value: u8) {
        self.set_scroll(Scroll::Right, value);
    }

    pub fn scroll_left(&mut self, value: u8) {
        self.set_scroll(Scroll::Left, value);
    }

    pub fn layout_left_to_right(&mut self) {
        self.set_layout(Layout::LeftToRight);
    }

    pub fn layout_right_to_left(&mut self) {
        self.set_layout(Layout::RightToLeft);
    }

    pub fn display_on(&mut self) {
        self.set_display(Display::On);
    }

    pub fn display_off(&mut self) {
        self.set_display(Display::Off);
    }

    pub fn cursor_on(&mut self) {
        self.set_cursor(Cursor::On);
    }

    pub fn cursor_off(&mut self) {
        self.set_cursor(Cursor::Off);
    }

    pub fn blink_on(&mut self) {
        self.set_blink(Blink::On);
    }

    pub fn blink_off(&mut self) {
        self.set_blink(Blink::Off);
    }

    pub fn autoscroll_on(&mut self) {
        self.set_autoscroll(AutoScroll::On);
    }

    pub fn autoscroll_off(&mut self) {
        self.set_autoscroll(AutoScroll::Off);
    }

    // ========================================================================
    // CURRENT STATE
    pub fn mode(&self) -> Mode {
        if (self.display_func & Mode::EightBits as u8) == 0 {
            Mode::FourBits
        } else {
            Mode::EightBits
        }
    }

    pub fn layout(&self) -> Layout {
        if (self.display_mode & Layout::LeftToRight as u8) == 0 {
            Layout::RightToLeft
        } else {
            Layout::LeftToRight
        }
    }

    pub fn display(&self) -> Display {
        if (self.display_ctrl & Display::On as u8) == 0 {
            Display::Off
        } else {
            Display::On
        }
    }

    pub fn cursor(&self) -> Cursor {
        if (self.display_ctrl & Cursor::On as u8) == 0 {
            Cursor::Off
        } else {
            Cursor::On
        }
    }

    pub fn blink(&self) -> Blink {
        if (self.display_ctrl & Blink::On as u8) == 0 {
            Blink::Off
        } else {
            Blink::On
        }
    }

    pub fn autoscroll(&self) -> AutoScroll {
        if (self.display_mode & AutoScroll::On as u8) == 0 {
            AutoScroll::Off
        } else {
            AutoScroll::On
        }
    }

    pub fn lines(&self) -> Lines {
        if (self.display_func & Lines::TwoLines as u8) == 0 {
            Lines::OneLine
        } else {
            Lines::TwoLines
        }
    }

    // ========================================================================
    // PRINT
    pub fn print(&mut self, text: &str) {
        for ch in text.chars() {
            delay!(CHR_DELAY);
            self.write(ch as u8);
        }
    }

    pub fn write(&mut self, value: u8) {
        self.send(value, 1);
    }

    // ========================================================================
    // PRIVATE
    fn command(&mut self, value: u8) {
        self.send(value, 0);
    }

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

    fn pulse(&mut self) {
        set!(self.en, 1);
        set!(self.en, 0);
    }
}