use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Colour {
    White = 0,
    Orange = 1,
    Magenta = 2,
    LightBlue = 3,
    Yellow = 4,
    Lime = 5,
    Pink = 6,
    Gray = 7,
    LightGray = 8,
    Cyan = 9,
    Purple = 10,
    Blue = 11,
    Brown = 12,
    Green = 13,
    Red = 14,
    Black = 15,
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Colour::White => "white",
            Colour::Orange => "orange",
            Colour::Magenta => "magenta",
            Colour::LightBlue => "light_blue",
            Colour::Yellow => "yellow",
            Colour::Lime => "lime",
            Colour::Pink => "pink",
            Colour::Gray => "gray",
            Colour::LightGray => "light_gray",
            Colour::Cyan => "cyan",
            Colour::Purple => "purple",
            Colour::Blue => "blue",
            Colour::Brown => "brown",
            Colour::Green => "green",
            Colour::Red => "red",
            Colour::Black => "black",
        })
    }
}

impl From<i32> for Colour {
    fn from(colour_number: i32) -> Self {
        match colour_number {
            0 => Colour::White,
            1 => Colour::Orange,
            2 => Colour::Magenta,
            3 => Colour::LightBlue,
            4 => Colour::Yellow,
            5 => Colour::Lime,
            6 => Colour::Pink,
            7 => Colour::Gray,
            8 => Colour::LightGray,
            9 => Colour::Cyan,
            10 => Colour::Purple,
            11 => Colour::Blue,
            12 => Colour::Brown,
            13 => Colour::Green,
            14 => Colour::Red,
            15 => Colour::Black,
            _ => panic!("Invalid colour number: {}", colour_number),
        }
    }
}

impl From<Colour> for u8 {
    fn from(colour: Colour) -> u8 {
        match colour {
            Colour::White => 0,
            Colour::Orange => 1,
            Colour::Magenta => 2,
            Colour::LightBlue => 3,
            Colour::Yellow => 4,
            Colour::Lime => 5,
            Colour::Pink => 6,
            Colour::Gray => 7,
            Colour::LightGray => 8,
            Colour::Cyan => 9,
            Colour::Purple => 10,
            Colour::Blue => 11,
            Colour::Brown => 12,
            Colour::Green => 13,
            Colour::Red => 14,
            Colour::Black => 15,
        }
    }
}

impl From<&str> for Colour {
    fn from(colour_string: &str) -> Self {
        match colour_string {
            "white" => Colour::White,
            "orange" => Colour::Orange,
            "magenta" => Colour::Magenta,
            "light_blue" => Colour::LightBlue,
            "yellow" => Colour::Yellow,
            "lime" => Colour::Lime,
            "pink" => Colour::Pink,
            "gray" => Colour::Gray,
            "light_gray" => Colour::LightGray,
            "cyan" => Colour::Cyan,
            "purple" => Colour::Purple,
            "blue" => Colour::Blue,
            "brown" => Colour::Brown,
            "green" => Colour::Green,
            "red" => Colour::Red,
            "black" => Colour::Black,
            _ => Colour::Black,
        }
    }
}
pub type Color = Colour;
