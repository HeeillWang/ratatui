use termion;
use rustbox;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Magenta,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightMagenta,
    LightCyan,
    White,
    Rgb(u8, u8, u8),
}

macro_rules! termion_fg {
    ($color:ident) => (format!("{}", termion::color::Fg(termion::color::$color)));
}

macro_rules! termion_fg_rgb {
    ($r:expr, $g:expr, $b:expr) => (format!("{}", termion::color::Fg(termion::color::Rgb($r, $g, $b))));
}

macro_rules! termion_bg {
    ($color:ident) => (format!("{}", termion::color::Bg(termion::color::$color)));
}

macro_rules! termion_bg_rgb {
    ($r:expr, $g:expr, $b:expr) => (format!("{}", termion::color::Bg(termion::color::Rgb($r, $g, $b))));
}

impl Color {
    pub fn termion_fg(&self) -> String {
        match *self {
            Color::Reset => termion_fg!(Reset),
            Color::Black => termion_fg!(Black),
            Color::Red => termion_fg!(Red),
            Color::Green => termion_fg!(Green),
            Color::Yellow => termion_fg!(Yellow),
            Color::Magenta => termion_fg!(Magenta),
            Color::Cyan => termion_fg!(Cyan),
            Color::Gray => termion_fg_rgb!(146, 131, 116),
            Color::DarkGray => termion_fg_rgb!(80, 73, 69),
            Color::LightRed => termion_fg!(LightRed),
            Color::LightGreen => termion_fg!(LightGreen),
            Color::LightYellow => termion_fg!(LightYellow),
            Color::LightMagenta => termion_fg!(LightMagenta),
            Color::LightCyan => termion_fg!(LightCyan),
            Color::White => termion_fg!(White),
            Color::Rgb(r, g, b) => termion_fg_rgb!(r, g, b),
        }
    }
    pub fn termion_bg(&self) -> String {
        match *self {
            Color::Reset => termion_bg!(Reset),
            Color::Black => termion_bg!(Black),
            Color::Red => termion_bg!(Red),
            Color::Green => termion_bg!(Green),
            Color::Yellow => termion_bg!(Yellow),
            Color::Magenta => termion_bg!(Magenta),
            Color::Cyan => termion_bg!(Cyan),
            Color::Gray => termion_bg_rgb!(146, 131, 116),
            Color::DarkGray => termion_bg_rgb!(80, 73, 69),
            Color::LightRed => termion_bg!(LightRed),
            Color::LightGreen => termion_bg!(LightGreen),
            Color::LightYellow => termion_bg!(LightYellow),
            Color::LightMagenta => termion_bg!(LightMagenta),
            Color::LightCyan => termion_bg!(LightCyan),
            Color::White => termion_bg!(White),
            Color::Rgb(r, g, b) => termion_bg_rgb!(r, g, b),
        }
    }
}

impl Into<rustbox::Color> for Color {
    fn into(self) -> rustbox::Color {
        match self {
            Color::Reset => rustbox::Color::Default,
            Color::Black => rustbox::Color::Black,
            Color::Red => rustbox::Color::Red,
            Color::Green => rustbox::Color::Green,
            Color::Yellow => rustbox::Color::Yellow,
            Color::Magenta => rustbox::Color::Magenta,
            Color::Cyan => rustbox::Color::Cyan,
            Color::Gray => rustbox::Color::Black,
            Color::DarkGray => rustbox::Color::Black,
            Color::LightRed => rustbox::Color::Red,
            Color::LightGreen => rustbox::Color::Green,
            Color::LightYellow => rustbox::Color::Yellow,
            Color::LightMagenta => rustbox::Color::Magenta,
            Color::LightCyan => rustbox::Color::Cyan,
            Color::White => rustbox::Color::White,
            Color::Rgb(r, g, b) => rustbox::Color::Default,
        }
    }
}
